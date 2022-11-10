package main

import (
	"fmt"
	"log"
	"time"
)

func PlayTime(){
	// After(): wait for duration, then send the current time on the returned channel
	// For efficiency: use NewTimer(d).C instead
	var neverSendingChan chan int
	select {
	case m := <-neverSendingChan:
		fmt.Printf("Received: %d\n", m)
	case <-time.After(10 * time.Microsecond):
		fmt.Printf("Timed out\n")
	}

	// NewTimer() sends the current tiem after at least `d` duration
	// AfterFunc() waits for the duration to elapse, and then calls f() in its own goroutine
	timer := time.AfterFunc(1*time.Millisecond, func(){
		fmt.Printf("Beep! Beep! Timer\n")
	})
	timer.Stop()  // cancel

	// Sleep(): pause the current goroutine
	time.Sleep(100 * time.Millisecond)

	// Tick() provides a ticking channel.
	// The ticker will adjust the time interval or drop ticks to make up for slow receivers (!)
	// NOTE: it leaks!! not GC-collected! Use NewTicker() and `defer Stop()` it 
	c := time.Tick(10 * time.Millisecond)
	startedAt := time.Now()
	for next := range c {
		fmt.Printf("Tick %v: status update\n", next)  //-> "m=+0.207104495: status update"

		// time.Since(t): shorthand for time.Now().Sub(t)
		if time.Since(startedAt) > (30 * time.Millisecond) {
			break
		}
	}



	// time.Since(t): Duration since `t`, the start time in the past
	// time.Until(t): Duration until `t`, the deadline in the future
	elapsed := time.Since(startedAt)
	fmt.Printf("Elapsed: %v\n", elapsed)

	// Timezone
	// Use "", "UTC" for UTC. Use "Local" for local.
	newYork, _ := time.LoadLocation("America/New_York")
	beijing := time.FixedZone("Beijing Time", int((8 * time.Hour).Seconds()))
	local, _ := time.LoadLocation("Local")
	
	timeInNewYork := time.Date(2009, 1, 1, 12, 0, 0, 0, newYork)
	fmt.Printf("Time: %v\n", timeInNewYork)



	// Time. Thread-safe, except for GodDecode, UnmarshalBinary, UnmarshalJSON, Unmarshaltext.
	// Compare: Before(), After(), Equal()
	// Math: Add() a duration, Sub() dates to get a duration.
	// Each Time has associated with it a Location.
	now := time.Now()  // current local time
	parsedTime, err := time.ParseInLocation(time.RFC3339, "2022-09-01T00:00:00+03:00", local)
	if err != nil {
		log.Fatal("Failed to parse the time: %v", err)
	} else {
		fmt.Printf("Parsed time: %v\n", parsedTime)
	}

	// Time.AddDate(y, m, d) adds this number of years
	nextYear := now.AddDate(1, 0, 0)
	fmt.Printf("Next year: %v\n", nextYear)

	// Time.Clock() returns (hour, minute, second)
	// Time.Date() returns  (year, month, day)
	h, m, s := now.Clock()
	Y, M, D := now.Date()
	fmt.Printf("Now: %04d-%02d-%02d %02d:%02d:%02d\n", Y,M,D, h,m,s)

	// Convert to string
	// Time.Format()
	// Time.AppendFormat() will write to a []byte buffer
	nowInBeijing := now.In(beijing).Format(time.RFC3339)
	fmt.Printf("Formatted time: %s\n", nowInBeijing)
}
