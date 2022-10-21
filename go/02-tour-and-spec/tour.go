// Every Go program is made up of packages
// Programs start running in package "main"
//
// By convention, the package name is the same as the last element of the import path.
package main

import (
	"fmt"
	"io"
	"math/cmplx"
	"math/rand"
	"runtime"
	"strings"
	"time"
)

func main(){
	num := getFavoriteNumber()
	fmt.Println("My favorite number is", rand.Intn(num))

	variableTypes()
	pointers()
	structures()
	arrays()
	maps()
	forLoops()
	conditions()
	deferring()
	functionValues()
	interfaces()
	useReader()
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
	var lab_locations map[string]GPSLocation = make(map[string]GPSLocation)
	lab_locations["Bell Labs"] = GPSLocation{40.68433, -74.39967}

	// Map literals
	lab_locations = map[string]GPSLocation{
		"Bell Labs": GPSLocation{40.68433, -74.39967}, 
		// If the top-level type is just a type name, you can omit it from the elements
		"Google": {37.42202, -122.08408}, 
	}

	// Delete element
	delete(lab_locations, "Bell Labs")

	// Get value
	fmt.Println("The value", lab_locations["NOTEXIST"]) // {0 0} returns zero value for the map's element
	v, ok := lab_locations["NOTEXIST"]
	if ok {
		fmt.Println("Value exists", v)
	} else {
		fmt.Println("Value does not exist")
	}

	// Example: word count
	wordcount := make(map[string]uint)
	sample_string := "a b c d aaa a b x y z z z"
	words := strings.Split(sample_string, " ")
	for _, word := range words {
		wordcount[word] += 1
	}
	fmt.Println(wordcount)

	UNUSED(lab_locations)
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

func functionValues(){
	// Functions can be closures: have access to local variables
	offset := -1
	add := func(x, y int) int {return offset + x+y}
	mul := func(x, y int) int {return offset + x*y}

	// Functions are values too
	var fn func(int, int) int = add 
	fmt.Println(fn(1, 2))
	fn = mul
	fmt.Println(fn(1, 2))
}


// === Methods === //

// Go does not have classes.
// But you can define methods on types.
// Methods have a special *receiver* argument

// Value receiver: will operate *on a copy* of this value
// Pointer receiver: will operate on the actual value, modifications are possible
func (v *GPSLocation) DistanceToNorthPole() float64 {
	// Interfaces may hold `nil` values.
	// In Go, it is common to write methods that gracefully handle being called with a nil receiver
	if v == nil {
		return 0
	}
	return 1000.0  // don't know yet
}
var distance = (&GPSLocation{}).DistanceToNorthPole()

// You can define methods on non-struct types too.
// Here we define a type alias and give it a method
type MyFloat float64 
func (f MyFloat) Abs() float64 {
	if f < 0 {
		return float64(-f)
	} else {
		return float64( f)
	}
}

var absFloat = MyFloat(-127).Abs()


// === Interfaces === //

// Go does not have inheritance.
// But it has interfaces: i.e. signature matching.
// A type implements an interface by implementing its methods.
// This, `GPSLocation`` is an implicit Abser
type Abser interface {
	Abs() float64 
}

// Pitfall: an interface will only match pointer receiver methods if you use it like this:
// webLoader := &WebLoader{} 
// This is because of *method sets*:
// * Pointer struct type would include all non pointer / pointer receiver methods
// * Non pointer struct type would only include non pointer receiver methods.

func interfaces(){
	var value Abser
	value = MyFloat(127)

	// Interface values can be thought of as: (value, type) tuple
	fmt.Printf("value=%v, type=%T\n", value, value)
	fmt.Printf("abs value = %v\n", value.Abs())

	// Empty interface: may hold values of any type
	var i interface{} = "hello"

	// type assertion
	// If `i` does not hold a `string`, the statement will trigger a panic
	s := i.(string) // type assertion
	fmt.Println(s)

	// test whether an interface value holds a specific type
	// If not, `t` will contain the zero value
	t, ok := i.(string)
	if ok {
		fmt.Printf("`i` is a string: %s\n", t)
	}

	// A type switch permits several type assertions in series
	switch v := i.(type) {
	case string:
		fmt.Printf("It is a string: %s", v)
	case int:
		fmt.Printf("It is an int: %d", v)
	default:
		fmt.Printf("It is something else")
	}
}

// The most ubiquitous interface is `Stringer` from "fmt".
// It's a type that can describe itself as a string.

type Person struct {
	Name string
	Age  int
}

func (p Person) String() string {
	return fmt.Sprintf("%v (%v years)", p.Name, p.Age)
}

// `error` is also an interface, with method Error() string.

type MyError struct {
	When time.Time
	What string
}

func (e *MyError) Error() string {
	return fmt.Sprintf("at %v, %s", e.When, e.What)
}

// `io.Reader` represents the read end of a stream of data
// It has a Read(b []byte) method that populates the byte slice with data

func useReader(){
	r := strings.NewReader("Hello reader!")
	b := make([]byte, 8)
	for {
		n, err := r.Read(b)

		fmt.Printf("b=%v, b[:n] = %q\n", b, b[:n])

		if err == io.EOF {
			break
		}
	}
}


// === Generics === //
