package main_test

import (
	"net/http/httptest"
	"log"
	"io"
	"testing"
	"net/http"
	"fmt"
)

func TestHttpServer(t *testing.T) {
	// Server
	ts := httptest.NewServer(http.HandlerFunc(
		func(w http.ResponseWriter, r *http.Request){
			fmt.Fprintf(w, "Hello")
		},
	))
	defer ts.Close()

	// Client 
	client := ts.Client()
	res, err := client.Get(ts.URL)
	if err != nil {
		t.Errorf("Failed: %v", err)
	}

	// Or just use the server URL
	res, err = http.Get(ts.URL)
	if err != nil {
		log.Fatal(err)
	}
	greeting, err := io.ReadAll(res.Body)
	res.Body.Close()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Greeting: %q", greeting)
}

