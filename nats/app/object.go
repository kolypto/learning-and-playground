package main

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
	"github.com/pkg/errors"
)

// Object storage example
func serveObjectStorage(ctx context.Context, k *nats.Conn) error {
	// ObjectStore is implemented using JetStreams.
	js, err := jetstream.New(k)
	if err != nil {
		return errors.Wrap(err, "jetstream.New() failed")
	}

	// Create/Update Object store
	// Use `ObjectStore()` to lookup an existing store.
	os, err := js.CreateOrUpdateObjectStore(ctx, jetstream.ObjectStoreConfig{
		// Independent Object Store
		Bucket: "avatars",
		Description: `User avatars`,
		// TTL
		// Default: no expire
		TTL: 365 * 24 * time.Hour,
		// Max file size
		MaxBytes: 100 * MB,
		// Storage: file | memory
		Storage: jetstream.MemoryStorage,
	})
	if err != nil {
		return errors.Wrap(err, "CreateOrUpdateObjectStore() failed")
	}

	// Put: create/overwrite
	buf := bytes.NewBufferString("contents")
	savedObj, err := os.Put(ctx, jetstream.ObjectMeta{
		Name: "filename.txt",
		Description: `Whatever description`,
	}, buf)
	if err != nil {
		return errors.Wrap(err, "Put() failed")
	}
	fmt.Println("\tChunks:", savedObj.Chunks)
	fmt.Println("\tDigest:", savedObj.Digest)
	fmt.Println("\tNUID:", savedObj.NUID)


	// Get the file as I/O
	obj, err := os.Get(ctx, "filename.txt")
	if err != nil {
		return errors.Wrap(err, "Get() failed")
	}
	contents, _ := io.ReadAll(obj)
	fmt.Println("Contents:", string(contents))


	// Watch for any updates
	watcher, err := os.Watch(ctx)
	for update := range watcher.Updates() {
		fmt.Println("Update:", update)

		// Just once
		break
	}

	// Done
	return nil
}

const (
    _        = iota
    KB int64 = 1 << (10 * iota)
    MB
    GB
    // ... other units
)
