package main

import (
	"context"
	"fmt"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/pkg/errors"
)

func producerServe(ctx context.Context, k *nats.Conn) error {
	// Give the consumer some time :)
	time.Sleep(100 * time.Millisecond)



	// Publish one message
	if err := k.Publish("updates", []byte("All is Well")); err != nil {
		return errors.Wrap(err, "Publish() failed")
	}



	// Publish, expect a response (using "reply-subject" or "inbox")
	resp, err := k.Request("request.hello", []byte("Hi there"), 1 * time.Second)
	if err != nil {
		return errors.Wrap(err, "Request() failed")
	}
	fmt.Printf("response: %+v\n", resp)//nocommit



	// Send job (to a work queue, but we don't care)
	if err := k.Publish("jobs.task", []byte("do this")); err != nil {
		return errors.Wrap(err, "Publish() failed")
	}


	// Send a PING/PONG to the server to make sure all publishes are written
	if err := k.FlushTimeout(time.Second); err != nil {
		return errors.Wrap(err, "FlushTimeout() failed")
	}


	// Wait
	<-ctx.Done()
	return nil
}
