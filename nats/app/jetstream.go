package main

import (
	"context"
	"fmt"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
	"github.com/pkg/errors"
)

func serveJetstream(ctx context.Context, k *nats.Conn) error {
	js, err := jetstream.New(k)
	if err != nil {
		return errors.Wrap(err, "JetStream() failed")
	}

	// A stream: a process within NATS that listens to subjects and stores everything it sees.
	// CreateStream() is idempotent! But use UpdateStream() to migrate
	stream, err := js.CreateOrUpdateStream(ctx, jetstream.StreamConfig{
		// NOTE: stream names
		// Stream name is also a subject that you can subscribe to!
		// This is why in NATS you'd use UPPERCASE on stream names and consumers.
		Name:     "orders",
		Description: `Order statuses: new, processed`,
		// Subjects that are consumed by the stream.
		// Wildcards are supported
		Subjects: []string{
			"orders.*",  // -> orders.new, orders.processed
		},
		// Store where? File | Memory
		Storage: jetstream.MemoryStorage,
		// Retention Policy:
		// * `limits`: limit N messages, storage size, message age: MaxAge, MaxMsgs, MaxBytes, MaxMsgsPerSubject
		// * `work`: only keep messages until they're consumed, then delete (NOTE: in this mode consumer filters shouldn't overlap!)
		// * `interest`: keep messages until *all* consumers have ACKed it. Messages are removed if no one is listening.
		Retention: jetstream.LimitsPolicy,
		DenyDelete: true,
		DenyPurge: false,
		AllowRollup: true, // requires purge permission
	})
	if err != nil {
		return errors.Wrap(err, "AddStream() failed")
	}

	// Stream: get one last msg for subject
	{
	msg, err := stream.GetLastMsgForSubject(ctx, "orders.new")
	if err != nil {
		return errors.Wrap(err, "GetLastMsgForSubject() failed")
	}
	fmt.Printf("stream last msg: %+v\n", msg)
	}


	// Consumer: a view into the stream with their own cursor.
	// Idempotent! But use UpdateConsumer() to migrate
	consumer, err := js.CreateOrUpdateConsumer(ctx, "orders", jetstream.ConsumerConfig{
		// Create a durable consumer: because it has a name.
		// Consumer with no name: an ephemeral consumer with an "InactiveThreshold" removal timeout
		//
		// Durable consumers remember where they are. They can be used for load-balancing.
		// Ephemeral consumer does not persist progress. It will get deleted when no one is connected (after a timeout)
		Durable: "new",
		// Description.
		// Especially useful for Ephemeral consumers because they have no name.
		Description: "New orders",
		// Filter messages by subjects
		// NOTE: work queues do not allow subjects to overlap! But regular streams ("limits") and interest streams can.
		FilterSubjects: []string{
			"orders.new",
		},
		// First launch: deliver all messages from the beginning? Only new ones?
		// Replay one last message? Replay one last message per subject?
		DeliverPolicy: jetstream.DeliverAllPolicy,
		// How to ack messages? None|All|Explicit. Don't; ack all messages (batch); ack explicitly every single one
		AckPolicy: jetstream.AckExplicitPolicy,  // ACK every message
		AckWait: 30 * time.Second,
		// Max number of times a message will be redelivered (when nack'ed)
		MaxDeliver: 3,
		// ACK wait time (?)
		// AckWait: 30 * time.Second,
		// MaxWaiting: 512,
		// MaxAckPending: ,
	})
	if err != nil {
		return errors.Wrap(err, "AddConsumer() failed")
	}



	// Publish some messages for ourselves.
	// They will get stored in the JetStream.
	for i := range 5 {
		// Each PublishAsync() returns a promise.
		// But we'll watch them all together using PublishAsyncComplete()
		_, err := js.PublishAsync("orders.new", []byte(fmt.Sprintf("order #%d", i)))
		if err != nil {
			return errors.Wrap(err, "PublishAsync() failed")
		}
	}
	// Wait for all Publish()es simultaneously:
	// PublishAsyncComplete() returns a channel that gets closed when all outstanding requests are ACKed
	select {
	case <-js.PublishAsyncComplete():
	case <-time.After(5 * time.Second):
		fmt.Println("Did not resolve in time")
	}
	fmt.Println("Published")//nocommit




	// Read messages one by one
	{
	msg, err := consumer.Next()
	if err != nil {
		return errors.Wrap(err, "Next() failed")
	}
	fmt.Printf("stream Next(): %+v\n", string(msg.Data()))
	msg.Nak()
	}


	// Read them manually in batches:
	// - Fetch(): receive a batch
	// - Messages(): iterate over messages



	// Start consuming messages: pull consumer.
	// Unlike the legacy Subscribe(), it won't create a consumer but instead use the existing one.
	consumeContext, err := consumer.Consume(func(msg jetstream.Msg){
		fmt.Printf("stream msg: %s %+v\n", msg.Subject(), string(msg.Data()))
		msg.Ack()
	})
	if err != nil {
		return errors.Wrap(err, "Consume() failed")
	}
	defer consumeContext.Stop()

	// Keep working
	<-ctx.Done()


	// Done
	return nil
}