package main 

import (
	"fmt"
	"bytes"
	"encoding/hex"
	"testing"
	"math/rand"
)

// Test*() function
func TestXxx(t *testing.T){
	// Register a cleanup function
	t.Cleanup(func(){
		// ... clean-up when the test completes
	})
	
	// Example test
	value := 1+1
	t.Logf("Result: %d", value) // records the text in the error log

 	if value != 2 {
		// Errorf(): Log() + Fail()
		t.Errorf("Something's wrong with math in this universe: 1+1=%d", value)
		t.Fail()  // Marks it as failed, but continues execution

		t.FailNow()  // Marks it as failed and stops execution
		t.Fatal("reason")  // = Log() + FailNow()
	}

	// Ends: FailNow(), Fatal(), Fatalf(), SkipNow(), Skip(), Skipf()
	// These methods should only be called from the current goroutine

	// Helpers
	t.Setenv("NAME", "value")  // set env variable temporarily
	t.TempDir()  // a temporary directory for the test to use. It will be removed.
}

// Markers
func TestMarkers(t *testing.T){
	t.Helper()  // mark the calling function as a test helper function: is skipped in tracebacks
	t.Parallel()  // signals that the test is to be run with (and only with!) other parallel tests
}

// Subtests
func TestGroup(t *testing.T){
	// .. common setup code

	// Run(): runs f() in a separate goroutine and blocks.
	// If f() calls Parallel() and becomes a parallel test, does not block.
	t.Run("A=1", func(t *testing.T){})
	t.Run("A=2", func(t *testing.T){})

	// .. common teardown code
}

// Main function: runs in the main goroutine and sets up whatever's necessary
func TestMain(m *testing.M){
	// .. set up

	m.Run()

	// .. tear down
}


// Benchmarks are run with $ go test -bench .
func BenchmarkXxx(b *testing.B){
	// Do expensive set up
	// Register a cleanup function
	b.Cleanup(func(){
		// ... clean-up when the test completes
	})

	// Start measuring from this point
	b.ResetTimer()

	for i:=0; i<b.N; i++ {
		rand.Int()
	}

	// Custom additional metric
	b.ReportMetric(127, "requests")

}

// It will also run Example*() functions
func Example_printHello() {
    fmt.Println("hello")
    // Output: hello
}

// Fuzz tests: test with randomly generated inputs
func FuzzHex(f *testing.F){
	// Seed
	f.Add([]byte{9})
	f.Add([]byte{1, 2, 3})

	f.Fuzz(func(t *testing.T, in []byte){
		enc := hex.EncodeToString(in)
		out, err := hex.DecodeString(enc)
		
		if err != nil {
			t.Fatalf("%v: decode: %v", in, err)
		}
		
		if !bytes.Equal(in, out) {
			t.Fatalf("%v: not equal to %v", in, out)
		}
	})
}

// Test flags:
func TestTimeConsuming(t *testing.T) {
	// Skip tests that are time consuming
	// Use: $go test -test.short
    if testing.Short() {
        t.Skip("skipping test in short mode.")
    }

	// Verbose output?
	// Use: $go test -test.v
	if testing.Verbose(){
		fmt.Printf("Print something else\n")
	}
}

