package main

import (
	"context"
	"os"
	"os/signal"

	"github.com/cockroachdb/errors"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/twmb/franz-go/pkg/kadm"
	"github.com/twmb/franz-go/pkg/kerr"
	"github.com/twmb/franz-go/pkg/kgo"
	"github.com/twmb/franz-go/pkg/sasl/scram"
	"golang.org/x/sync/errgroup"
)


var KAFKA_URLS = [...]string{
	"localhost:19092",
}

func serve(ctx context.Context) error {
	// New Client
	k, err := kgo.NewClient(
		// Log internal Kafka events
		// kgo.WithLogger(kzerolog.New(&log.Logger)),
		// Connect to
		kgo.SeedBrokers(KAFKA_URLS[:]...),
		kgo.SASL(
			scram.Sha256(func(ctx context.Context) (scram.Auth, error) {
				return scram.Auth{
					User: "superuser",
					Pass: "secretpassword",
				}, nil
			}),
		),
		// Consumers: group, topics, commit behavior
		kgo.ConsumerGroup("my-group-identifier"),
		kgo.ConsumeTopics("messages", "cars"),  // must exist; however, the library will retry consuming (but fail producing)
		kgo.BlockRebalanceOnPoll(),
		kgo.AutoCommitMarks(),  // mark messages as they are processed
		// Producers
		kgo.ProducerLinger(0),  // send immediately (optimize for low latency)
		kgo.RequiredAcks(kgo.LeaderAck()),  // one ack is sufficient (lower latency)
	)
	if err != nil {
		return errors.Wrap(err, "failed to init Kafka client")
	}
	defer k.CloseAllowingRebalance()  // AllowRebalance() + Close()
	// defer k.Close()

	// Create topics
	{
		adm := kadm.NewClient(k)

		// List existing topics
		topics, err := adm.ListTopics(ctx, "messages", "cars")
		if err != nil {
			return errors.Wrap(err, "failed to get topic info")
		}

		// Results
		for _, topic := range topics {
			// Create if not exists
			if errors.Is(topic.Err, kerr.UnknownTopicOrPartition) {
				_, err = adm.CreateTopic(ctx, 1, -1, nil, "cars")  // can do it only once
				if err != nil {
					return errors.Wrap(err, "failed to create topic")
				}
				log.Info().Str("topic", topic.Topic).Msg("Created topic")
			}

		}
	}

	// Schema registry
	err = schemaRegistry(ctx, k)
	if err != nil {
		return errors.Wrap(err, "schema failed")
	}

	// Services: producer, consumer
	// errgroup handles panics
	g, ctx := errgroup.WithContext(ctx)
	g.Go(func() error { return consumerServe(ctx, k) })
	g.Go(func() error { return producerServe(ctx, k) })

	if err := g.Wait(); err != nil {
		log.Error().Err(err).Msg("Quitting")
	}

	// Wait
	return nil
}

func main(){
	ctx := context.Background()
	ctx, cancel := signal.NotifyContext(ctx, os.Kill, os.Interrupt)
	defer cancel()

	if err := serve(ctx); err != nil {
		log.Panic().Err(err).Msg("Panic")
	}
}

func init(){
	log.Logger = zerolog.New(zerolog.NewConsoleWriter())
}