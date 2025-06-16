package main

import (
	"context"
	"fmt"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/pkg/errors"
)

func subscriptionServe(ctx context.Context, k *nats.Conn) error {
	// Async subscription: will invoke the callback
	// It starts 1 goroutine that will call your callback. Running more goroutines is up to you.
	// Message delivery: serial, one at a time.
	async_sub, err := k.Subscribe("updates", func(msg *nats.Msg) {
		fmt.Printf("async msg: %+v\n", msg)
		// if err := msg.Ack(); err != nil {
		// 	log.Error().Err(err).Msg("Ack() failed")
		// }
	})
	if err != nil {
		return errors.Wrap(err, "Subscribe() failed")
	}
	defer async_sub.Unsubscribe()



	// Async chan subscription: will push to a channel
	// It will not start a goroutine: instead, it will push messages to the channel
	updates_chan := make(chan *nats.Msg, 3)
	chan_sub, err := k.ChanSubscribe("updates", updates_chan)
	if err != nil {
		return errors.Wrap(err, "Subscribe() failed")
	}
	defer chan_sub.Unsubscribe()





	// Wait for a message: "Sync Subscribe"
	// Under the hood, it reads from a channel.
	one_sub, err := k.SubscribeSync("request.*")
	if err != nil {
		return errors.Wrap(err, "SubscribeSync() failed")
	}
	// Auto-unsubscribe after N messages.
	// Use case: when only 1 messages is expected
	if err := one_sub.AutoUnsubscribe(1); err != nil {
		return errors.Wrap(err, "AutoUnsubscribe() failed")
	}
	msg, err := one_sub.NextMsg(1 * time.Second)
	if err != nil {
		return errors.Wrap(err, "NextMsg(2) failed")
	}
	// Reply: Incoming messages may have a "reply-to" field: a subject where a reply is expected.
	err = msg.Respond([]byte("nice to see you!"))
	if err != nil {
		return errors.Wrap(err, "Respond() failed")
	}






	// Queue Subscriptions
	// Provide a queue name: the server will load-balance between all members of the queue group.
	// Queue groups in NATS are dynamic and do not require any server configuration.
	// NOTE: it's still not JetStream: just load-balancing in real time!
	queue_sub, err := k.QueueSubscribe("jobs.>", "job_workers", func(msg *nats.Msg) {
		fmt.Printf("job msg: %+v\n", msg)
	})
	if err != nil {
		return errors.Wrap(err, "QueueSubscribe() failed")
	}
	defer queue_sub.Unsubscribe()


	<-ctx.Done()
	return nil
}
