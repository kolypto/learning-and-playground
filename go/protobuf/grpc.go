package main

import (
	"context"
	"fmt"
	pb "goplay/protobuf/protoc/go/goplay/protobuf/goplaypb"
	"net"

	"github.com/cockroachdb/errors"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

func PlayGRPC() error {
	serverReady := make(chan int)
	go startServer(serverReady)
	defer close(serverReady)
	<-serverReady
	
	err := grpcClient()
	if err != nil {
		return err
	}
	serverReady <- 0

	return nil
}

func startServer(ready chan int) error {
	// Listen
	listen, err := net.Listen("tcp", "localhost:1234")
	if err != nil {
		return err
	}

	// gRPC server
	var opts []grpc.ServerOption
	srv := grpc.NewServer(opts...)
	pb.RegisterUsersServer(srv, exampleApiServer{})

	// Serve
	go srv.Serve(listen)
	defer listen.Close()

	// Keep up
	ready <- 1
	<- ready
	return nil
}

func grpcClient() error {
	// Connect
	var opts []grpc.DialOption
	opts = append(opts, grpc.WithInsecure())

	conn, err := grpc.Dial("localhost:1234", opts...)
	if err != nil {
		return err
	}
	defer conn.Close()

	// Client
	client := pb.NewUsersClient(conn)
	ctx := context.Background()
	
	// RPC
	user, err := client.GetUserInfo(ctx, &pb.GetUserInfoArgs{UserId: 1})
	if err != nil {
		return err
	}
	fmt.Printf("gRPC GetUserInfo(): %v\n", user)

	return nil
}


// Implement the server
type exampleApiServer struct {
	pb.UnimplementedUsersServer
}

func (s exampleApiServer) GetUserInfo(ctx context.Context, in *pb.GetUserInfoArgs) (*pb.GetUserInfoResult,  error) {
	if in.UserId != 1 {
		return nil, errors.Newf("Cannot find user by id")
	}

	result := &pb.GetUserInfoResult{
		User: &pb.UserInfo{
			Id: 1,
			Login: "kolypto",
			Email: "kolypto@gmail.com",
			Age: proto.Uint32(35),	 
		},
	}
	return result, nil
}

// Test: implements interface
var _ pb.UsersServer = new(exampleApiServer)  // assert:implements
