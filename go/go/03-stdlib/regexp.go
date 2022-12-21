package main

import (
	"fmt"
	"regexp"
)

func PlayRegexp(){
	// Hyperscan and re2 are 24x to 40x faster than the default Go regexp library and use 7x less memory

	// Methods: Find(All)?(String)?(Submatch)?(Index)?
	// All: Returns a slice of matches. The `n` parameter sets the max
	// String: The argument is a string. Otherwise, it's a slice of bytes.
	// Submatch: Returns a slice of matches for capturing group N
	// Index: Returns [start, end] indexes, not string matches

	var validID = regexp.MustCompile(`^\w+\[\d+\]$`)
	matched := validID.MatchString("adam[23]")
	fmt.Printf("Regexp matched: %t\n", matched)

	// Expand()
	content := "a: b\nc: d"
	search := regexp.MustCompile(`(?m)(?P<key>\w+):\s+(?P<value>\w+)$`)
	replace := "$key=$value\n"
	result := []byte{}

	// For each match of the regex in the content.
	for _, submatches := range search.FindAllStringSubmatchIndex(content, -1) {
		// Apply the captured submatches to the template and append the output
		// to the result.
		result = search.ExpandString(result, replace, content, submatches)
	}
	fmt.Println(string(result))
}