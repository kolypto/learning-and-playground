package main

import "fmt"

func PlayBuiltin(){
	// Arrays and Slices
	{
		// append(): add elements
		var ints = []int{1, 2, 3}
		ints = append(ints, 4, 5)

		// len(): the length of
		fmt.Printf("len() = %v\n", len(ints)) //-> 5

		// cap(): the capacity of 
		fmt.Printf("cap() = %v\n", cap(ints))  //-> 6
		
		// copy() elements from a slice
		var more_ints []int = make([]int, len(ints))
		n := copy(more_ints, ints)  // copies min(len(a), len(b)) elements
		fmt.Printf("copied: %d\n", n) //-> copied: 5
		
		fmt.Printf("ints: %v\n", more_ints)  //-> [1 2 3 4 5]
	}

	// Maps
	{
		var ages = map[string]int{"John": 32, "James": 0}
		
		// Add
		ages["Methuselah"] = 969

		// Remove an element.
		// Does not fail.
		delete(ages, "James")

		fmt.Printf("ages = %v (len=%d)\n", ages, len(ages)) //-> ages = map[John:32 Methuselah:969] (len=2)
	}

	// Debugging
	{
		print("hey\n")
		println("something", 123)
	}
}