package main

import (
	"bytes"
	"fmt"
	"sync"
)

func PlaySync(){
	// Mutex
	// Can be used by different goroutines. Cannot be re-entered.
	var m sync.Mutex  // zero value ok
	m.Lock()  // Lock, block until the mutex is available
	m.Unlock()  // Unlock. Fail is not locked.
	locked := m.TryLock()  // try locking, report whether successful. NOTE: bad design!
	fmt.Printf("Locked successfully: %t\n", locked)

	// Once. Perform the action exactly once.
	var once sync.Once
	for i:=0; i<10; i++ { 
		once.Do(func() {fmt.Println("Only once") })
	}

	// RWMutex: reader/writer mutex. Can be held by many readers, or a single writer.
	var rw sync.RWMutex
	// For writing
	rw.Lock()
	rw.Unlock()
	// For reading
	rw.RLock()
	rw.RUnlock()

	// Cond: condition, a rendezvous point for goroutines waiting for an occurrence of an event.
	// Note: in most cases, use a channel! Broadcast corresponds to closing a channel, and Signal corresponds to sending on a channel
	var cond sync.Cond = *sync.NewCond(&m)
	cond.Broadcast() // wake all goroutines
	cond.Signal() // wake one goroutine
	// cond.Wait() // wait for the condition
	
	// WaitGroup: wait for several goroutines to finish
	var wg sync.WaitGroup
	wg.Add(10)  // start 10 goroutines. Can be negative. When 0, nobody waits.
	for i := 0; i<10; i++ {
		go func(){ wg.Done() }()  // call Done when finished
	}
	wg.Wait()  // wait until the counter is zero



	// Map: map[any]any safe for concurrent use
	var parallelMap sync.Map 
	parallelMap.Store("hey", 123)

	// Pool: a cache of unused objects for later reuse. Safe for use by multiple goroutines.
	// Purpose: relieve pressure on the garbage collector. Pool provides a way to amortize allocation overhead across many clients.
	// Example: "fmt" maintains a dynamically-sized store of temporary output buffers.
	var bufPool = sync.Pool{
		New: func() any {
			return new(bytes.Buffer)
		},
	}
	b := bufPool.Get().(*bytes.Buffer)
	b.Reset()
	b.WriteString("...")
	bufPool.Put(b)
}