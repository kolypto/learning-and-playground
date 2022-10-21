// Every Go program is made up of packages
// Programs start running in package "main"
//
// By convention, the package name is the same as the last element of the import path.
package main

import (
	"fmt"
	"math/cmplx"
	"math/rand"
	"runtime"
	"time"
)

func main(){
	num := getFavoriteNumber()
	fmt.Println("My favorite number is", rand.Intn(num))
}

// Exports: a name is exported if it begins with a Capital letter
// This function is not exported
func getFavoriteNumber() int {
	// NOTE: the environment is deterministic. You'll get the same number ever time! or use rand.Seed()
	return rand.Intn(10)
}

// When two named parameters share a type, you can omit the type
// A function can return multiple results
func swap(x, y string) (string, string){
	return y, x
}

// Return values can be named. They should be used to document the meaning.
func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

// Global variables can be defined like this
var global_a, global_b int = 1, 2
const Pi = 3.13

func variableTypes() {
	local_var := 3  // type inferred

	// Variable types
	var (
		// bool
		to_be bool // variables defined without an explicit initial value are given a "zero" value (0, false, "")
		// string
		hello string = "hi"
		// int  int8  int16  int32  int64
		// uint uint8 uint16 uint32 uint64 uintptr
		// byte = uint8, rune = int32 (unicode code point)
		// int, uint, uintptr: architecture-dependent 32bits or 64bits
		maxint uint64 = 1 << 64 - 1
		// float32 float64
		// complex64 complex128
		z complex128 = cmplx.Sqrt(-5 + 12i)
	)

	// type convertion
	now_float := float64(maxint)



	UNUSED(local_var, to_be, hello, maxint, z, now_float)
}


func pointers(){
	// Go has pointers
	// Go has no pointer arithmetic
	i := 42
	var p *int = &i
	*p = 21  // dereference


	UNUSED(p)
}


// Structure
type Point struct {
	X int 
	Y int 
}

func structures(){
	// construction, field access
	point := Point{1, 2}
	point = Point{X: 1}  // named field. Y:0 is implicit
	point.X += 1

	// pointer to a struct
	point_ptr := &point
	point_ptr.X += 1 // actually (*p).X, but the language allows a shortcut
}

func arrays(){
	// [n]T is an array of `n` items
	// An array's length is part of its type, so they cannot be resized.
	var words [2]string = [2]string{"hello", "world"}
	fmt.Println(words[0])

	// Slice: a dynamically-sized view into the elements of an array.
	// []T is a slice
	// A slice does not store data. It's a reference to an array
	primes := [6]int{2, 3, 5, 7, 11, 13}
	var s []int = primes[1:4]  // a slice of [low : high], excluding `high`. You can omit "low" or "high"

	// Slice literal: creates an array, builds a slice
	s = []int{1, 2, 3}
	fmt.Println(len(s))  // slice length: the number of elements it contains
	fmt.Println(cap(s))  // slice length: the number of elements in the underlying array, counting from the first element in the slice

	// The zero value of a slice is `nil`: has a length and capacity of `0`
	s = nil

	// Create a slice with make():
	a := make([]int, 5)  // a slice referring to a zeroed array
	a = make([]int, 0, 5)  // len=0, cap=5

	// Appending to a slice
	// if the backing array is too small, a bigger array will be allocated
	var sl []int = nil  
	sl = append(sl, 0)  // will point to a new slice 
	sl = append(sl, 1)


	// Iterate over a slice or map: (index, copy of the element)
	// for _, value := range sl // skip the index
	// for i := range sl // skip the value
	for i, v := range sl {
		fmt.Printf("key=%d, value=%d", i, v)
	}

	UNUSED(s, a)
}

type GPSLocation struct {
	Lat, Lon float64
}

func maps(){
	// Map: { key => value }
	// Zero value: `nil`. Has no keys, no keys can be added.

}


// Avoid the "declared but not used" error
func UNUSED(x ...interface{}) {}



// === Flow control statements === //

func forLoops(){
	// Go has only one looping construct: for
	// "init" and "post" statements are optional. Drop them, omit ";", and you get a "while" loop
	// Skip the condition altogether, and you get an infinite loop
	sum := 0
	for i := 0; i< 10; i++ {
		sum += i
	}
	fmt.Println(sum)
}

func conditions(){
	// if
	if 1 > 0 {
		fmt.Println("True")
	}

	// if, with a short statement. 
	// The defined variable is only visible in the scope
	if v := 1 ; v > 0 {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
	
	// switch.
	// It only runs one statement. There's no "break"
	// Evaluation order: from top to bottom.
	switch os := runtime.GOOS; os {
		case "darwin":
			fmt.Println("OS X")
		case "linux":
			fmt.Println("Linux")
		default:
			// FreeBSD, OpenBSD, Plan9, Windows, ...
			fmt.Printf("%s", os)
	}

	// switch , with expressions.
	// With no condition, it's a cleaner way to write long if-then-else statements
	t := time.Now()
	switch {
	case t.Hour() < 12:
		fmt.Println("Good morning!")
	case t.Hour() < 17:
		fmt.Println("Good afternoon")
	default:
		fmt.Println("Good evening")
	}
}

func deferring(){
	// `defer` defers the execution of a function until the function returns.
	// the arguments, however, are evaluated immediately, but the call is deferred
	// They are executed in LIFO order: as a stack
	defer fmt.Println("world")
	fmt.Print("Hello ")
}
