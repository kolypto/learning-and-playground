package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"time"
)

func PlayNet() {
	// Basic interface: Dial(), Listen(), Accept()
	// Access to low-level interface: most likely, not necessary

	// Dial() connects to a server
	time.AfterFunc(100 * time.Millisecond, func(){
		// Connect
		conn, err := net.Dial("tcp", "localhost:9876")
		if err != nil {
			log.Fatalf("Failed to connect: %v", err)
		}

		// Send
		n, err := fmt.Fprintf(conn, "hello\n")
		if err != nil {
			log.Fatalf("Failed to send to the server: %v", err)
		} else {
			log.Printf("Sent %d bytes to the server", n)
		}

		// Receive
		response, err := bufio.NewReader(conn).ReadString('\n')
		if err != nil {
			log.Fatalf("Failed to read from the server: %v", err)
		}
		log.Printf("Received from the server: %v", response)
	})

	// Listen() creates a server
	ln, err := net.Listen("tcp", ":9876")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}
	for {
		// Accept a connection
		log.Printf("Waiting for connections...")
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("Failed to accept a connection: %v", err)
			continue
		}

		// Handle connection
		go func(conn net.Conn){
			log.Printf("Connected client: %v", conn.RemoteAddr())  // Connected client: 127.0.0.1:45172

			// Read input
			rbuffer := make([]byte, 128)
			n, err := conn.Read(rbuffer)
			if err != nil {
				log.Printf("Client read error: %v", err)
				conn.Close()
			}
			log.Printf("Read %d bytes: %v (len=%d, cap=%d)", n, rbuffer, len(rbuffer), cap(rbuffer))
			
			// Respond
			wbuffer := make([]byte, 0, 128)
			wbuffer = append(wbuffer, "hello\n"...)

			n, err = conn.Write(wbuffer)
			if err != nil {
				log.Printf("Client write error: %v", err)
			} else {
				log.Printf("Wrote: %d bytes", n)
			}
			conn.Close()
		}(conn)

	}
}

