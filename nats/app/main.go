package main

import (
	"context"
	"os"
	"os/signal"
	"strings"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/pkg/errors"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"golang.org/x/sync/errgroup"
)

func serve(ctx context.Context) error {
    // NATS Connect
	k, err := nats.Connect(
        strings.Join([]string{
            // Front line servers in the cluster. They will tell the client about other ones.
            "nats://app:verysecret@127.0.0.1:4222/APP",  // "/APP" is the account.
            // Alternatively, you can use a token (nats://token@127.0.0.1/)
            // or an NKEY (ed25519 key file) or an OAuth JWT token
        }, ","),
        // Connection name (for monitoring)
        nats.Name("api-server"),
        // Don't deliver my pub messages to me
        // NOTE: turned off because this is exactly what we do here: send messages to ourselves :)
        // nats.NoEcho(),
        // Ping server every <duration>
        nats.PingInterval(10 * time.Second),
        // Timeout for draining a connection
        nats.DrainTimeout(10*time.Second),
        // Reconnect: default wait=2s, timeout=2s
        // Ping interval: 2 minutes (heartbeat)
        // Log connection events
        nats.DisconnectErrHandler(func(_ *nats.Conn, err error) {
            if err != nil {
                log.Error().Err(err).Msg("NATS Disconnected")
            } else {
                log.Info().Msg("NATS Disconnected")
            }
        }),
        nats.ReconnectHandler(func(_ *nats.Conn) {
            log.Info().Msg("NATS Reconnected")
        }),
        nats.ClosedHandler(func(_ *nats.Conn) {
            log.Info().Msg("NATS client closed")
        }),
        nats.DiscoveredServersHandler(func(nc *nats.Conn) {
            log.Info().Strs("known", nc.Servers()).Strs("discovered", nc.DiscoveredServers()).Msg("NATS new servers discovered")
        }),
        nats.ErrorHandler(func(_ *nats.Conn, _ *nats.Subscription, err error) {
            // E.g. slow consumer
            // log, or maybe send to an error channel
            log.Error().Err(err).Msg("NATS Error")
        }),
    )
	if err != nil {
		return errors.Wrap(err, "failed to connect to NATS")
	}
    log.Info().
        Bool("connected", k.IsConnected()).
        Int64("max_payload", k.MaxPayload()).  // 1Mb default
        Str("addr", k.ConnectedAddr()).
        Msg("NATS Connected")

    // Drain the connection, which will close it when done.
    // It lets all handlers finish: unsubscribe, process all cached/inflight messages, clean-up.
    // Drain() can be used instead of Unsubscribe()
    // Do this before quitting.
	// defer k.Close()
    defer k.Drain()



	// Services: producer, consumer
    // errgroup handles panics
    g, ctx := errgroup.WithContext(ctx)
    g.Go(func() error { return subscriptionServe(ctx, k) })
    g.Go(func() error { return publisherServe(ctx, k) })
    g.Go(func() error { return microserviceServe(ctx, k) })
    g.Go(func() error { return serveJetstream(ctx, k) })
    g.Go(func() error { return serveKvStorage(ctx, k) })
    g.Go(func() error { return serveObjectStorage(ctx, k) })

    // Wait
    if err := g.Wait(); err != nil {
        return errors.Wrap(err, "Wait() failed")
    }

    // Quit
    return nil

}

func main(){
    ctx := context.Background()
    ctx, cancel := signal.NotifyContext(ctx, os.Kill, os.Interrupt)
    defer cancel()

    if err := serve(ctx); err != nil {
        log.Fatal().Err(err).Msg("Panic")
    }
}

func init(){
    log.Logger = zerolog.New(zerolog.NewConsoleWriter())
}
