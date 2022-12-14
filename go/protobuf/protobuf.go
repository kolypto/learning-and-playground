package main

// $ sudo apt install protobuf-compiler  protobuf-compiler-grpc
// $ go install google.golang.org/protobuf
// $ go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
// $ go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

//go:generate protoc --go_out=protoc/go --go-grpc_out=protoc/go --python_out=protoc/python --experimental_allow_proto3_optional proto/example.proto

import (
	"fmt"
	"goplay/protobuf/protoc/go/goplay/protobuf/goplaypb"

	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/encoding/prototext"
	"google.golang.org/protobuf/proto"
)


func PlayProtobuf() error {
	// Create
	user := goplaypb.UserInfo{
		Login: "kolypto",
		Email: "kolypto@gmail.com",
		Age: proto.Uint32(35),	 
	}

	// Marshal
	out, err := proto.Marshal(&user)
	if err != nil {
		return err 
	}

	fmt.Printf("Marshal() user: %q\n", out)
	

	// Unmarshal
	if err := proto.Unmarshal(out, &user); err != nil {
		return err 
	}
	fmt.Printf("Unmarshal(): %v\n", user)

	
	// Marshal JSON
	out, err = protojson.Marshal(&user)
	if err != nil {
		return err 
	}
	fmt.Printf("Marshal() JSON: %s\n", out)

	// Marshal Text
	out, err = prototext.Marshal(&user)
	if err != nil {
		return err 
	}
	fmt.Printf("Marshal() text: %s\n", out)

	return nil
}


