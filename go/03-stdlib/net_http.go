package main

import (
	"fmt"
	"html"
	"io"
	"log"
	"net/http"
	_ "net/http/pprof"
	"time"
)

func PlayHttp() {
	httpFunc()
	httpClient()
	httpServer()
}


// "net/http" funcs
func httpFunc(){
	// CanonicalHeaderKey(): Header name: title case
	headerName := http.CanonicalHeaderKey("accept-encoding")
	fmt.Printf("Header name: %s\n", headerName)  //-> "Accept-Encoding"

	// DetectContentType(): Detect MIME Content-Type of the []byte data
	contentType := http.DetectContentType([]byte("JFIF"))
	fmt.Printf("Content-type guess: %s\n", contentType)  //-> Content-type guess: text/plain; charset=utf-8

	// ParseHTTPVersion()
	major, minor, ok := http.ParseHTTPVersion("HTTP/1.0")
	fmt.Printf("HTTP version: %d.%d (parsed ok: %t)\n", major, minor, ok)


}


// HTTP client
func httpClient(){
	// Simple client
	resp, err := http.Get("http://example.com/")
	if err != nil {
		log.Fatalf("HTTP request failed: %v", err)
	} 
	defer resp.Body.Close()

	
	responseBody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Failed to read the body: %v", err)
	}
	fmt.Printf("HTTP Client Response: %q\n", &responseBody)

	if resp.StatusCode >= 300 {
		log.Fatalf("HTTP Response failed with code: %d; body: %q", resp.StatusCode, responseBody)
	}

	// Full client: complete control
	// Clients are safe for concurrent use
	client := &http.Client{
		Transport: &http.Transport{
			IdleConnTimeout: 10 * time.Second,
			DisableCompression: true,
		},
	}
	_, err = client.Get("http://example.com/")
	if err != nil {
		log.Fatalf("HTTP request failed: %v", err)
	}
}

// HTTP server
func httpServer(){
	// Server
	// We use a custom server. Alternatively, use http.ListenAndServe()
	server := &http.Server{
		Addr: ":8080",
	}
	go func(){
		log.Fatal(server.ListenAndServe())
	}()
	
	// Add a route to the default Handle func (DefaultMux)
	http.HandleFunc("/index", func(w http.ResponseWriter, r *http.Request){
		// Respond ok
		fmt.Fprintf(w, "Hello at %q", html.EscapeString(r.URL.Path))
	})
	
	// Serve files
	http.Handle(
		"/tmp/", 
		// Strip prefix: removes the prefix so that `FileServer()` can find the file
		http.StripPrefix(
			"/tmp/", 
			http.FileServer(http.Dir("/tmp")),
		),
	)

	// pprof
	// imported

	// Error
	http.HandleFunc("/error", func(w http.ResponseWriter, r *http.Request){
		// Respond with a plain text error
		http.Error(w, "Not found", http.StatusNotFound)
	})

	// 404
	http.HandleFunc("/404/a", func(w http.ResponseWriter, r *http.Request){
		// Respond with an HTTP 404 Not found error
		http.NotFound(w, r)
	})

	http.Handle("/404/b", http.NotFoundHandler())

	// Redirect
	http.Handle("/go/to/url", http.RedirectHandler("https://google.com/", http.StatusSeeOther))
	
	// Client. Make a request, then quit
	resp, err := http.Get("http://localhost:8080/index")
	if err != nil {
		log.Fatalf("HTTP request failed: %v", err)
	}
	defer resp.Body.Close()

	responseBody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("HTTP client failed to read body: %v", responseBody)
	}
	fmt.Printf("HTTP handler returned: %s\n", responseBody)  // HTTP handler returned: Hello at "/index"
}
