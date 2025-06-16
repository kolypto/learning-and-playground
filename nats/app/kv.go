package main

import (
	"context"
	"fmt"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
	"github.com/pkg/errors"
)

// K/V storage example
func serveKvStorage(ctx context.Context, k *nats.Conn) error {
	// KV is implemented using JetStreams.
	js, err := jetstream.New(k)
	if err != nil {
		return errors.Wrap(err, "jetstream.New() failed")
	}

	// Create/Update KV store
	// Use `KeyValue()` to lookup an existing store.
	kv, err := js.CreateOrUpdateKeyValue(ctx, jetstream.KeyValueConfig{
		// Bucket name.
		// A "bucket" is an independent K/V store.
		Bucket: "bucket-name",
		Description: `Test bucket`,
		// The number of historical values to keep per key.
		// Default: 1. Max: 64
		History: 1,
		// Key expiration time.
		// Default: no expiration.
		TTL: 10 * time.Minute,
		// Storage: file | memory
		Storage: jetstream.MemoryStorage,
		// RePublish: publish new values to a subject
		RePublish: &jetstream.RePublish{
			Source: "subject.pattern.>",  // from
			Destination: "subject.pattern.>",  // to
			HeadersOnly: false,
		},
	})
	if err != nil {
		return errors.Wrap(err, "KeyValue() failed")
	}


	// Watch a key, or many keys
	w, err := kv.Watch(ctx, "key-name")
	if err != nil {
		return errors.Wrap(err, "kv.Watch() failed")
	}
	defer w.Stop()
	go func(){
		for {
			select {
			// Will return updates to the key as they come.
			// Will first report the current value, then `nil` as a separator.
			// Opt out with: UpdatesOnly
			case v, ok := <-w.Updates():
				if ok {
					if v == nil {
						fmt.Printf("kv value update: %+v\n", v)
					} else {
						fmt.Printf("kv value update: %+v\n", string(v.Value()))
					}
				}
			case <-ctx.Done():
				return
			}
		}
	}()

	// Store a value
	revision, err := kv.Put(ctx, "key-name", []byte("value1"))
	if err != nil {
		return errors.Wrap(err, "kv.Put() failed")
	}

	// Get a value
	v, err := kv.Get(ctx, "key-name")
	if err != nil {
		return errors.Wrap(err, "kv.Get() failed")
	}
	fmt.Printf("kv value get: %+v\n", string(v.Value()))

	// Get a specific revision
	kv.GetRevision(ctx, "key-name", revision)



	return nil
}