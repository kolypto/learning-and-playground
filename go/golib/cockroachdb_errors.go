package main

import (
	// "errors"
	"context"
	"fmt"

	// "github.com/pkg/errors"
	"github.com/cockroachdb/errors"
)

func playCockroachdbErrors() error {
	// Construct errors.New() + error leaf constructors
	
	// Wrap with errors.Wrap() + see other wrappers
	// Test identity with errors.Is(), errors.IsAny()
	
	// Encode with errors.EncodeError() / errors.DecodeError()
	// Sentry reports: errors.BuildSentryReport() / errors.ReportError()

	// Extract PII-free safe details: errors.GetSafeDetails() (Personally Identifiable Information)
	// Extract user-facing hints and details: errors.FlattenHints(), errors.FlattenDetails()

	err := errors.New("Failed")
	fmt.Printf("Error: %+v\n", err)  // "+v" with stack trace (same with pkg/errors)

	// Error leafs
	{
		// New(), Newf(), Errorf()
		// Leaf error with message
		// Use: common error cases
		err = errors.Newf("Failed with id=%d", 1)
		fmt.Printf("Newf(): %+v\n", err)

		// AssertionFailedf(), NewAssertionFailureWithWrappedErrf(), WithAssertionFailure()
		// Signals an assertion error / programming error
		// Use: invariant is violated ; unreachable code path is reached
		err = errors.AssertionFailedf("Impossible")
		err = errors.WithAssertionFailure(err)
		fmt.Printf("AssertionFailedf(): %+v, IsAssertionFailure()=%t\n", err, errors.IsAssertionFailure(err))

		// Handled(), Opaque(), HandledWithMessage()
		// Capture an error but make it invisible to Unwrap() / Is()
		// Use: a new error occurs while handling another one, and the original error must be hidden
		err = errors.Handled(err)
		fmt.Printf("Handled(assertion): %+v IsAssertionFailure()=%t\n", err, errors.IsAssertionFailure(err))
		
		// UnimplementedError(), WithIssueLink()
		// Captures a message string and URL to a Jira issue
		// Use: inform PM user that the feature is not yet implemented
		err = errors.UnimplementedError(errors.IssueLink{IssueURL: "app.jira.com/APP-001"}, "This feature is not implemented")
		err = errors.WithIssueLink(errors.New("Not implemented"), errors.IssueLink{IssueURL: "APP-001"})
		fmt.Printf("UnimplementedError(): %+v\n", err)
	}

	// Wrappers
	{
		// Wrap()
		// Combines message, stack, and safe details
		// Use: on error return paths
		err = errors.New("!")
		err = errors.Wrap(err, "Failed")
		fmt.Printf("Wrap(): %+v\n", err)

		// CombineErrors(), WithSecondaryErrors()
		// Use: combine -- when two errors occur, and they need to pass the Is() check
		// Use: secondary --  when an additional error occurs, and it should be hidden from the Is() check
		err = errors.WithSecondaryError(errors.New("!"), err)
		fmt.Printf("WithSecondaryError(): %+v\n", err)

		// Mark()
		// Give the identity of one error to another error
		err = errors.Mark(err, myErrType)
		fmt.Printf("Is(myErrType)=%t\n", errors.Is(err, myErrType))

		// WithStack(): Annotate with stack trace. 
		// WithMessage(): Annotate with message prefix
		// Use: never. Use Wrap() instead
		err = errors.WithStack(myErrType) // Use case: when returning a sentinel

		// WithDetail()
		// WithHint()
		// User-facing details with contextual information / hint with suggestion for action to take
		// Use: Message to be presented to a human
		err = errors.New("DB failure")
		err = errors.WithDetail(err, "Cannot find the user") // negative (context)
		err = errors.WithHint(err, "Check your input") // positive (what to do)
		fmt.Printf("Detail and Hint: details=%v, hints=%v\n", errors.GetAllDetails(err), errors.GetAllHints(err))

		// WithTelemetry()
		// Annotate with a key suitable for telemetry
		err = errors.WithTelemetry(err, "ray-id:12345")
		fmt.Printf("Telemetry: %v\n", errors.GetTelemetryKeys(err))

		// WithDomain(), HandledInDomain(), HandledInDomainWithMessage()
		// Annotate with an origin package
		// Use: at package boundaries
		err = errors.WithDomain(err, "example.com")
		fmt.Printf("Not in example.com: %t\n", errors.NotInDomain(err, "example.com"))

		// WithContextTags()
		// Annotate with key/value tags attached to a context.Context -- using `logtags` package
		// Use: when context is available
		ctx := context.WithValue(context.Background(), "something", "anything")
		err = errors.WithContextTags(err, ctx)
		fmt.Printf("Context: %v\n", errors.GetContextTags(err))
	}

	// PII-free details (Personally Identifiable Information)
	{
		// * By default, many strings are considered to be PII-unsafe: they are stripped out when building a Sentry report
		// * Some fields are assumed to be PII-safe: type, stack trace, issue link, telemetry, domain, context, format strings, argument types
		// To opt additional in to Sentry reporting:
		// * implement errors.SafeDetailer: func SafeDetails() []string
		// * use errors.Safe() wrapper
		// * use errors.WithSafeDetails()
		err = errors.Newf("Failed with user id=%d", errors.Safe(1))
		fmt.Printf("Safe: %v\n", errors.GetAllSafeDetails(err))
	}

	return nil 
}


var myErrType = errors.New("My Err")


// Example custom type
type httpErrorType struct {
	code int 
	tag string 
}

// Implements: Error interface
func (e *httpErrorType) Error() string {
	return fmt.Sprintf("#%d: %s", e.code, e.tag)
}

// Implements: Formatter()
// This enables %+v recursive application
func (w *httpErrorType) Format(s fmt.State, verb rune) { 
	errors.FormatError(w, s, verb) 
}


// Implements: SafeDetailer(): mark all fields as Safe
func (e *httpErrorType) SafeDetails() []string {
	return []string{
		fmt.Sprintf("%d", e.code),
		e.tag,
	}	
}
