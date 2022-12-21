package main

import (
	"context"
	"log"
	"os"
	"runtime/trace"
)

func PlayTrace() {
	// When CPU profiling is active, the execution tracer makes an effort to include: 
	// goroutine create/block/unblock, syscall enter/exit/block, GC events, changes of heap size, processor start/stop, etc

	// Run tests and write the trace file:
	// $ go test -trace=trace.out
	// Then inspect the trace:
	// $ go tool trace trace.out 

	// Standard HTTP interface to trace data:
	// import _ "net/http/pprof"

	// Start tracing into a file
	tracefile, err := os.Create("/tmp/trace.out"); 
	if err != nil {
		log.Fatalf("Failed to create trace file: %v", err)
	}
	defer tracefile.Close()
	if err := trace.Start(tracefile); err != nil {
		log.Fatalf("failed to start trace: %v", err)
	}
	defer trace.Stop()

	// Tracing works with the context
	type myContextKey string
	const jobContextKey = myContextKey("job")
	ctx := context.WithValue(context.Background(), jobContextKey, "demoRuntime")

	// User annotation API: log interesting events during execution
	// Log: emits a timestamped message. Execution tracer UI can filter/group using log category and the message.
	// Region: log a time interval during a goroutine's execution. Starts and ends in the same goroutine.
	
	// Task: aids tracing of logical operations, such as an RPC request, HTTP request, any operation that involves multiple goroutines
	// Tasks are tracked via a context.
	// Task latency: time between the task creation and Task.End()
	ctx, task := trace.NewTask(ctx, "makeCoffee")
	defer task.End()

	trace.WithRegion(ctx, "makeCoffee", func(){
		trace.Log(ctx, "orderId", "1234")
	
		steamMilk := func(){
			trace.Log(ctx, "milkVolume", "0.2")
		}

		trace.WithRegion(ctx, "steamMilk", steamMilk)
	})
}