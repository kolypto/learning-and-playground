package main

import (
	"context"
	"time"

	"github.com/cockroachdb/errors"
	"github.com/twmb/franz-go/pkg/kgo"
)

func producerServe(ctx context.Context, k *kgo.Client) error {
	for {
		// Produce bytes
		record := &kgo.Record{
			Topic: "messages",
			Key: []byte("key"),
			Value: []byte(time.Now().String()),
			Headers: []kgo.RecordHeader{
				{Key: "Content-Type", Value: []byte("text/plain")}, // e.g.
			},
		}
		_, err := k.ProduceSync(ctx, record).First()
		if err != nil {
			// May be: context, client closing, ...
			return errors.Wrap(err, "produce failed")
		}

		// Produce with Schema
		_, err = k.ProduceSync(ctx, &kgo.Record{
			Topic: "cars",
			Value: carSchemaSD.MustEncode(Car{
				Make: "Toyota",
				Model: "Camry",
				Year: 2017,
				Engine: 180,
			}),
		}).First()
		if err != nil {
			return errors.Wrap(err, "produce failed")
		}

		// Sleeeeep
		select {
		case <-ctx.Done():
			return nil
		case <-time.After(3 * time.Second):
		}
	}
}