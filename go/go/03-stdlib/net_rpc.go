package main

import (
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
)

func PlayRPC() {
	// RPC only exports methods that are:
	// * exported
	// * have two arguments, both exported types
	// * second argument type: a pointer
	// * has return type error
	// func (t *T) MethodName(argType T1, replyType *T2) error

	// First argument: arguments provided by the caller
	// Second argument: result parameters to be returned to the caller. Not sent in case of an error.
	// Return value: if non-nil, is passed back as a string that the client sees as if created by `errors.New()`
	var serverReady = make(chan int)
	go rpcServer(serverReady)
	<- serverReady
	rpcClient()
}


func rpcClient(){
	// Get a client
	client, err := rpc.DialHTTP("tcp", "localhost:1234")
	if err != nil {
		log.Fatalf("Dialing: %s", err )
	}

	// Make a sync remote call
	args := &IntOperands{7, 8}
	var reply int 

	err = client.Call("Arithmetic.Multiply", args, &reply)
	if err != nil {
		log.Fatalf("RPC error: %s", err)
	}

	// Check result
	fmt.Printf("RPC Multiply result: %d * %d = %d\n", args.A, args.B, reply)
	

	// Make async remote call
	multCall := client.Go("Arithmetic.Multiply", args, &reply, nil)
	<-multCall.Done
	if multCall.Error != nil {
		log.Fatalf("RPC error: %s", multCall.Error)
	}
	reply = *multCall.Reply.(*int)
	
	fmt.Printf("RPC Multiply result: %d * %d = %d\n", args.A, args.B, reply)
}


func rpcServer(serverReady chan int){
	// Register the object with methods
	arithmetic := new(Arithmetic)
	rpc.Register(arithmetic)

	// Register an HTTP handler
	rpc.HandleHTTP()

	// Serve
	sock, err := net.Listen("tcp", ":1234")
	if err != nil {
		log.Fatalf("Listen error: %v", err)
	}
	serverReady <- 1
	http.Serve(sock, nil) // go
}

type Arithmetic int 

// RPC Operation
func (t *Arithmetic) Multiply(args *IntOperands, result *int) error {
	*result = args.A * args.B 
	return nil 
}

// Arguments
type IntOperands struct {
	A, B int
} 