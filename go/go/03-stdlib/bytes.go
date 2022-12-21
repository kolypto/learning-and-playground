package main

import (
	"bytes"
	"fmt"
)

func PlayBytes(){
	var a = []byte{1, 2, 3}
	var b = []byte{2, 3, 4}
	
	// Compare(): -1, 0, 1
	cmp := bytes.Compare(a, b)
	fmt.Printf("cmp=%d\n", cmp)

	// Contains(): subslice is within
	ok := bytes.Contains(a, []byte{1, 2})
	fmt.Printf("contains=%t\n", ok)

	// TODO: more []byte functions. 
	// See "strings".
}