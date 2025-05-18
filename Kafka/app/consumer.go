package main

import (
	"context"

	"github.com/cockroachdb/errors"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/twmb/franz-go/pkg/kgo"
)

func consumerServe(ctx context.Context, k *kgo.Client) error {
	topics := k.GetConsumeTopics()
	log.Info().Strs("topics", topics).Msg("Consumer")

	// Consume
	for {
		log.Debug().Msg("Poll...")

		// Read. Mind context.
		fetches := k.PollFetches(ctx)

		// Mind context. The error may simply be "cancelled"
		if errors.Is(fetches.Err0(), context.Canceled) {
			log.Info().Msg("Consumer stopping")
			return nil
		}

		// Client closed? Literally: Close() called.
		// Also see: fetches.IsClientClosed()
		if errors.Is(fetches.Err0(), kgo.ErrClientClosed) {
			log.Info().Msg("Consumer stopping")
			return nil
		}

		// Fatal errors.
		// Returns an error per partition: they may fail independently.
		if errs := fetches.Errors(); len(errs) > 0 {
			// All errors are retried internally.
			// Non-retriable errors are returned.
			for _, err := range errs {
				log.WithLevel(zerolog.FatalLevel).
					Err(err.Err).
					Str("topic", err.Topic).
					Int32("partition", err.Partition).
					Msg("Poll failed")
			}
			log.Fatal().Msg("Poll fatal")
			return errors.New("Poll fatal")
		}

		// Iterate manually
		for _, fetch := range fetches {
			for _, ftopic := range fetch.Topics {
				topic := ftopic.Topic
				for _, fpart := range ftopic.Partitions {
					// TODO: process message with a per-message context (with deadline)
					log.Info().
						Str("topic", topic).
						Int32("partition", fpart.Partition).
						Int64("offset_log_start", fpart.LogStartOffset).
						Int64("offset_high_watermark", fpart.HighWatermark).
						Int64("offset_last_stable", fpart.LastStableOffset).
						Int("n_records", len(fpart.Records)).
						Msg("Received partition")
				}
			}
		}

		// Iterate records
		for records := fetches.RecordIter(); !records.Done(); {
			record := records.Next()
			log.Info().
				Str("topic", record.Topic).
				Int32("partition", record.Partition).
				Any("headers", record.Headers).
				Str("key", string(record.Key)).
				Str("value", string(record.Value)).
				Msg("Received")

			// Say, processed
			// Rejected messages should go to DLQ for retry or report

			// Mark this record as successful
			k.MarkCommitRecords(record)
		}

		// Iterate using a callback
		fetches.EachPartition(func(fpart kgo.FetchTopicPartition) {
			// ... fpart.Records
			switch fpart.Topic {
			case "messages":
				// ... process
			case "cars":
				// Decode schema
				for _, record := range fpart.Records {
					// NOTE: raw bytes contain Schema Registry header (the so-called "wire format")
					var car Car
					if err := carSchemaSD.Decode(record.Value, &car); err != nil {
						// TODO: DLQ for failed messages
						return
					}
					log.Info().
						Any("car", car).
						Msg("Received car")
				}
			}
		})

		// Commit marked records
		if err := k.CommitMarkedOffsets(ctx); err != nil {
			log.Error().Err(err).Msg("Failed to commit offsets; we're probably in trouble")
			return errors.New("Commit failed")
		}
		k.AllowRebalance()
	}
}
