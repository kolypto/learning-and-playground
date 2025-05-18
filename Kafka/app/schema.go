package main

import (
	"context"
	"encoding/json"

	"github.com/cockroachdb/errors"
	"github.com/rs/zerolog/log"
	"github.com/twmb/franz-go/pkg/kgo"
	"github.com/twmb/franz-go/pkg/sr"
)


var SCHEMA_REG = [...]string{
	"localhost:18081",
}

type Car struct {
	Make string `json:"make"`
	Model string `json:"model"`
	Year uint16 `json:"year"`
	Engine uint16 `json:"engine"`
}

const CAR_JSON_SCHEMA = `
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "make": { "type": "string" },
    "model": { "type": "string" },
    "year": { "type": "integer" },
    "engine": { "type": "integer" },
	"km": {"type": "integer"}
  },
  "required": ["make", "model"],
  "additionalProperties": false
}
`

var carSchemaSD sr.Serde

func schemaRegistry(ctx context.Context, k *kgo.Client) error {
	// Schema Registry
	sch, err := sr.NewClient(
		sr.URLs(SCHEMA_REG[:]...),
	)
	if err != nil {
		return errors.Wrap(err, "failed to init schema registry")
	}

	// Register (or re-register) schema.
	// It will auto-update its ID/version when you add fields.
	// It will also check compatibility and refuse if not backwards-compatible!
	carSchema, err := sch.CreateSchema(ctx, `cars-value`, sr.Schema{  // register or re-register
		Type: sr.TypeJSON,
		Schema: CAR_JSON_SCHEMA,
	})
	if err != nil {
		return errors.Wrap(err, "failed to create schema")
	}
	log.Info().
		Int("id", carSchema.ID).
		Str("subject", carSchema.Subject).
		Int("version", carSchema.Version).
		Msg("Schema registered")

	// Serializer/Deserializer
	carSchemaSD.Register(
		carSchema.ID,
		Car{},
		sr.EncodeFn(func(v any) ([]byte, error) {
			return json.Marshal(v)
		}),
		sr.DecodeFn(func(b []byte, v any) error {
			return json.Unmarshal(b, v)
		}),
	)

	// Done
	return nil
}