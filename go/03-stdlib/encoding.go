package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
)

func PlayEncodingJson(){
	// Encode
	// For struct: only exported fields are encoded
	// For []byte: encodes as base64-encoded string
	// For slice: array. nil slice -> null JSON value
	{
		msg := Message{"Alice", "Hello", 123456789}
		json_str, err := json.Marshal(msg)
		if err == nil {
			fmt.Printf("JSON: %s\n", json_str) //-> JSON: {"Name":"Alice","Body":"Hello","Time":123456789}
		} else {
			fmt.Printf("JSON encoding error: %w\n", err)
		}
	}

	// Decode
	// For struct: looks for 1) Tags 2) field names 3) case-insensitive field name
	// Will only populate fields that JSON contains.
	// Will allocate slices and pointers.
	{
		msg := Message{}
		err := json.Unmarshal([]byte(`{"Name": "Alice"}`), &msg)
		if err == nil {
			fmt.Printf("Parsed: %#v\n", msg)
		} else {
			log.Println("JSON parse error: %w\n", err)
			return
		}
	}

	// Generic JSON
	{
		generic_msg := make(map[string]any)
		err := json.Unmarshal([]byte(`{"a": 1}`), &generic_msg)
		if err == nil {
			fmt.Printf("Parsed: %#v\n", generic_msg)
		} else {
			fmt.Printf("JSON parse error: %w\n", err)
		}

		for k, v := range generic_msg {
			fmt.Printf("Key: %q; ", k)
			switch v.(type) {
				case int: fmt.Printf("(int)%v\n", v)
				case float64: fmt.Printf("(float)%v\n", v)
				case string: fmt.Printf("(string)%v\n", v)
				case []any: panic("Nested arrays not supported")
				default: fmt.Println()
			}
		}
	}

	// Decoding from a stream
	{
		json_stream := bytes.NewBufferString(`{"a": 1, "b": 2}`)
		dec := json.NewDecoder(json_stream)
		dec.DisallowUnknownFields() // extra fields not allowed xD
		var v map[string]any
		if err := dec.Decode(&v); err != nil {
			log.Println("Decoding failed", err)
			return
		}
		fmt.Printf("Decoded: %#v\n", v)
	}


	// Functions
	// Compact() removes whitespace
	{
		var dst = bytes.NewBuffer([]byte{})
		err := json.Compact(dst, []byte(`{ "a": 1 }`))
		if err != nil {
			log.Println("Compact() error", err)
		}
		fmt.Printf("Compact JSON: %s\n", string(dst.Bytes()))
	}

	// HTMLEscape() replaces HTML tags with escape sequences: for safe embedding inside HTML <script> tags
	{
		var dst = bytes.NewBufferString(``)
		json.HTMLEscape(dst, []byte(`{"tag": "<a>"}`))
		fmt.Printf("HTMLEscape() = %s\n", dst)
	}
}

type Message struct {
	Name, Body string
	Time int64
}