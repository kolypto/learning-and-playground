package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"net/netip"
	"time"
)

func PlayNet() {
	loadWebpage()
	tcpServer()
	netFuncs()
}

// TCP client: Dial(), load webpage
func loadWebpage(){
	// NOTE: for Name Resolution, Go resolver is used: because a blocked DNS request consumes only a goroutine.
	// Cgo mode calls C library and performs a syscall. A blocked C call consumes an OS thread.
	// In some cases (OS X, $LOCALDOMAIN, $RES_OPTIONS, $HOSTALIASES, $ASR_CONFIG, /etc/resolv.conf) Go uses Cgo.
	
	// Connect
	conn, err := net.Dial("tcp", "example.com:80")
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	// Send
	n, err := fmt.Fprintf(conn, "GET / HTTP/1.0\r\n\r\n")
	if err != nil {
		log.Fatalf("Failed to send data: %v", err)
	} else {
		log.Printf("Sent %d bytes", n)
	}

	// Receive
	reader := bufio.NewReader(conn)
	line, err := reader.ReadString('\n')
	if err != nil {
		log.Fatalf("Failed to read from socket")
	} else {
		log.Printf("Received: %v", line)
	}
}

// TCP server: Listen(), Dial()
func tcpServer(){
	// Basic interface: Dial(), Listen(), Accept()
	// Access to low-level interface: most likely, not necessary

	// Dial() connects to a server
	// DialTimeout() connects with a duration
	time.AfterFunc(100 * time.Millisecond, func(){
		// Connect
		conn, err := net.DialTimeout("tcp", ":9876", 10*time.Second)
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

	done := make(chan int)  // use this channel to sync exising the function after exactly 1 client was handled.
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

			close(done)
		}(conn)

		break
	}

	<-done
}

// "net" functions
func netFuncs(){
	// Join "host:port" or "[host]:port" (for IPv6 addresses that contain a ":")
	hostport := net.JoinHostPort("localhost", "80")
	fmt.Printf("hostport: %s\n", hostport)

	// Split "host:port"
	host, port, err := net.SplitHostPort(hostport)
	fmt.Printf("Host: %s, Port: %s, Err: %v\n", host, port, err)

	// IPv4: Parse, build
	var ip net.IP
	ip = net.ParseIP("127.0.0.1")
	ip = net.IPv4(127, 0, 0, 1)
	fmt.Printf("IPv4=%v\n", ip)

	// netip: an IP that takes less memory, is immutable, and is comparable (supports == and being a map key)
	var nip netip.Addr  // IPv4 or IPv6
	nip, ok := netip.AddrFromSlice(ip)
	if !ok {
		log.Fatalf("Failed to parse IP: %v", ip)
	}
	fmt.Printf("IP: %v\n", nip)

	// netip.AddrPort: IP + port, efficient
	var ipport netip.AddrPort = netip.MustParseAddrPort("127.0.0.1:80")
	fmt.Printf("IP:port: %v\n", ipport)

	// Pipe() (Conn, Conn) creates a synchronous, in-memory, full duplex network connection.
	// Reads on one end are matched with writes on the other, copying data directly between the two. There is no internal buffering.
	recv, send := net.Pipe()
	// Send
	go send.Write([]byte("hello\n"))  // or use bufio.NewWriter(send).WriteString("hello\n")
	// Read, with timeout
	recv.SetDeadline(time.Now().Add(time.Second)) // timeout
	data, err := bufio.NewReader(recv).ReadString('\n')
	if err != nil {
		log.Fatalf("Pipe() read failed: %v", err)
	}
	fmt.Printf("Pipe() received: %q\n", data)
}