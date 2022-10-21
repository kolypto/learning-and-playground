// Filename ends with "_test.go": this is a test that `go test` will run
// Run me:
// $ go test

package greetings

import (
	"regexp"
	"testing"
)

// every Test*() function is a test


func TestHello (t *testing.T) {
	name := "GlaDOS"
	want := regexp.MustCompile(`\b` + name)

	// Test: name
	msg, err := HailByName(name)
	if !want.MatchString(msg) || err != nil {
		t.Fatalf(`Hello(%v)=(%q,%v) no match for %#q`, name, msg, err, want)
	}

	// Test: empty 
	msg, err = HailByName("")
	if msg != "" || err == nil {
		t.Fatalf(`Hello("") = (%q, %v) should have failed`, msg, err)
	}
}
