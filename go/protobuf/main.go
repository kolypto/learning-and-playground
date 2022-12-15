package main

import "log"

func main() {
	if err := PlayProtobuf(); err != nil {
		log.Fatalf("playProtobuf() failed: %+v", err)
	}

	if err := PlayGRPC(); err != nil {
		log.Fatalf("gRPC() failed: %+v", err)
	}
}
