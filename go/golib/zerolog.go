package cmd

import (
	"context"
	"errors"
	"time"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/rs/zerolog/pkgerrors"
)


func ConfigureLogging(debug bool){
	// Output time format
	zerolog.TimeFieldFormat = time.RFC3339

	// Log level
	zerolog.SetGlobalLevel(zerolog.InfoLevel)
	if debug {
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
	}

	// Default Print() level: debug
	log.Print("Lol")

	// Structured log
	log.Debug().
        Str("Scale", "833 cents").
        Float64("Interval", 833.09).
        Msg("Fibonacci is everywhere")

	// Log without a message. Same as Msg("")
    log.Debug().
        Str("Name", "Tom").
        Send()

	// Log only if Debug logging is enabled
	if e := log.Debug(); e.Enabled() {
        // Compute log output only if enabled.
        value := "bar"
        e.Str("foo", value).Msg("some debug message")
    }

	// Log a sub-dictionary
	log.Info().
		Str("foo", "bar").
		Dict("dict", zerolog.Dict().
			Str("bar", "baz").
			Int("n", 1),
		).Msg("hello world")

	// Log error
	err := errors.New("seems we have an error here")
	log.Error().Err(err).Msg("")

	// Log with stack
	zerolog.ErrorStackMarshaler = pkgerrors.MarshalStack
	log.Error().Stack().Err(err).Msg("")

	// Sub-logger
	sublogger := log.With().
                 Str("component", "foo"). // always added
                 Logger()
	sublogger.Info().Msg("hello world")

	// Add context to the global logger
	log.Logger = log.With().Str("always", "added").Logger()

	// Add file:line numbers to log
	log.Logger = log.With().Caller().Logger()
	log.Info().Msg("hello world")

	// Pass sub-logger using context
	ctx := context.Background()
	ctx = log.With().Str("component", "module").Logger().WithContext(ctx)
	log.Ctx(ctx).Info().Msg("hello world")

	// Log fatal message
	log.Fatal().
        Err(err).
        Str("service", "service-name").
        Msgf("Cannot start %s", "service-name")
}
