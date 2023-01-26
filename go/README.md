# Go





# go/01-hello


# go/01-hello/go.mod

```
module example.com/hello

go 1.18

// $ go mod init example.com/hello
// $ vim $hello.go

// $ cd greetings 
// $ go mod init example.com/greetings
// $ vim greetings.go
// $ cd ..

// $ go mod tidy
// $ go build
// $ ./hello

// Install?
// Install target:
// $ go list -f '{{.Target}}'
// $ go install

replace example.com/greetings => ./greetings

require rsc.io/quote v1.5.2

require example.com/greetings v0.0.0-00010101000000-000000000000

require (
	golang.org/x/text v0.0.0-20170915032832-14c0d48ead0c // indirect
	rsc.io/sampler v1.3.0 // indirect
)

```



# go/01-hello/hello.go

```go
// $ go mod init example.com/hello
// Run me:
// $ go run hello.go

// Declare a main package
package main

// Import a package -- from standard library
import (
	"fmt"
	"log"
	"math/rand"
	"time"

	"example.com/greetings"

	"rsc.io/quote"
)

// Import another package, download
// Load it:
// $ go mod tidy

// This function gets executed when you run the "main" package
func main(){
	// Configure logging
	log.SetPrefix("01-hello: ")
	log.SetFlags(0) // don't print time, source file, line number

	// Get message, handle errors
	names := []string{"world", "people", "fish"}
	messages, err := greetings.HailByNames(names)
	if err != nil {
		log.Fatal(err) // print and exit
	}

	// Print
	fmt.Println(messages)
	fmt.Println(quote.Go())
}



// Gets executed at program startup, after global variables have been initialized
func init() {
	// init random seed
    rand.Seed(time.Now().UnixNano())
}

```





# go/01-hello/greetings


# go/01-hello/greetings/go.mod

```
module example.com/greetings

go 1.18

```



# go/01-hello/greetings/greetings.go

```go
package greetings

// Import a package -- from standard library
import (
	"errors"
	"fmt"
	"math/rand"
)

// A function returns a string and error struct
func HailByName(name string) (string, error) {
	// Error handling
	if name == "" {
		return "", errors.New("no name provided")
	}

	// var message string
	message := fmt.Sprintf(randomGreetMessage(), name)
	return message, nil
}


// Greet multiple people
// Returns: map { name => greeting }
func HailByNames(names []string) (map[string]string, error){
	// Results: mapping { name => greeting }
	messages := make(map[string]string)  // map[key-type] value-type

	for _, name := range names {
		message, err := HailByName(name)
		if err != nil {
			return nil, err
		}

		messages[name] = message
	}

	return messages, nil
}

// local function: starts with a lowercase letter
func randomGreetMessage() string {
	// Declare a slice.
	// Empty [] means its size can be changed.
	messages := []string{
		"Hi %v!",
		"Hello %v!",
		"Greetings %v!",
	}
	return messages[rand.Intn(len(messages))]
}
```



# go/01-hello/greetings/greetings_test.go

```go
// Filename ends with "_test.go": this is a test that `go test` will run
// Run me:
// $ go test

package greetings

import (
	"regexp"
	"testing"
)

// every Test*() function is a test


func TestHello (t *testing.T) {
	name := "GlaDOS"
	want := regexp.MustCompile(`\b` + name)

	// Test: name
	msg, err := HailByName(name)
	if !want.MatchString(msg) || err != nil {
		t.Fatalf(`Hello(%v)=(%q,%v) no match for %#q`, name, msg, err, want)
	}

	// Test: empty 
	msg, err = HailByName("")
	if msg != "" || err == nil {
		t.Fatalf(`Hello("") = (%q, %v) should have failed`, msg, err)
	}
}

```





# go/02-tour-and-spec
# The Go Programming Language Spec

<https://go.dev/ref/spec>, Version of June 29, 2022

## Lexical Elements 

Comments:

```go
// comment
/* comment
 */
```

Semicolons: are automatically inserted when a line ends with: 
an identifier, a constant (int/float/string/...), a keyword, an operator `++` `--`, a bracket `) ] }`.

Identifiers: national names are supported:

```
identifier = letter { letter | unicode_digit }
letter        = unicode_letter | "_" 
unicode_letter = /* a Unicode code point categorized as "Letter" */ 
unicode_digit  = /* a Unicode code point categorized as "Number, decimal digit" */ 
```

Reserved keywords:

```
break        default      func         interface    select
case         defer        go           map          struct
chan         else         goto         package      switch
const        fallthrough  if           range        type
continue     for          import       return       var
```

Operators:

```
+    &     +=    &=     &&    ==    !=    (    )
-    |     -=    |=     ||    <     <=    [    ]
*    ^     *=    ^=     <-    >     >=    {    }
/    <<    /=    <<=    ++    =     :=    ,    ;
%    >>    %=    >>=    --    !     ...   .    :
     &^          &^=          ~
```

Integer literals:

```go
42000
42_000

0b000111
0o755 , 0755
0xDEADBEEF
```

Float literals:

```go
0.
.25
072.40
1.e+0
```

Rune literal: a rune constant, an integer value identifying a Unicode code point. 

```go
'x'
'\n'
'\377' // octal
'\x07' // hex byte
'\u12e4' // little u value
'\U00101234' // big u value
```

String literals: 

* raw string literals <code>\`foo\`</code>: backslashes have no special meaning, carriage return `\r` characters are discarded
* Interpreted string literals `"bar"`: backslashes are interpreted, newlines not allowed.

Struct types:

```go
struct {
    x, y float32
    _ int  // padding
    
    // Embedded field: a field declared with type but with no explicit field name
    T1      // Field name is `T1`
    *T2     // Field name is `T2`
    P.T3    // Field name is `T3`

    // Tags: a field can be followed by a string literal which becomes an attribute of that field
    // They can be accessed through the 'reflection interface' and take part in type identity for structs
    microsec uint64 `protobuf:"1"`
}
```

Pointer types:

```go
*Point
*[4]int
```

Function types:


```go
func(a, b, int) bool
func(a, b int, z float64, opt ...interface{}) (success bool)
```

Interface type: defines a *type set*.

A *type set*: a set of types which implement all methods of an interface. 

Empty interface matches all types. Convenience shortcut: `any`.

*Basic interfaces*: Interfaces whose type sets can be defined entirely by a list of methods.

```go
type Reader interface {
	Read(p []byte) (n int, err error)
	Close() error
}

type Writer interface {
	Write(p []byte) (n int, err error)
	Close() error
}

// Interface includes other interfaces
type ReadWriter interface {
	Reader
	Writer
}
```

General interfaces:

```go
// An interface representing only the type int.
interface {
	int
}

// An interface representing all types with underlying type int.
interface {
	~int
}

// An interface representing all types with underlying type int that implement the String method.
interface {
	~int
	String() string
}

// An interface representing an empty type set: there is no type that is both an int and a string.
interface {
	int
	string
}

// Union elements denote unions of type sets:

type Float interface {
    ~flaot32 | ~float64
}
```

Map types: unordered group of elements of one type, indexed by a set of unique keys of another type.

```go
map[string]int
map[*T]struct{x, y float64}

// Initialize
m := make(map[string]int)
m := make(map[string]int, 100) // argument: optional size hint. Maps will grow to accommodate the number
m = nil // a nil map is equivalent to an empty map, but no elements may be added

// Add, retrieve, delete
m["name"] = 100
value := m["name"]
delete(m["name"])
```

Channels: FIFO queues:

```go
chan T      // can be used to send and receive
chan<- T    // can be used only to send
<-chan int  // can be used only to receive

// Initialize
// Capacity: 
// * when 0, the channel is unbuffered, and communication succeeds only when both a sender and receiver are ready
// * when >0, then channel is buffered and communication succeeds without blocking if the buffer is not full (sends) or empty (receives)
ch := make(chan int, 100)
close(ch) // close it
```

Receiving:

```go
// The expression blocks until a value is available.
// Receiving from a `nil` channel blocks forever.
// A receive operation on a closed channel can always proceed immediately, yielding the element's zero value
v1 := <-ch

// Multi-value assignment: reports `ok=false` if a zero value was generated because the channel is closed or empty
x, ok = <-ch
x, ok := <-ch
var x, ok = <-ch
var x, ok T = <-ch
```

Sending:

```go
// A send on an unbuffered channel can proceed if a receiver is ready.
// A send on a    buffered channel can proceed if there's room in the buffer
// A send on a closed channel proceeds by causing a run-time panic.
// A send on a `nil` channel blocks forever.
ch <- 3
```

## Properties of Types and Values

Underlying types: each type `T` has an *underlying type*.
* For bool, num, str, type, the underlying type is the type itself.
* Otherwise, it's the type to which `T` refers in its declaration

```go
type (
    A1 = string  // underlying type: string
    A2 = A1  // same
)
type B []A1  // underlying type is []A1
```

Two types are either *identical* or *different*. 
* A named type is always different
* Otherwise, two types are identical if their underlying type literals are structurally equivalent

*Method set* of a type: methods that can be called on an operand of that type

## Declarations and Scope

Blank identifier `_`: a special anonymous placeholder.

Identifiers are *exported* when they start with an uppercase letter. 

Constants:

```go
const (
    size int64 = 1024
    eof = -1
)
const a, b, c = 3, 4, "foo"
const u, v float32 = 0, 3
```

Within a constant definition, `iota` represents successive untyped integer constants.
Its value is the index of the respective *ConstSpec*:

```go
const (
    c0 = iota  // 0
    c1 = iota  // 1
)
const (
    a = 1 << iota  // a == 1  (iota == 0)
	b = 1 << iota  // b == 2  (iota == 1)
	c = 3          // c == 3  (iota == 2, unused)
	d = 1 << iota  // d == 8  (iota == 3)
)
const (
    Monday = iota
    // Auto-fill using implicit repetition of the last non-empty expression
    Tuesday
    Wednesday
    Thursday
    Friday
    Saturday
    Sunday
)
const (
    bit9, mask0 = 1 << iota, 1 << iota - 1 // multiple uses have the same value
    bit1, mask1
    _, _
    bit3, mask3  // uses implicit repetition of the last non-empty expression list
)
```

Types.
If a type name specifies type parameters, it's a *generic type*. Generic types must be *instantiated* when they are used.

```go
type List[T any] struct {
    next *List[T]
    value T
}
```

Type parameters specify not types, but *type constraints*: a set of permissible type arguments. 
I.e. you can use `A | B` (union) and `~A` (underlying type).

```go
[P any]
[S interface{ ~[]byte|string }]
[S ~[]E, E any]
[P Constraint[int]]
[_ any]


// Normally you would write interfaces with embedded types
[P interface{E}]
// But Go allows a shortcut:
[P E]
```

Variable declarations

```go
var a, b, c int
var k = 0
var x, y int = -1, 2
var (
    i int,
    u, v, s = 2.0, 3.0, "bar"
)

// Short variable declarations
// Infer type. May redeclare variables -- only when the type is the same, and when they're from the same block!
i, j := 0, 10
ch := make(chan int)
```

## Expressions

Qualified identifiers: `PackageName.identifier`

Composite literals:

```go
// Struct
origin := Point3D{}                            // zero value for Point3D
line := Line{origin, Point3D{y: -4, z: 12.3}}  // zero value for line.q.x

// Arrays
buffer := [10]string{}             // len(buffer) == 10
intSet := [6]int{1, 2, 3, 5}       // len(intSet) == 6
days := [...]string{"Sat", "Sun"}  // len(days) == 2

// Slice
[]T{x1, x2, … xn}

// Nested composite literals: can omit type name
[...]Point{{1.5, -3.5}, {0, 0}}     // same as [...]Point{Point{1.5, -3.5}, Point{0, 0}}
[][]int{{1, 2, 3}, {4, 5}}          // same as [][]int{[]int{1, 2, 3}, []int{4, 5}}
map[string]Point{"orig": {0, 0}}    // same as map[string]Point{"orig": Point{0, 0}}
```

Slice expressions: `low: high`.
But the full slice expression is this:

```go
a[low : high : max]  // controls slice's capacity by setting it to `max-low`
```

### Selector

For a *primary expression* that is not a package name, `x.f` denotes a field of `x`.
It may also be an embedded field. As an exception, `(*x).f` may be written as just `x.f`.

Method expressions `T.M` returns a function where receiver is moved into the first argument:
`M(receiver T, ...)`.
Similarly, `(*T).M` -> `M(receiver *T, ...)`.

For an instance, `x.M` is a *method value*.
I.e. these two invocations are equivalent:

```go
t.Mv(7)
f := t.Mv; f(7)
```

### Type Assertions

Asserts that `x` is not `nil` and that the stored value is identical to `T` (implements the interface).
If not, run-time panic occurs.

```go
x.(T)  // not nil, is identical to `T`

// Type assertion test: yields a boolean value
v, ok = x.(T)
v, ok := x.(T)
var v, ok = x.(T)
var v, ok interface{} = x.(T) 
```

### Calls

Normally, parameters are passed positionally and are evaluated in the usual order.

As a special case, `f(g(parameters))` can be used if the return values of `g` are equal in number and individually assignable to the parameters of `f`.
If `f` has a final parameter `...`, it is assigned the return values of `g` that remain after assignment of regular parameters.

Variadic parameters:

```go
func Greeting(who ...string){
    // who: type []string
}
```

### Comparisons

Equality `==` and `!=` operators apply to operands that are *comparable*. 
Ordering operators `<` `<=` `>` `>=` apply to operands that are *ordered*.

* Booleans: comparable
* Int, float, string: comparable and ordered.
* Complex: comparable
* Pointer: comparable
* Channel: comparable. Equal: created by the same call to `make()`, or both are `nil`
* Interface: comparable. Equal: have identical dynamic types and equal dynamic values, or both `nil`
* Struct: comparable if all fields are comparable. Equal: corresponding fields are equal (excluding `_`)
* Array: comparable, if values are comparable. Equal: corresponding elements are equal.

Slice, map, function: not comparable. Can only be compared to `nil`.


## Statements

Label:

```go
goto Error
// ...
Error: log.Panic("error encountered")
```

Label for loop:

```go
OuterLoop:
    for i = 0; i<n; i++ {
        ...
        switch a[i][j] {
            case nil:
                break OuterLoop
        }
    }
```

Assignment: `x = y`.

Assignment operation: `x op= y`.

Tuple assignment: assigns elements of a multi-valued operation to a list of variables:

```go
x, y = f()  // f() returns two values
```

Fallthrough statement: transfers control to the first statement of the next case in `switch`.

Go: `go expression` starts a new goroutine. When the function terminates, its goroutine also terminates. 
Any return values are discarded.

Defer: `defer` fucntions are executed immediately before the surrounding function returns. In reverse order.


## Built-in functions

`close(channel)`: records that no more values will be send on the channel.
Sending on a closed channel will cause a run-time panic.
When closed, receive operations will return the zero value without blocking.

`len()`, `cap()`: always returns an `int` (any platform). 

`len()` returns the length of strings, arrays, slices, maps. 
`cap()` returns the length of the array, slice capacity.

`len(channel)` returns the number of elements queued in channel buffer, 
and `cap(channel)` returns the channel buffer capacity.

At any time, the following relationship holds:

> 0 <= len(s) <= cap(s)

For a `nil` slice/map/channel, `len()=0`, `cap()=0`.

The expression `len(s)` is a constant for: string constants, array. In this case, the expression is not evaluated.

`new(T)` takes a a type, allocates storage, initializes it with a zero value, and returns a pointer `*T`.

`make(T, ...)` is used when the *core type* of `T` is a slice/map/channel. It returns a value (not a pointer).
It has optional parameters: slice capacity, map initial capacity, channel buffer size:

```go
make(T, n)      // slice, len()=n, cap()=n
make(T, n, m)   // slice, len()=n, cap()=m

make(T, n)      // map, initial space for approx. `n` elements

make(T, n)      // channel, buffer size n
```

`append()` appends several values to the slice and returns the resulting slice.
If capacity of the slice is not large enough, it allocates a new array.

```go
s0 := []int{0, 0}
s1 := append(s1, 3, 5, 7)
s2 := append(s2, s1...)  // append a slice

// Array of any
var t []interface{}
t = append(t, 42, 3.1415, "foo")

// Special case: append a string to []byte
var b []byte
b = append(b, "bar"...)
```

`copy(dst, src)` copies slice elements, returns the number of elements copied.
As a special case, it can copy a `string` into a `[]byte` string.

`delete(map, key)` removes the element with key `key` from a map.
If the map is `nil`, delete is a no-op.


## Handling Panics

Two built-in functions:

```go
func panic(interface{})
func recover() interface{}
```

A call to `panic()` terminates the execution of the current function, but any deferreds are executed as usual.
Next, the top-level function terminates, with its deferreds. 
In the end, the program is terminated and the error condition is reported.

```go
panic(42)
panic("unreachable")
panic(Error("cannot parse"))
```

The `recover()` function allows a program to manage behavior of a panicking goroutine. 
It is usually run in a deferred function.

`recover()` returns the panic value. If you return normally, without starting a new panic, the panicking sequence stops,
normal execution resumes.

`recover()` returns `nil` when there's no panic, or the argument was `nil`, or when `recover()` was not called directly
by a deferred function.

```go
func something(){
    defer func(){
        if x:= recover() ; x != nil {
            log.Printf("run time panic: %v", x)
        }
    }

    // ...
}
```


## Import

Importing:

```go
import   "lib/math"         // math.Sin
import m "lib/math"         // m.Sin: alias
import . "lib/math"         // Sin: direct

// Import solely for side-effects (initialization)
import _ "lib/math"

```


## Initialization

Variables may be initialized using `init()` declared in the package block. 
Multiple such functions may be defined per package.
The `init()` identifier itself is not declared: it cannot be referred to from anywhere in a program.

A complete program is created by linking a single, unimported, package `"main"`.
It must have a `main()` function. When the function returns, the program exists. 
It does not wait for other non-main goroutines to complete.


## Errors
Predeclared:

```go
type error interface {
	Error() string
}
```


## Package `unsafe`

Package `"unsafe"` provides facilities for low-level programming including operations that violate the type system. 



































# Effective Go

<https://go.dev/doc/effective_go>, 25.10.2022

## Formatting
`gofmt` handles the indentation and the lining up. Don't fight with it.

We use tabs for indentation. Use spaces only if you must.

## Commentary
`//` and `/* .. */`. 
Comments that go before a declaration are doc comments.



## Names
Convention: package name in lowercase. Source directory has the same name.

Remember that the importer sees the package name: `bufio` package has `Reader`, not `BufReader`, because users see it as `bufio.Reader`.

Examples of names that read well: `ring.New` (not `ring.NewRing`), `once.Do`. 
Long names don't always automatically make things more readable.
A helpful doc comment can often be more valuable than an extra long name.

Getters and setters. When the field is called `owner`, use `Owner` for getters, `SetOwner` for setters.

Interfaces are named: method name + `-er`.

Finally, the convention is to use `MixedCaps` or `mixedCaps` rather than underscores.

## Semicolons

Semicolons are automatically inserted if the line ends with an identifier, a basic literal, or 

> break continue fallthrough return ++ -- ) }

This means that you cannot put a `{` on the next line after `if`!


## Control Structures

If. With an optional init statement.

For: 

```go
for init; condition; post {}
for condition {}  // while
for {}  // while true

// Mappings and arrays
for key, value := range mapping_or_array {}
for key := range mapping_or_array {}
for _, value := range mapping_or_array {}
for pos, char := range "string_value" {}  // breaks unicode code points
```

Go has no comma operator, and `++`/`--` are statements, not expressions.
Use parallel assignment:

```go
for i, j := 0, len(a)-1; i < j; i, j = i+1, j-1 {
    a[i], a[j] = a[j], a[i]
}
```

Switch evaluates cases top to bottom.
It is idiomatic to write if-else-if-else as a switch:

```go
func unhex(c byte) byte {
    switch {
    case '0' <= c && c <= '9':
        return c - '0'
    case 'a' <= c && c <= 'f':
        return c - 'a' + 10
    case 'A' <= c && c <= 'F':
        return c - 'A' + 10
    }
    return 0
}
```

Type switch:

```go
switch t := t.(type) { // it is idiomatic to reuse the name, in effect declaring a new variable, but with a different type
default:
    fmt.Printf("unexpected type %T\n", t)     // %T prints whatever type t has
case bool:
    fmt.Printf("boolean %t\n", t)             // t has type bool
case int:
    fmt.Printf("integer %d\n", t)             // t has type int
case *bool:
    fmt.Printf("pointer to boolean %t\n", *t) // t has type *bool
case *int:
    fmt.Printf("pointer to integer %d\n", *t) // t has type *int
}
```


## Functions

Function can return multiple values. Their names are optional, but they are documentation

```go
func nextInt(b []byte, pos int) (value, nextPos int) {
    ...
}
```

## Data

Allocation: `new(T)` returns a pointer to a new zero value or type `T`. 

Allocation: `make(T, ...args)` only creates slices, maps and channels, and returns an *initialized* (not zeroed) value of `T` (not `*T`).

Why the distinction? Because these types reference data structures that must be initialized before use. 
A slice, for instance, is a 3-item descriptor: (pointer to data, length, capacity).
For instance, `make([]int, 10, 100)` initializes a slice with an underlying array. But `new([]int)` returns a pointer to a newly allocated `nil` slice.

If possible, the zero value of a structure should already be usable: the "zero-value-is-useful" property.

Constructors: it's ok to return a pointer:

```go
func NewFile(fd int, name string) *File {
    if fd <= 0 {
        return nil
    }
    return &File(fd, name, nil, 0)
}
```

Literals:

```go
// Positional
return &File{fd, name, nil, 0}

// With labelling
return &File{fd: fd, name: name}

// Labelling can be used with array/slice/map literals and is ignored:
a := [...]string   {Enone: "no error", Eio: "Eio", Einval: "invalid argument"}
s := []string      {Enone: "no error", Eio: "Eio", Einval: "invalid argument"}
m := map[int]string{Enone: "no error", Eio: "Eio", Einval: "invalid argument"}
```

### Arrays and Slices

Arrays are values. Assigning one array to another copies all the elements.

If you pass it to a function, it will receive a copy, not a pointer to it.
It's expensive. Pass a pointer. Or use slices.

The size of an array is part of its type.

Slices wrap arrays. Most array programming in Go is done with slices rather than arrays.
The capacity `cap()` reports the limit of the underlying array. If the data exceeds the capacity, 
the slice is reallocated.

If you need a 2D slice, there are two ways: allocate a new slice for each row, 
or allocate a single array and point individual slices into it. If slices don't grow, it can be more efficient
to construct the object with a single allocation.

### Maps

Maps: keys to values.

When a missing key is accessed, the zero value it returned.
Use multiple assignment to differentiate between missing values and zero values:

```go
value, ok := mapping[key] // the "comma ok" idiom
```

To test for presence without worrying about the actual value:

```go
_, present := mapping[key]
```

### Printing

`fmt.Println`, `fmt.Printf`, ...

Use `%v` to print the default conversion (the catchall format). 

It can print arrays and maps. For maps, the output is sorted by key.

* `%v` print the default representation
* `%+v` print struct, annotate fields. `%#v` prints the full Go syntax with type name
* `%q` prints a quoted string. `%#q` uses backquotes.
* `%x` hex string
* `%T`: prints the type of a value

To control the default format of a type, define a method `String() string` on the type.


## Initialization

The `iota` enumerator:

```go
const (
    _ = iota  // ignore iota=0
    KB ByteSize = 1 << (10 * iota)
    MB // implicitly repeated
    GB
    TB
    PB
    EB
    ZB
    YB
)
```

Every file can have multiple `init` methods to prepare the program state before real execution begins.

## Methods

Methods can be defined for any named type:

```go
type ByteSlice []byte

// This method requires the returning of an updated slice
func (slice ByteSlice) Append(data []byte) []byte {
    return append(slice, data)
}

// Use a pointer to be able to update the slice in place
// No return.
func (p *ByteSlice) Append(data []byte) {
    slice := append(slice, data)

    // Instead of a return
    *p = slice
}
```

The rule about pointers vs. values for receivers:

* Value methods: can be invoked on pointers and values
* Pointer methods: can be invoked on pointers only

this is because pointer methods can modify the receiver.
Invoking a pointer method on a value would receive a copy of the value, so any modifications would be discarded.
The language, therefore, disallows the mistake.

One handy exception: when the value is addressable, Go will do `(&value).Method` for you automatically. 

## Interfaces and Other types

Interface: if something can do *this*, then it can be used *here*.

It's a common idiom in Go to convert the type of an expression to access a different set of methods.

Type switch: `value.(type)` and `value.(typeName)`. Use the *comma-ok* idiom to test safely.


## Embedding

Go does not have subclasses, but can "borrow" pieces of an implementation by *embedding types*.

Interface embedding: just mention an interface.

For struct:

```go
type ReadWriter struct {
    // Methods for both will become available
    *Reader
    *Writer
}
```

Important difference from subclassing: when we invoke a method of an embedded type on our outer type,
the receiver of the method is still the inner type, not the outer one!

Embedding adds fields, but the original structure is also a regular fields:

```go
type Job {
    ...
    *log.Logger
}

// Constructor, literal
func NewJob(logger *log.Logger){
    return &Job{..., logger}
}

// Refer to a method on the embedded struct
func (job *Job) String(){
    return job.Logger.String()  // "parent" method
}
```

## Concurrency

In Go, shared values are passed around on channels. As a result, data races cannot occur by design:
only one goroutine has access to the value at any given time.

> Do not communicate by sharing memory; share memory by communicating.

Goroutines are simple functions that start executing in parallel. They're cheap, with a small starting stack.
Goroutines are multiplexed onto multiple OS threads: N:M concurrency. 

Use channels to signal results and completion. 

### Channels

```go
make(chan int) // unbuffered
make(chan int, 0) // unbuffered
make(chan *os.File, 100) // buffered channel
```

Unbuffered channels combine communication with synchronization, guaranteeing that two goroutines are in a known state.

Idiom: wait for a goroutine to complete

```go
c := make(chan int)

// Do something in the background
go func(){
    ...
    c <- 1
}

// Do something in the meanwhile
...

// Wait for results
<- c
```

An unbuffered channel: sender blocks until the receiver has received the value. 

A buffered channel: sender blocks only until the value has been copied to the buffer; if the buffer is full, 
this means waiting until some receiver has retrieved the value.

Idiom: use a buffered channel as a semaphore (to limit throughput):

```go
var sem = make(chan int, 10)

func handle(r *Request){
    sem <- 1  // wait for the active queue to drain
    process(r)
    <-sem  // done, enable next request to run
}
```

This approach creates a new goroutine for every incoming request, which is an overkill, since only 10 can run at any moment.
We can address that deficiency by changing the design: *gate* the creation of goroutines:

```go
func Serve(queue chan *Request){
    for req := range queue {
        // Get a fresh variable, deliberately shadowing the loop variable locally.
        // If we don't, all goroutines will share the same variable.
        // Alternatively: pass it as a param to the closure
        req := req
        
        
        sem <- 1
        go func(){
            process(req)
            <- sem
        }()
    }
}
```

Another approach that manages resources well: start with a fixed number of goroutines all reading from the channel:

```go
func handle(queue chan *Request){
    // keep reading, process
    for r := range queue {
        process(r)
    }
}

func Serve(clientRequests chan *Request, quit chan bool){
    // The number of goroutines limits the number of parallel processes
    for i:= 0; i<10; i++ {
        go handle(clientRequests)
    }

    <- quit // wait to be told to exit
}
```

### Channels of channels
A channel is a value. It can be sent over a channel.
This allows the implementation of safe, parallel demultiplexing.

If the job includes a channel on which to reply, each client can provide its own path for the answer:

```go
type Request struct {
    args []int
    f    func([]int) int
    result chan int
}

func main(){
    request := &Request([]int{3, 4, 5}, sum, make(chan int))
    clientRequest <- request

    // Wait for results
    result := request.resultChan
}
```

And here's the server:

```go
func handle(queue chan *Request){
    for req := range queue {
        req.result <- req.f(req.args)
    }
}
```

This code is a framework for a rate-limited, parallel, non-blocking RPC system.
And there's not a mutex in sight.

### Parallelization

Parallelize a calculation across multiple CPU cores (if the calculation can be broken into separate pieces)

```go
func DoSomeCalculation(i){
    ...
    c <- 1 // signal completion
}

const numCPU = runtime.GOMAXPROCS(0)  // =runtime.NumCPU, but can be overridden

func DoAll(){
    c := make(chan int, numCPU)

    for i:=0; i<numCPU; i++ {
        go DoSomeCalculation(i)
    }

    // Drain the channel
    for i:=0; i<numCPU; i++ {
        <-c
    }
}
```

### A Leaky Buffer

In an RPC package, to avoid allocating and freeing buffers, we can abuse a channel:

```go
var freeList = make(chan *Buffer, 100)
var serverChan = make(chan *Buffer)

// Read data into a buffer. When it's ready, send it to the server for processing
func client(){
    for {
        var b *Buffer

        // Grab a buffer, allocate if it's not available
        select {
            case b = <-freeList:
                // Got one, nothing more to do
            default:
                b = new(Buffer)
        }

        // Read next message from the net
        load(b)

        // Done
        serverChan <- b
    }
}

// Receive messages from the client, process it, then return the buffer
func server(){
    for {
        // Wait for work
        b := <-serverChan
        process(b)

        // Reuse buffer if there's room
        select {
            case freeList <- b:
                // Buffer returned. Nothing more to do.
            default:
                // Free list is full. Drop the buffer, it will be garbage collected
        }
    }
}
```

This implementation builds a leaky bucket free list in just a few lines, relying on the buffered channel and the garbage collector for bookeeping.


## Errors

Callers that care about the precise error details can use a type switch:

```go
file, err = os.Create(filename)
if err == nil {
    return
}
if e, ok := err.(*os.PathError); ok && e.Err == syscall.ENOSPC {
    deleteTempFiles()  // Recover some space.
    continue
}
```

It is preferable, however, to use the "errors" package like this:

```go
// This code will unwrap errors, looking for the one you expect
var perr *fs.PathError
if errors.As(err, &perr) {
	fmt.Println(perr.Path)
}

// This code will not work as expected with wrapped errors
if perr, ok := err.(*fs.PathError); ok {
	fmt.Println(perr.Path)
}
```

And here's an easy way to wrap an error:

> errors.Unwrap(fmt.Errorf("... %w ...", ..., err, ...))


### Panic

Panic creates a run-time error that will stop the program. It's a way to indicate that something impossible has happened.

When `panic` is called, it immediately stops execution of the current function and begins unwinding the stack of the goroutine,
running any deferred functions along the way. It that unwinding reaches the top of the goroutine's stack, the program dies.

However, it's possible to use the built-in function `recover()` to regain control of the goroutine and resume normal execution.
A call to `recover()` stops the unwinding and returns the argument passed to `panic()`. 
Because the only code that runs while unwinding is inside deferred functions, `recover()` is only useful inside deferred functions.

One application: shut down a failing goroutine without killing the other executing goroutines.

```go
func server(workChan <-chan *Work) {
    for work := range workChan {
        go safelyDo(work)
    }
}

func safelyDo(work *Work) {
    defer func() {
        if err := recover(); err != nil {
            log.Println("work failed:", err)
        }
    }()
    do(work)
}
```




# Go Doc Comments
<https://go.dev/doc/comment>, 25.10.2022

Every exported (capitalized) name should have a doc comment.

The [go/doc](https://go.dev/pkg/go/doc) and [go/doc/comment](https://go.dev/pkg/go/doc/comment) provide the ability to extract
documentation from Go source code.
The `go doc` command looks up and prints the doc comment for a given package or symbol (top level definition).

### Package

Every package should have a package comment that goes before `package`.

A package comment for a command describes the behavior of the program.

### Types

By default, programmers should expect a type to be safe for use only by a single goroutine at a time (non-threadsafe).
If a type provides stronger guarantees, the doc comment should state them:

> A \<type> is safe for concurrent use

Go types should aim to have a usable zero value. If it isn't obvious, it's meaning should be documented:

> The zero value for Buffer is an empty buffer.

For a struct with exported fields, the doc comment or per-field comment should explain the meaning of each exported field.

### Funcs

A doc comment should explain what the function returns. If there are side effects -- what it does.

Named arguments can be referred to directly.

By default, programmers can assume that a top-level func is thread-safe (safe to call from multiple goroutines).
If not, it should be stated.

Doc comments should not explain internal details such as the algorithm used: use comments inside the function.

### Syntax

Simplified Markdown with no HTML support.

Use `[Text]` followed by `[Text]: URL` for hyperlinks.

Use <code>``quoted''</code>  backticks-quotes to produce proper left-right “quotes”.

Doc links: `[Name1]` and `[Name1.Name2]` refer to exported identifiers in the current package.

Doc links: `[pkg.Name1]` and `[pkg.Name1.Name2]` refer to identifiers in other packages.
If the current package imports `encoding/json`, then `[json.Decoder]` can be written in place of `[encoding/json.Decoder]` to link to the docs.

Lists: start with `*` `+` `-` `•` `*` `+` `-` `•`.
List items can only contain paragraphs: no code blocks or nested lists. This avoids any space-counting subtlety.

A code block: a span of indented text.
Can be used for preformatted text with alignment.















# Generics
Source: <https://go.dev/doc/tutorial/generics>, 15.12.2022

A function that sums up a map of ints of floats:

```go
func SumNumbers [K comparable, V int64|float64] (m map[K]V) V {
    var sum V
    for _, v := range m {
        s += V
    }
    return V
}
```

Now call it:

```go
ints := map[string]int64{
    "first":  34,
    "second": 12,
}

SumNumbers[string, int64](ints) // with type safety
SumNumbers(ints) // simple
```

To improve on this example, define a type and use it:

```go
type Number interface {
    int64 | float64
}

func SumNumbers[K comparable, V Number](m map[K]V) V { 
    ...
}
```


# go/02-tour-and-spec/go.mod

```
module goplay/tour

go 1.18

```



# go/02-tour-and-spec/tour.go

```go
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
	goroutines()
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

// A function that works on multiple types.
// Index is applicable to any type that fulfills the built-in constraint "comparable"
func Index[T comparable](s []T, x T) int {
	for i, v := range s {
		if v == x {
			return i
		}
	}
	return -1
}

// Go also supports generic types
type List[T any] struct {
	next *List[T]
	value T
}



// === Goroutines === //

// A goroutine is a lightweight thread managed by Go runtime.
// 	go f(x, y, z)
// Arguments are evaluated immediately in the current goroutine, but f() gets executed in a new goroutine.

// They both run in the same address space: access to shared memory must be synchronized.
// You may use the "sync" package -- or better, channels.

func goroutines(){
	// sum() gets arguments, runs a calculation, returns results into a channel
	sum := func (s []int, results chan int) {
		sum := 0
		for _, v := range s {
			sum += v
		}
		results <- sum
	}

	array_of_ints := []int{7, 2, 8, -9, 4, 0}

	// Communicate through the channel
	// Optional buffer size parameter: sending will block and wait until there's free space in the buffer
	ch := make(chan int) // sending does not block (unless the buffer is full)
	go sum(array_of_ints[:len(array_of_ints)/2 ], ch)
	go sum(array_of_ints[ len(array_of_ints)/2:], ch)

	// Wait for *both* to finish
	first, second := <-ch, <-ch

	// Test whether a channel is open?
	close(ch) // Indicate that no more values will be sent. Should be done by the sender.
	v, ok := <-ch

	// ### Loop

	// Iterate over the incoming values. Stops when the channel is close()d
	for value := range ch {
		fmt.Println("incoming", value)
	}

	// Print
	fmt.Println(first, second, v, ok)

	// ### Select
	// Select lets a goroutine wait on multiple communication operations

	numbers := make(chan int, 10)
	signal_finish := make(chan int)

	go func(){
		fmt.Println("Sending...")
		numbers <- 1
		numbers <- 2
		signal_finish <- 0
		fmt.Println("Sending done")
	}()

	func (){
		var x, sum int
		for {
			fmt.Println("Listening...")
			select {
			// Wait on either
			case numbers <- x:
				sum += x
				fmt.Println("Number", x)
			case <- signal_finish:
				fmt.Println("Finish")
				return
			// Do if nothing happened (`select` would block)
			//default:
			//	fmt.Println("Default")
			//	continue
			}
		}
	}()
}
```



# go/02-tour-and-spec/tour_webserver.go

```go
/*
Webserver serves a page on :1718 by default. It's an app to generate QR codes.
*/
package main

import (
	"flag"
	"html/template"
	"log"
	"net/http"
)

// Command-line flag
var bind = flag.String("addr", ":1718", "http service address") // Q=17, R=18

func main(){
	// Parse command-line flags
	flag.Parse()

	// Serve page
	http.Handle("/", http.HandlerFunc(indexPage))
	err := http.ListenAndServe(*bind, nil)
	if err != nil {
		log.Fatal("ListenAndServe:", err)
	}
}

func indexPage(w http.ResponseWriter, req *http.Request){
	// "s": GET parameter, it sent by the form
	// Render the template, using the form value as the "current" (".") data item
	indexPageTemplate.Execute(w, req.FormValue("s"))
}

// HTML template.
// `Must()` panics if the result is non-nil
// New(name) is the template name
var indexPageTemplate = template.Must(template.New("qr").Parse(indexPageTemplateStr))

// index page
const indexPageTemplateStr = `
<!DOCTYPE html>
<html>
<head>
	<title>QR Link Generator</title>
</head>
<body>
	{{if .}}
		<img src="http://chart.apis.google.com/chart?chs=300x300&cht=qr&choe=UTF-8&chl={{.}}" />
		<br>
		{{.}}
		<br>
		<br>
	{{end}}

	<form action="/" name=f method="GET">
		<input maxLength=1024 size=70 name="s" value="" title="Text to QR Encode">
		<input type=submit value="Show QR" name="qr">
	</form>
</body>
</html>
`
```





# go/03-stdlib


# go/03-stdlib/go.mod

```
module goplay/stdlib

go 1.19

```



# go/03-stdlib/main.go

```go
package main

// Run me:
// $ go run .
// $ go test

// Build or test with race detector:
// $ go test -race mypkg
// $ go run -race mysrc.go
// $ go build -race mycmd
// $ go install -race mypkg

import (
	"log"
)

func main(){
	log.SetFlags(log.Lshortfile | log.Lmsgprefix)

	PlayFmt()
	PlayBuiltin()
	PlayBytes()
	PlayEmbed()
	
	PlayEncodingJson()
	PlayHtml()
	PlayHtmlTemplate()

	PlayOS()
	PlayOsExec()
	PlayIO()
	PlayIOFS()
	
	PlayReflect()
	PlayRegexp()
	PlayTrace()
	PlaySort()
	PlayCustomSort()
	PlaySync()
	PlayTime()

	PlayNet()
	PlayHttp()

	PlayRPC()
}

```



# go/03-stdlib/fmt.go

```go
package main

import (
	"fmt"
)



func PlayFmt(){
	// Fprint(), Fprintf(), Fprintln() write to `io.Writer`
	// Print(), Printf(), Println() write to stdout

	// %v		the value in default format
	// %#v 		Go representation
	{
		var person = &Person{"John", "Smith"}
		fmt.Printf("%%v: %v\n", person)		//-> %v: {John Smith}
		fmt.Printf("%%#v: %#v\n", person)  	//-> %#v: main.Person{FirstName:"John", LastName:"Smith"}
		fmt.Printf("%%T: %T\n", person) 	//-> %T: main.Person

		// Scalars
		fmt.Printf("%%t: %t\n", true)  		//-> %t: true
		fmt.Printf("%%d: %d\n", 127)		//-> %d: 127
		fmt.Printf("%%f: %f\n", 3.14) 		//-> %f: 3.140000
		fmt.Printf("%%.2f: %.2f\n", 3.14) 	//-> %f: 3.14
		fmt.Printf("%%g: %g\n", 3.14) 		//-> %g: 3.14
		fmt.Printf("%%s: %s\n", "hey")		//-> %s: hey
		fmt.Printf("%%q: %q\n", "hey")		//-> %q: "hey"
		// Alternate format: print "0x" for hex
		fmt.Printf("%%#X: %#X\n", 0xDEADBEEF)	//-> %#x: 0xDEADBEEF

		// Argument indexes
		fmt.Printf("[1]: %[1]d %[1]x %[1]q\n", 127)	//-> 127 7f '\x7f'
	}

	// Print into a string
	{
		var s = fmt.Sprintf("%d", 127)
		fmt.Printf("sprintf()=%s\n", s)  //-> sprintf()=127
	}

	// Scanning
	// Scan(), Scanf(), Scanln() read from stdin
	// Fscan(), Fscanf(), Fscanln() read from a `io.Reader`
	// Sscan(), Sscanf(), Sscanln() read from a string
	// * A space after a newline consumes 0+ spaces
	// * Otherwise, a space consumes 1+ spaces
	// * "\r\n" means the same as "\n"
	{
		// Scan a number
		var number int
		if n, err := fmt.Sscanf("123", "%d", &number); err == nil {
			fmt.Printf("Scanned: %%d=%d trailer (read: %d items)\n", number, n)  //-> Scanned: %d=123 trailer (read: 1 items)
		} else {
			fmt.Printf("Scanning error: %v\n", err) //-> Scanning error: expected integer
		}
	}
	

	// Appending to a byte slice
	{
		var s []byte = []byte("start")
		s = fmt.Append(s, " more")
		s = fmt.Appendf(s, " int=%d", 127)
		s = fmt.Appendln(s, "")
		fmt.Println(string(s))  //-> start more int=127
	}

	// Error
	// Use %w: unwraps errors
	err := fmt.Errorf("Failed: %s, value=%d", "Error string", 127)
	fmt.Printf("Errorf()=%#v\n", err)  //-> Errorf()=&errors.errorString{s:"Failed: Error string, value=127"}


}

type Person struct {
	FirstName, LastName string 
}

// Implement type fmt.Formatter: custom formatter
func (person *Person) Format(f fmt.State, verb rune) {
	// verb: the "%v" verb
	if verb != 'v' {
		panic("Unsupported verb")
	}
	// print into `f`
	fmt.Fprintf(f, "{fname=%s lname=%s}", person.FirstName, person.LastName)
}

// Implement type fmt.GoStringer: custom %#v
func (person *Person) GoString() string {
	return fmt.Sprintf("%T{fname=%q lname=%q}", person, person.FirstName, person.LastName)
}
```



# go/03-stdlib/builtin.go

```go
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
```



# go/03-stdlib/bytes.go

```go
package main

import (
	"bytes"
	"fmt"
)

func PlayBytes(){
	var a = []byte{1, 2, 3}
	var b = []byte{2, 3, 4}
	
	// Compare(): -1, 0, 1
	cmp := bytes.Compare(a, b)
	fmt.Printf("cmp=%d\n", cmp)

	// Contains(): subslice is within
	ok := bytes.Contains(a, []byte{1, 2})
	fmt.Printf("contains=%t\n", ok)

	// TODO: more []byte functions. 
	// See "strings".
}
```



# go/03-stdlib/embed.go

```go
package main

import (
	"embed"
	"fmt"
)

// Here's how you embed a file into a variable
//go:embed main.go
var maingo string

// Embed into a filesystem
// Use `path.Match` patterns. 
// Directories are included recursively (excluding ".*" and "_*")
//go:embed *.go
var f embed.FS

func PlayEmbed(){
	// Read string
	fmt.Println("embed", maingo[:12])
	
	// Read file
	data, _ := f.ReadFile("main.go")
	println("embed", string(data[:12]))
}
```



# go/03-stdlib/encoding.go

```go
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
)

func PlayEncodingJson(){
	// Encode
	// For struct: only exported fields are encoded
	// For []byte: encodes as base64-encoded string
	// For slice: array. nil slice -> null JSON value
	{
		msg := Message{"Alice", "Hello", 123456789}
		json_str, err := json.Marshal(msg)
		if err == nil {
			fmt.Printf("JSON: %s\n", json_str) //-> JSON: {"Name":"Alice","Body":"Hello","Time":123456789}
		} else {
			fmt.Printf("JSON encoding error: %v\n", err)
		}
	}

	// Decode
	// For struct: looks for 1) Tags 2) field names 3) case-insensitive field name
	// Will only populate fields that JSON contains.
	// Will allocate slices and pointers.
	{
		msg := Message{}
		err := json.Unmarshal([]byte(`{"Name": "Alice"}`), &msg)
		if err == nil {
			fmt.Printf("Parsed: %#v\n", msg)
		} else {
			log.Printf("JSON parse error: %v\n", err)
			return
		}
	}

	// Generic JSON
	{
		generic_msg := make(map[string]any)
		err := json.Unmarshal([]byte(`{"a": 1}`), &generic_msg)
		if err == nil {
			fmt.Printf("Parsed: %#v\n", generic_msg)
		} else {
			fmt.Printf("JSON parse error: %v\n", err)
		}

		for k, v := range generic_msg {
			fmt.Printf("Key: %q; ", k)
			switch v.(type) {
				case int: fmt.Printf("(int)%v\n", v)
				case float64: fmt.Printf("(float)%v\n", v)
				case string: fmt.Printf("(string)%v\n", v)
				case []any: panic("Nested arrays not supported")
				default: fmt.Println()
			}
		}
	}

	// Decoding from a stream
	{
		json_stream := bytes.NewBufferString(`{"a": 1, "b": 2}`)
		dec := json.NewDecoder(json_stream)
		dec.DisallowUnknownFields() // extra fields not allowed xD
		var v map[string]any
		if err := dec.Decode(&v); err != nil {
			log.Println("Decoding failed", err)
			return
		}
		fmt.Printf("Decoded: %#v\n", v)
	}


	// Functions
	// Compact() removes whitespace
	{
		var dst = bytes.NewBuffer([]byte{})
		err := json.Compact(dst, []byte(`{ "a": 1 }`))
		if err != nil {
			log.Println("Compact() error", err)
		}
		fmt.Printf("Compact JSON: %s\n", string(dst.Bytes()))
	}

	// HTMLEscape() replaces HTML tags with escape sequences: for safe embedding inside HTML <script> tags
	{
		var dst = bytes.NewBufferString(``)
		json.HTMLEscape(dst, []byte(`{"tag": "<a>"}`))
		fmt.Printf("HTMLEscape() = %s\n", dst)
	}
}

type Message struct {
	Name, Body string
	Time int64
}
```



# go/03-stdlib/html.go

```go
package main

import (
	"fmt"
	"html"
	"html/template"
	"log"
	"os"
)


func PlayHtml(){
	// HTML escape
	escaped := html.EscapeString("<tag>")
	fmt.Printf("escaped=%q\n", escaped)  //-> "&lt;tag&gt;"

	// HTML unescape
	unescaped := html.UnescapeString(escaped)
	fmt.Printf("unescaped=%q\n", unescaped)  //-> "<tag>"
}


func PlayHtmlTemplate(){
	// "html/template" provides the same interface as "text/template", but for HTML.
	// Plain text is escaped. Escaping is contextual: HTML, JavaScript, CSS, URI contexts (uses proper sanitizing)
	// Assumption: the template is trusted, the parameters are not.

	template.HTMLEscape(os.Stdout, []byte("<a>")) //-> &lt;a&gt;
	template.JSEscape(os.Stdout, []byte("<a>")) //-> \u003Ca\u003E
	fmt.Println(template.URLQueryEscaper("<a>")) //-> %3Ca%3E
	
	// I.e. This is what happens:
	//   <a href="/search?q={{.}}">{{.}}</a> 
	//   <a href="/search?q={{. | urlescaper | attrescaper}}">{{. | htmlescaper}}</a>
	// "href" enters the URI namespace. So do "data-href" and "my:href"

	{
		// Template.
		// {{...}}} is the action that gets evaluated
		// {{.}} prints the cursor
		// {{23 -}} < {{- 45}}  -- trims whitespace after, and before, the action
		// {{$variableName.fieldName}} and {{$variableName.$keyName}}
		// {{$object.Method "arg"}}  
		// -- the method mey return `_, error` 
		// -- in case of an error: execution terminates, and an error is returned by Execute()
		//
		// {{/* a comment */}}
		// {{pipeline}}  -- will use fmt.Print() 
		//
		// {{if pipeline}} T1 {{else}} T2 {{end}}
		// {{if pipeline}} T1 {{else if pipeline}} T2 {{end}}
		//
		// {{range pipeline}} item {{else}} empty {{end}}  --  iterate, set dot to the current value
		// {{break}}, {{continue}}  -- control structures for range
		//
		// {{with pipeline}} T1 {{else}} no value {{end}}  -- set dot to the value of the pipeline. If empty, do "else" (if present)
		// {{template "name"}} -- execute template with nil data
		// {{template "name" pipeline}}  -- execute template with dot set to the value of the `pipeline`
		// {{block "name" pipeline}} T1 {{end}}   -- a shorthand for defining a template and then executing it in place:
		// {{define "name"}} T1 {{end}}{{template "name" pipeline}}
		//
		// Pipeline: {{ argument.Method "arg" | ...}}
		// In a chained pipeline, the result of each command is passed as the last argument of the following command
		//
		// Variables:
		// {{range $index, $element := pipeline}}
		// {{with $x := "output" | printf "%q"}}{{$x}}{{end}}
		//
		// Strings and printing:
		// {{"\"output\""}}
		// {{`"output"`}}
		// {{printf "%q" "output"}}
		// {{"output" | printf "%q"}}
		// {{printf "%q" (print "out" "put")}}
		const templateString = `
		{{define "Named-Block"}}
			Hello, {{.}}!
		{{end}}
		`

		// Create a named template
		// Template names form a namespace of templates. You can evaluate them by name.
		// Must() panics if the error is non-nil.
		t := template.Must(template.New("name").Parse(templateString))

		// Fail when a key is missing.
		t.Option("missingkey=error")  

		// Execute a template. Use data object "T"
		err := t.ExecuteTemplate(os.Stdout, "Named-Block", "John")  //-> Hello, John!
		if err != nil {
			log.Fatal(err)
		}
		
		// Insert literal HTML (known safe HTML)
		// See also: CSS(), HTMLAttr(), JS(), JSStr(), Srcset(), URL()  -- known safe elements
		err = t.ExecuteTemplate(os.Stdout, "Named-Block", template.HTML(`<b>John</b>`))  //-> Hello, <b>John</b>!
		if err != nil {
			log.Fatal(err)
		}
	}


	// Template features
	{
		const templateString = `
			<html>
			<head>
				<title>{{.Title}}</title>
			</head>
			<body>

			{{range .Items}}
				<div>{{.}}</div>
			{{else}}
				<p>No items
			{{end}}
		`

		// Prepare the template
		t, err := template.New("page").Parse(templateString)
		if err != nil {
			log.Fatal(err)
		}

		// Prepare the data
		data := struct {
			Title string
			Items []string
		}{
			Title: "My page",
			Items: []string{
				"My Photos",
				"My Blog",
			},
		}

		// Render
		err = t.Execute(os.Stdout, data)
		if err != nil {
			log.Fatal(err)
		}
	}
}

```



# go/03-stdlib/reflect.go

```go
package main

import (
	"fmt"
	"reflect"
)

func PlayReflect(){
	// Inspect value type.
	var v any = "hey"
	value := reflect.ValueOf(v)  // get `Value`

	if value.Kind() == reflect.String {
		fmt.Printf("Is string: %s\n", value.String())
	}

	// Deep equal
	var a = []int{1, 2, 3}
	var b = []int{1, 2, 3}
	fmt.Printf("Deeply equal: %t", reflect.DeepEqual(a, b))

	// MapIter: iterator for ranging over a map
	var m = map[string]int{"a": 1, "b": 2, "c": 3}
	iter := reflect.ValueOf(m).MapRange()
	for iter.Next() {
		k := iter.Key()
		v := iter.Value()
		fmt.Printf("Key [%q]=%d\n", k.String(), v.Int())	
	}

	// Inspect a struct
	v = struct{
		Name string
		Age int64
	}{"John", 32}
	typ := reflect.ValueOf(v).Type()
	for i:=0; i<typ.NumField(); i++ {
		field := typ.Field(i)
		fmt.Printf("Field %s: %s\n", field.Name, field.Type.Kind())
	}
}
```



# go/03-stdlib/os.go

```go
package main

import (
	"fmt"
	"io"
	"io/fs"
	"log"
	"os"
	"os/exec"
	"strings"
)

func PlayOS(){
	// Open a file and read it
	file, err := os.Open("os.go")
	if err != nil {
		log.Fatal(err)
	}

	buf := make([]byte, 100)
	count, err := file.Read(buf)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Read %d bytes: %q\n", count, buf)

	// Read file, quick
	contents, err := os.ReadFile("os.go")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Read file: %q\n", contents[:20])

	// Write file, quick
	err = os.WriteFile("/tmp/example.txt", []byte("example"), 0644)
	if err != nil {
		log.Fatal(err)
	}
	defer os.Remove("/tmp/example.txt")

	// Write to a temporary file
	f, err := os.CreateTemp("", "example.*.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer os.Remove(f.Name())

	if _, err := f.Write([]byte("content")); err != nil {
		f.Close()
		log.Fatal(err)
	}

	// User cache directory
	cachedir, err := os.UserCacheDir()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Cache dir: %s\n", cachedir)

	// Command-line args
	fmt.Println(os.Args)
	executable, err := os.Executable()
	fmt.Printf("Executable: argv[0]=%s\n", executable)


	// Environment
	fmt.Printf("SHELL=%s\n", os.Getenv("SHELL"))
	fmt.Println(os.ExpandEnv("SHELL=$SHELL"))
	value, ok := os.LookupEnv("USER")
	if ok {
		fmt.Printf("$USER=%s\n", value)
	}

	// List dir
	files, err := os.ReadDir(".")
	if err != nil {
		log.Fatal(err)
	}
	for _, file := range files {
		fmt.Printf("File: %s\n", file.Name())
	}
}


func PlayOsExec(){
	// Execute command with arguments
	cmd := exec.Command("hostname")
	if cmd.Err != nil {
		log.Fatal(cmd.Err)
	}
	output, err := cmd.CombinedOutput()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Exec: %s\n", output)
}


func PlayIO(){
	// Copy(): copy from one stream to another
	r := strings.NewReader("data to be read\n")
	if _, err := io.Copy(os.Stdout, r); err != nil {
		log.Fatal(err)
	}
}

func PlayIOFS(){
	rootFs := os.DirFS("/")

	fs.WalkDir(rootFs, "mnt", func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Walk file: %s\n", path)
		return nil
	})
}
```



# go/03-stdlib/regexp.go

```go
package main

import (
	"fmt"
	"regexp"
)

func PlayRegexp(){
	// Hyperscan and re2 are 24x to 40x faster than the default Go regexp library and use 7x less memory

	// Methods: Find(All)?(String)?(Submatch)?(Index)?
	// All: Returns a slice of matches. The `n` parameter sets the max
	// String: The argument is a string. Otherwise, it's a slice of bytes.
	// Submatch: Returns a slice of matches for capturing group N
	// Index: Returns [start, end] indexes, not string matches

	var validID = regexp.MustCompile(`^\w+\[\d+\]$`)
	matched := validID.MatchString("adam[23]")
	fmt.Printf("Regexp matched: %t\n", matched)

	// Expand()
	content := "a: b\nc: d"
	search := regexp.MustCompile(`(?m)(?P<key>\w+):\s+(?P<value>\w+)$`)
	replace := "$key=$value\n"
	result := []byte{}

	// For each match of the regex in the content.
	for _, submatches := range search.FindAllStringSubmatchIndex(content, -1) {
		// Apply the captured submatches to the template and append the output
		// to the result.
		result = search.ExpandString(result, replace, content, submatches)
	}
	fmt.Println(string(result))
}
```



# go/03-stdlib/runtime.go

```go
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
```



# go/03-stdlib/sort.go

```go
package main

import (
	"fmt"
	"sort"
	"strings"
)


func PlaySort(){
	// Float64s(): Sorts a slice of float64
	list_of_floats := []float64{5.2, -1.3, 0.7, -3.8, 2.6} // unsorted
	sort.Float64s(list_of_floats)

	// Ints(): sort a list of ints
	list_of_ints := []int{5, 2, 6, 3, 1, 4} // unsorted
	sort.Ints(list_of_ints)

	// Strings(): sort a list of strings
	list_of_strings := []string{"a", "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z"}
	sort.Strings(list_of_strings)


	// Slice(): sort with the provided Less function
	// SliceStable() keeps equal elements in their original order
	sort.Slice(list_of_floats, func(i, j int) bool{ return list_of_floats[i] < list_of_floats[j] })
	//sorted := sort.SliceIsSorted(...)


	// Sort() sorts data as determined by the Less method
	// Stable() sorts data, keeping the original order of equal elements
	people := []PersonWithAge{
		{"Bob", 31},
		{"John", 42},
		{"Michael", 17},
		{"Jenny", 26},
	}
	sort.Sort(ByAge(people)) // cast to a sortable interface

	// Check is sorted
	sort.Float64sAreSorted(list_of_floats)
	sort.IntsAreSorted(list_of_ints)
	sort.StringsAreSorted(list_of_strings)
	sort.IsSorted(ByAge([]PersonWithAge{}))  // check sort.Interface


	// Find(): binary search to find the smallest [i] at which cmp(i)<=0.	
	// Requires: cmp(i) starts with >0, hits ==0, then remains <0 until the end of the list
	target := "k"
	i, found := sort.Find(len(list_of_strings), func(i int) int { return strings.Compare(target, list_of_strings[i]) })
	fmt.Printf("Found [%d]%q (%t)\n", i, list_of_strings[i], found)

	// Search(): binary search to find the smallest [i] at which f(i)==true
	// Requires: f(i) starts as false, then switches to true, and remains so until the end of the list
	i = sort.Search(len(list_of_strings), func(i int) bool { return list_of_strings[i] == "k" })
	if i < len(list_of_strings) {
		fmt.Printf("Found [%d]%q\n", i, list_of_strings[i])
	} else {
		fmt.Printf("Not found\n")
	}

	// Search*(): return the index to insert <value> if it is not present.
	i = sort.SearchInts(list_of_ints, 4)
	i = sort.SearchFloat64s(list_of_floats, 0.0)
	i = sort.SearchStrings(list_of_strings, "k")
	fmt.Printf("Found: [%d]%q\n", i, list_of_strings[i])
}


func PlayCustomSort() {
	people := []PersonWithAge{
		{"Bob", 31},
		{"John", 42},
		{"Michael", 17},
		{"Jenny", 26},
	}

	// Sort people by Age: use the implementation of the sort.Interface
	sort.Sort(ByAge(people))
	
	// Sort people by Age: use a custom Less function
	sort.Slice(people, func(i, j int) bool {
		return people[i].Age > people[j].Age
	})

	// (!) Programmable sort keys
	sorters := map[string]lessFunc{
		"name": func(p1, p2 *PersonWithAge) bool {
			return p1.Name < p2.Name
		},
		"age": func(p1, p2 *PersonWithAge) bool {
			return p1.Age < p2.Age 
		},
	}
	By(sorters["age"]).Sort(people)

	// (!) Programmable multi-sort keys
	OrderedBy(sorters["name"], sorters["age"]).Sort(people)
	

	// (!) Sort wrapper
	// Struct wraps the list.
	sort.Sort(ByName{people})
	

	// Done
	fmt.Println(people)
}



type PersonWithAge struct {
	Name string
	Age  int
}

// === Sorter by Age
// Implement sort.Interface 
type ByAge []PersonWithAge
func (a ByAge) Len() int           { return len(a) }
func (a ByAge) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByAge) Less(i, j int) bool { return a[i].Age < a[j].Age }

//===  Programmable Less() function
type By func(p1, p2 *PersonWithAge) bool
func (by By) Sort(people []PersonWithAge){
	sort.Sort(&peopleSorter{people: people, by: by})
}
// Implement sort.Interface
type peopleSorter struct {
	people []PersonWithAge
	by By
}
func (ps *peopleSorter) Len() int { return len(ps.people) }
func (ps *peopleSorter) Swap(i, j int) { ps.people[i], ps.people[j] = ps.people[j], ps.people[i] }
func (ps *peopleSorter) Less(i, j int) bool { return ps.by(&ps.people[i], &ps.people[j]) }


// === Programmable Less() func with multiple sort fields
type lessFunc func(p1, p2 *PersonWithAge) bool
func OrderedBy(less ...lessFunc) *multiSorter {
	return &multiSorter{less: less}
}

type multiSorter struct {
	less []lessFunc
	people []PersonWithAge
}
func (ms *multiSorter) Sort(people []PersonWithAge){
	ms.people = people
	sort.Sort(ms)
}

// Implement sort.Interface
func (ms *multiSorter) Len() int { return len(ms.people) }
func (ms *multiSorter) Swap(i, j int) { ms.people[i], ms.people[j] = ms.people[j], ms.people[i] }
func (ms *multiSorter) Less(i, j int) bool {
	p1, p2 := &ms.people[i], &ms.people[j]

	// Run every function in turn. If it definitely says "less", then return.
	// If it says ">=" -- proceed to the next function.
	// When all comparisons have been tried, return whatever the last comparison returns
	for _, less := range ms.less {
		switch {
		case less(p1, p2): return true
		case less(p2, p1): return false
		}
	}
	
	// If all comparisons have been tried, then return whatever the last comparison returns.
	// It should be a false
	return false	
}


// === Alternative: Sort wrapper
// Common methods to all people
type People []PersonWithAge
func (s People) Len() int      { return len(s) }
func (s People) Swap(i, j int) { s[i], s[j] = s[j], s[i] }

// Special sorter by name
type ByName struct{People}
func (s ByName) Less(i, j int) bool { return s.People[i].Name < s.People[j].Name }

```



# go/03-stdlib/sync.go

```go
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
```



# go/03-stdlib/time.go

```go
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
		log.Fatalf("Failed to parse the time: %v", err)
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

```



# go/03-stdlib/main_test.go

```go
package main_test

import (
	"fmt"
	"bytes"
	"encoding/hex"
	"testing"
	"math/rand"
)

// Test*() function
func TestXxx(t *testing.T){
	// Register a cleanup function
	t.Cleanup(func(){
		// ... clean-up when the test completes
	})
	
	// Example test
	value := 1+1
	t.Logf("Result: %d", value) // records the text in the error log

 	if value != 2 {
		// Errorf(): Log() + Fail()
		t.Errorf("Something's wrong with math in this universe: 1+1=%d", value)
		t.Fail()  // Marks it as failed, but continues execution

		t.FailNow()  // Marks it as failed and stops execution
		t.Fatal("reason")  // = Log() + FailNow()
	}

	// Ends: FailNow(), Fatal(), Fatalf(), SkipNow(), Skip(), Skipf()
	// These methods should only be called from the current goroutine

	// Helpers
	t.Setenv("NAME", "value")  // set env variable temporarily
	t.TempDir()  // a temporary directory for the test to use. It will be removed.
}

// Markers
func TestMarkers(t *testing.T){
	t.Helper()  // mark the calling function as a test helper function: is skipped in tracebacks
	t.Parallel()  // signals that the test is to be run with (and only with!) other parallel tests
}

// Subtests
func TestGroup(t *testing.T){
	// .. common setup code

	// Run(): runs f() in a separate goroutine and blocks.
	// If f() calls Parallel() and becomes a parallel test, does not block.
	t.Run("A=1", func(t *testing.T){})
	t.Run("A=2", func(t *testing.T){})

	// .. common teardown code
}

// Main function: runs in the main goroutine and sets up whatever's necessary
func TestMain(m *testing.M){
	// .. set up

	m.Run()

	// .. tear down
}


// Benchmarks are run with $ go test -bench .
func BenchmarkXxx(b *testing.B){
	// Do expensive set up
	// Register a cleanup function
	b.Cleanup(func(){
		// ... clean-up when the test completes
	})

	// Start measuring from this point
	b.ResetTimer()

	for i:=0; i<b.N; i++ {
		rand.Int()
	}

	// Custom additional metric
	b.ReportMetric(127, "requests")

}

// It will also run Example*() functions
func Example_printHello() {
    fmt.Println("hello")
    // Output: hello
}

// Fuzz tests: test with randomly generated inputs
func FuzzHex(f *testing.F){
	// Seed
	f.Add([]byte{9})
	f.Add([]byte{1, 2, 3})

	f.Fuzz(func(t *testing.T, in []byte){
		enc := hex.EncodeToString(in)
		out, err := hex.DecodeString(enc)
		
		if err != nil {
			t.Fatalf("%v: decode: %v", in, err)
		}
		
		if !bytes.Equal(in, out) {
			t.Fatalf("%v: not equal to %v", in, out)
		}
	})
}

// Test flags:
func TestTimeConsuming(t *testing.T) {
	// Skip tests that are time consuming
	// Use: $go test -test.short
    if testing.Short() {
        t.Skip("skipping test in short mode.")
    }

	// Verbose output?
	// Use: $go test -test.v
	if testing.Verbose(){
		fmt.Printf("Print something else\n")
	}
}


```



# go/03-stdlib/net_ip.go

```go
package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"net/netip"
	"time"
)

func PlayNet() {
	loadWebpage()
	tcpServer()
	netFuncs()
}

// TCP client: Dial(), load webpage
func loadWebpage(){
	// NOTE: for Name Resolution, Go resolver is used: because a blocked DNS request consumes only a goroutine.
	// Cgo mode calls C library and performs a syscall. A blocked C call consumes an OS thread.
	// In some cases (OS X, $LOCALDOMAIN, $RES_OPTIONS, $HOSTALIASES, $ASR_CONFIG, /etc/resolv.conf) Go uses Cgo.
	
	// Connect
	conn, err := net.Dial("tcp", "example.com:80")
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	// Send
	n, err := fmt.Fprintf(conn, "GET / HTTP/1.0\r\n\r\n")
	if err != nil {
		log.Fatalf("Failed to send data: %v", err)
	} else {
		log.Printf("Sent %d bytes", n)
	}

	// Receive
	reader := bufio.NewReader(conn)
	line, err := reader.ReadString('\n')
	if err != nil {
		log.Fatalf("Failed to read from socket")
	} else {
		log.Printf("Received: %v", line)
	}
}

// TCP server: Listen(), Dial()
func tcpServer(){
	// Basic interface: Dial(), Listen(), Accept()
	// Access to low-level interface: most likely, not necessary

	// Dial() connects to a server
	// DialTimeout() connects with a duration
	time.AfterFunc(100 * time.Millisecond, func(){
		// Connect
		conn, err := net.DialTimeout("tcp", ":9876", 10*time.Second)
		if err != nil {
			log.Fatalf("Failed to connect: %v", err)
		}

		// Send
		n, err := fmt.Fprintf(conn, "hello\n")
		if err != nil {
			log.Fatalf("Failed to send to the server: %v", err)
		} else {
			log.Printf("Sent %d bytes to the server", n)
		}

		// Receive
		response, err := bufio.NewReader(conn).ReadString('\n')
		if err != nil {
			log.Fatalf("Failed to read from the server: %v", err)
		}
		log.Printf("Received from the server: %v", response)
	})

	// Listen() creates a server
	ln, err := net.Listen("tcp", ":9876")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	done := make(chan int)  // use this channel to sync exising the function after exactly 1 client was handled.
	for {
		// Accept a connection
		log.Printf("Waiting for connections...")
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("Failed to accept a connection: %v", err)
			continue
		}

		// Handle connection
		go func(conn net.Conn){
			log.Printf("Connected client: %v", conn.RemoteAddr())  // Connected client: 127.0.0.1:45172

			// Read input
			rbuffer := make([]byte, 128)
			n, err := conn.Read(rbuffer)
			if err != nil {
				log.Printf("Client read error: %v", err)
				conn.Close()
			}
			log.Printf("Read %d bytes: %v (len=%d, cap=%d)", n, rbuffer, len(rbuffer), cap(rbuffer))
			
			// Respond
			wbuffer := make([]byte, 0, 128)
			wbuffer = append(wbuffer, "hello\n"...)

			n, err = conn.Write(wbuffer)
			if err != nil {
				log.Printf("Client write error: %v", err)
			} else {
				log.Printf("Wrote: %d bytes", n)
			}
			conn.Close()

			close(done)
		}(conn)

		break
	}

	<-done
}

// "net" functions
func netFuncs(){
	// Join "host:port" or "[host]:port" (for IPv6 addresses that contain a ":")
	hostport := net.JoinHostPort("localhost", "80")
	fmt.Printf("hostport: %s\n", hostport)

	// Split "host:port"
	host, port, err := net.SplitHostPort(hostport)
	fmt.Printf("Host: %s, Port: %s, Err: %v\n", host, port, err)

	// IPv4: Parse, build
	var ip net.IP
	ip = net.ParseIP("127.0.0.1")
	ip = net.IPv4(127, 0, 0, 1)
	fmt.Printf("IPv4=%v\n", ip)

	// netip: an IP that takes less memory, is immutable, and is comparable (supports == and being a map key)
	var nip netip.Addr  // IPv4 or IPv6
	nip, ok := netip.AddrFromSlice(ip)
	if !ok {
		log.Fatalf("Failed to parse IP: %v", ip)
	}
	fmt.Printf("IP: %v\n", nip)

	// netip.AddrPort: IP + port, efficient
	var ipport netip.AddrPort = netip.MustParseAddrPort("127.0.0.1:80")
	fmt.Printf("IP:port: %v\n", ipport)

	// Pipe() (Conn, Conn) creates a synchronous, in-memory, full duplex network connection.
	// Reads on one end are matched with writes on the other, copying data directly between the two. There is no internal buffering.
	recv, send := net.Pipe()
	// Send
	go send.Write([]byte("hello\n"))  // or use bufio.NewWriter(send).WriteString("hello\n")
	// Read, with timeout
	recv.SetDeadline(time.Now().Add(time.Second)) // timeout
	data, err := bufio.NewReader(recv).ReadString('\n')
	if err != nil {
		log.Fatalf("Pipe() read failed: %v", err)
	}
	fmt.Printf("Pipe() received: %q\n", data)
}
```



# go/03-stdlib/net_http.go

```go
package main

import (
	"fmt"
	"html"
	"io"
	"log"
	"net/http"
	_ "net/http/pprof"
	"time"
)

func PlayHttp() {
	httpFunc()
	httpClient()
	httpServer()


	// Example HTTP server:
	// * https://github.com/enricofoltran/simple-go-server/blob/master/main.go
	// * https://gist.github.com/creack/4c00ee404f2d7bd5983382cc93af5147
}


// "net/http" funcs
func httpFunc(){
	// CanonicalHeaderKey(): Header name: title case
	headerName := http.CanonicalHeaderKey("accept-encoding")
	fmt.Printf("Header name: %s\n", headerName)  //-> "Accept-Encoding"

	// DetectContentType(): Detect MIME Content-Type of the []byte data
	contentType := http.DetectContentType([]byte("JFIF"))
	fmt.Printf("Content-type guess: %s\n", contentType)  //-> Content-type guess: text/plain; charset=utf-8

	// ParseHTTPVersion()
	major, minor, ok := http.ParseHTTPVersion("HTTP/1.0")
	fmt.Printf("HTTP version: %d.%d (parsed ok: %t)\n", major, minor, ok)


}


// HTTP client
func httpClient(){
	// Simple client
	resp, err := http.Get("http://example.com/")
	if err != nil {
		log.Fatalf("HTTP request failed: %v", err)
	} 
	defer resp.Body.Close()

	
	responseBody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Failed to read the body: %v", err)
	}
	fmt.Printf("HTTP Client Response: %q\n", &responseBody)

	if resp.StatusCode >= 300 {
		log.Fatalf("HTTP Response failed with code: %d; body: %q", resp.StatusCode, responseBody)
	}

	// Full client: complete control
	// Clients are safe for concurrent use
	client := &http.Client{
		Transport: &http.Transport{
			IdleConnTimeout: 10 * time.Second,
			DisableCompression: true,
		},
	}
	_, err = client.Get("http://example.com/")
	if err != nil {
		log.Fatalf("HTTP request failed: %v", err)
	}
}

// HTTP server
func httpServer(){
	// Server
	// We use a custom server. Alternatively, use http.ListenAndServe()
	server := &http.Server{
		Addr: ":8080",
	}
	go func(){
		log.Fatal(server.ListenAndServe())
	}()
	
	// Add a route to the default Handle func (DefaultMux)
	http.HandleFunc("/index", func(w http.ResponseWriter, r *http.Request){
		// Respond ok
		fmt.Fprintf(w, "Hello at %q", html.EscapeString(r.URL.Path))
	})
	
	// Serve files
	http.Handle(
		"/tmp/", 
		// Strip prefix: removes the prefix so that `FileServer()` can find the file
		http.StripPrefix(
			"/tmp/", 
			http.FileServer(http.Dir("/tmp")),
		),
	)

	// pprof
	// imported

	// Error
	http.HandleFunc("/error", func(w http.ResponseWriter, r *http.Request){
		// Respond with a plain text error
		http.Error(w, "Not found", http.StatusNotFound)
	})

	// 404
	http.HandleFunc("/404/a", func(w http.ResponseWriter, r *http.Request){
		// Respond with an HTTP 404 Not found error
		http.NotFound(w, r)
	})

	http.Handle("/404/b", http.NotFoundHandler())

	// Redirect
	http.Handle("/go/to/url", http.RedirectHandler("https://google.com/", http.StatusSeeOther))
	
	// Client. Make a request, then quit
	resp, err := http.Get("http://localhost:8080/index")
	if err != nil {
		log.Fatalf("HTTP request failed: %v", err)
	}
	defer resp.Body.Close()

	responseBody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("HTTP client failed to read body: %v", responseBody)
	}
	fmt.Printf("HTTP handler returned: %s\n", responseBody)  // HTTP handler returned: Hello at "/index"
}

```



# go/03-stdlib/net_http_test.go

```go
package main_test

import (
	"net/http/httptest"
	"log"
	"io"
	"testing"
	"net/http"
	"fmt"
)

func TestHttpServer(t *testing.T) {
	// Server
	ts := httptest.NewServer(http.HandlerFunc(
		func(w http.ResponseWriter, r *http.Request){
			fmt.Fprintf(w, "Hello")
		},
	))
	defer ts.Close()

	// Client 
	client := ts.Client()
	res, err := client.Get(ts.URL)
	if err != nil {
		t.Errorf("Failed: %v", err)
	}

	// Or just use the server URL
	res, err = http.Get(ts.URL)
	if err != nil {
		log.Fatal(err)
	}
	greeting, err := io.ReadAll(res.Body)
	res.Body.Close()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Greeting: %q", greeting)
}


```



# go/03-stdlib/net_rpc.go

```go
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
```





# go/04-database


# go/04-database/go.mod

```
module goplay/database/sql

go 1.19

require (
	github.com/Masterminds/squirrel v1.5.3
	github.com/doug-martin/goqu v5.0.0+incompatible
	github.com/jackc/pgx/v5 v5.1.1
	github.com/jmoiron/sqlx v1.3.5
)

require (
	ariga.io/atlas v0.7.3-0.20221011160332-3ca609863edd // indirect
	entgo.io/ent v0.11.4 // indirect
	github.com/Masterminds/goutils v1.1.1 // indirect
	github.com/Masterminds/semver/v3 v3.1.1 // indirect
	github.com/Masterminds/sprig/v3 v3.2.2 // indirect
	github.com/agext/levenshtein v1.2.1 // indirect
	github.com/apparentlymart/go-textseg/v13 v13.0.0 // indirect
	github.com/c2fo/testify v0.0.0-20150827203832-fba96363964a // indirect
	github.com/doug-martin/goqu/v9 v9.18.0 // indirect
	github.com/friendsofgo/errors v0.9.2 // indirect
	github.com/fsnotify/fsnotify v1.5.1 // indirect
	github.com/go-openapi/inflect v0.19.0 // indirect
	github.com/google/go-cmp v0.5.6 // indirect
	github.com/google/uuid v1.3.0 // indirect
	github.com/hashicorp/hcl v1.0.0 // indirect
	github.com/hashicorp/hcl/v2 v2.13.0 // indirect
	github.com/huandu/xstrings v1.3.1 // indirect
	github.com/imdario/mergo v0.3.11 // indirect
	github.com/inconshreveable/mousetrap v1.0.1 // indirect
	github.com/jackc/pgpassfile v1.0.0 // indirect
	github.com/jackc/pgservicefile v0.0.0-20200714003250-2b9c44734f2b // indirect
	github.com/jackc/puddle/v2 v2.1.2 // indirect
	github.com/lann/builder v0.0.0-20180802200727-47ae307949d0 // indirect
	github.com/lann/ps v0.0.0-20150810152359-62de8c46ede0 // indirect
	github.com/lib/pq v1.10.7 // indirect
	github.com/magiconair/properties v1.8.5 // indirect
	github.com/mattn/go-runewidth v0.0.9 // indirect
	github.com/mattn/go-sqlite3 v1.14.16 // indirect
	github.com/mitchellh/copystructure v1.0.0 // indirect
	github.com/mitchellh/go-wordwrap v0.0.0-20150314170334-ad45545899c7 // indirect
	github.com/mitchellh/mapstructure v1.5.0 // indirect
	github.com/mitchellh/reflectwalk v1.0.0 // indirect
	github.com/olekukonko/tablewriter v0.0.5 // indirect
	github.com/pelletier/go-toml v1.9.4 // indirect
	github.com/shopspring/decimal v1.2.0 // indirect
	github.com/spf13/afero v1.6.0 // indirect
	github.com/spf13/cast v1.4.1 // indirect
	github.com/spf13/cobra v1.6.1 // indirect
	github.com/spf13/jwalterweatherman v1.1.0 // indirect
	github.com/spf13/pflag v1.0.5 // indirect
	github.com/spf13/viper v1.9.0 // indirect
	github.com/subosito/gotenv v1.2.0 // indirect
	github.com/volatiletech/inflect v0.0.1 // indirect
	github.com/volatiletech/sqlboiler/v4 v4.13.0 // indirect
	github.com/volatiletech/strmangle v0.0.4 // indirect
	github.com/zclconf/go-cty v1.8.0 // indirect
	go.uber.org/atomic v1.10.0 // indirect
	golang.org/x/crypto v0.0.0-20220829220503-c86fa9a7ed90 // indirect
	golang.org/x/mod v0.6.0-dev.0.20220419223038-86c51ed26bb4 // indirect
	golang.org/x/sync v0.0.0-20220923202941-7f9b1623fab7 // indirect
	golang.org/x/sys v0.0.0-20220722155257-8c9f86f7a55f // indirect
	golang.org/x/text v0.3.8 // indirect
	golang.org/x/tools v0.1.13-0.20220804200503-81c7dc4e4efa // indirect
	golang.org/x/xerrors v0.0.0-20200804184101-5ec99f83aff1 // indirect
	gopkg.in/DATA-DOG/go-sqlmock.v1 v1.3.0 // indirect
	gopkg.in/doug-martin/goqu.v5 v5.0.0 // indirect
	gopkg.in/ini.v1 v1.63.2 // indirect
	gopkg.in/yaml.v2 v2.4.0 // indirect
)

```



# go/04-database/main.go

```go
package main

import (
	"fmt"
	"log"
)

func main() {
	var playgrounds = []SimplePlayFunc{
		{"database/sql", PlayDatabaseSqlPostgres},
		{"pgx", PlayPgx},
		{"sqlx", PlaySqlx},
		{"orm", PlayOrm},
	}

	for _, playfunc := range playgrounds {
		fmt.Printf("==========[ %s ]==========\n", playfunc.Name)
		err := playfunc.Func()
		if err != nil {
			log.Fatalf("%s got itself killed: %v", playfunc.Name, err)
		}
	}
}

type SimplePlayFunc struct {
	Name string
	Func func() error
} 

```



# go/04-database/database_sql_postgres.go

```go
// stdlib: database/sql

package main

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"time"

	_ "github.com/jackc/pgx/v5"
	_ "github.com/jackc/pgx/v5/stdlib" // register for database/sql
)

// database/sql + pgx
func PlayDatabaseSqlPostgres() error{
	// Set up the pool
	db, err := sql.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Verify that a connection can be made
	// Use a context to ensure timeout 1 second
	timeoutCtx, _ := context.WithTimeout(ctx, 1 * time.Second)
	if err := db.PingContext(timeoutCtx); err != nil {
		return err
	}
	
	// BEGIN transaction
	// NOTE: If the context is canceled, the sql package will roll back the transaction. 
	tx, err := db.BeginTx(ctx, nil)  // default isolation level depends on the driver
	defer tx.Rollback()
	if err != nil {
		return err
	}

	// Exec(): for queries when no rows are returned
	{
		// CREATE 
		_, err = tx.ExecContext(ctx, `
			CREATE TABLE users (
				id SERIAL PRIMARY KEY,
				name varchar NOT NULL,
				age int NULL
			);
		`)
		if err != nil {
			return err
		}
	}

	// QueryRow(): retrieve one result
	{
		// INSERT
		var id int 
		err := tx.QueryRowContext(ctx,
			"INSERT INTO users (name, age) VALUES($1, $2) RETURNING id",
			"kolypto", 35,
		).Scan(&id)

		// Handle: 0 results, 1 result, error
		switch {
		case errors.Is(err, sql.ErrNoRows):
			// This can't happen with INSERT, but may happen with other queries
			fmt.Print("Nothing inserted\n") 
		case err != nil:
			return err
		default:
			fmt.Printf("User id: %d\n",	id)
		}
	}

	// Query(): retrieve many results
	{
		// SELECT
		rows, err := tx.QueryContext(ctx, `SELECT name, age FROM users;`)
		if err != nil {
			return err
		}
		defer rows.Close()
		
		// Fetch
		for rows.Next() {
			var (
				name string
				age sql.NullInt64  // NOTE: nullable type!
			)

			// Scan a row.
			// * you can pass a pointer, but be careful: it requires extra memory allocations and will degrade performance!
			// * Scan() converts between string and numeric types, as long as no information is lost.
			// * Implement a Scanner interface to support a custom type
			// * Pass a `*[]byte` => Scan() will save a copy of the corresponding data. Use `*RawBytes` to avoid copying.
			// * Pass an `*any` => Scan() will copy without conversion
			// * Time may be scanned into *time.Time, *any, *string and *byte[] -- using time.RFC3339Nano
			// * Pass a `*bool` => Scan() will convert true, false, 1, 0, or string inputs parseable by `strconv.ParseBool`
			// * Scan can convert a cursor into a *Rows: "SELECT cursor(SELECT * FROM mytable) FROM dual"
			// * 
			if err := rows.Scan(&name, &age); err != nil {
				return err 
			}
			fmt.Printf("Person: name=%s age=%d\n", name, age.Int64)
		}

		// Iteration errors
		if err := rows.Err(); err != nil {
			return err
		}
	}

	// Stats
	fmt.Printf("Pool.OpenConnections: %v\n", db.Stats().OpenConnections)
	return nil
}
```



# go/04-database/pgx.go

```go
// pgx: Postgres client

package main

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

func PlayPgx() error {
	// Context
	ctx, done := context.WithCancel(context.Background())
	defer done()

	// Pool
	// To disable prepared statements: ?default_query_exec_mode=simple_protocol
	db, err := pgxpool.New(ctx, "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}
	defer db.Close()

	// Transaction
	// Use Begin() on it to use SAVEPOINTs
	tx, err := db.BeginTx(ctx, pgx.TxOptions{})
	if err != nil {
		return err
	}

	// Use: BeginFunc() to exec a function in a transaction
	pgx.BeginFunc(ctx, db, func(tx pgx.Tx) error {
		// will COMMIT when done
		return nil
	})

	// Rollback() is safe to call even if the transaction is closed
	defer tx.Rollback(ctx)

	// Exec(): create schema
	{
		_, err := tx.Exec(ctx, usersSchema)
		if err != nil {
			return err
		}
	}

	// Insert some rows
	{
		var uid int 
		err := tx.QueryRow(ctx, 
			`INSERT INTO users (name, age) VALUES(@name, @age) RETURNING id;`,
			pgx.NamedArgs{
				"name": "kolypto",
				"age": 35,
			},
		).Scan(&uid)
		if err != nil {
			return err
		}
		fmt.Printf("INSERT id=%d\n", uid)
	}

	// CollectRow(): returns a value from the first row
	// CollectRows(): returns a slice
	{
		// SELECT
		rows, err := tx.Query(ctx, `SELECT name from users;`)
		if err != nil {
			return err
		}	

		// CollectRows() -> slice
		names, err := pgx.CollectRows(rows, pgx.RowTo[string])  // Generic, func() returns string
		if err != nil {
			return err
		}

		fmt.Printf("Names: %v\n", names)
	}
	
	// ForEachRow(): invoke callback on every row
	{
		// SELECT
		rows, err := tx.Query(ctx, `SELECT age from users;`)
		if err != nil {
			return err
		}	
		
		// ForEachRow(): aggregate max age
		var maxAge, age int 
		_, err = pgx.ForEachRow(rows, []any{&age}, func() error {
			if age > maxAge {
				maxAge = age 
			}
			return nil 
		})

		fmt.Printf("Max age: %d\n", maxAge)
	}

	// ToRow(): scan rows into maps, structs, etc
	// See: RowToStructByName() and RowToMap()
	{
		// SELECT
		rows, err := tx.Query(ctx, `SELECT id, name, age FROM users`)
		if err != nil {
			return err 
		}

		// RowToStructByName()
		users, err := pgx.CollectRows(rows, pgx.RowToStructByName[UserRow])
		if err != nil {
			return err
		}

		fmt.Printf("User: %v\n", users)
	}

	// QueryRow()
	{
		var (
			name string 
		)
		err = tx.QueryRow(ctx, `SELECT 'hey';`).Scan(&name)
		if err != nil {
			return err 
		}

		fmt.Printf("Scan(): name=%s\n", name)
	}

	return nil
}

const usersSchema = `
	CREATE TEMPORARY TABLE users (
		id SERIAL PRIMARY KEY,
		name varchar NOT NULL,
		age int
	);
`


// Struct for pgx.RowToStructByName()
type UserRow struct {
	Id int   // `db:id`  // field name override
	Name string 
	Age sql.NullInt64  // compatible with pgx
}
```



# go/04-database/sqlx.go

```go
// sqlx: extensions to database/sql

package main

import (
	"context"
	"database/sql"
	"fmt"

	_ "github.com/jackc/pgx/v5"
	"github.com/jmoiron/sqlx" // drop-in replacement, and a superset of "database/sql"
)

func PlaySqlx() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this
	
	// MustExec() panics on error
	db.MustExec(usersSchema)
	db.MustExec(`INSERT INTO users (name, age) VALUES($1, $2)`, "kolypto", 35)

	// Prepare context
	ctx, stop := context.WithCancel(context.Background())
	defer stop()
	
	// Beginx(), BeginTxx(), MustBegin()
	tx, err := db.BeginTxx(ctx, nil)
	if err != nil {
		return err
	}

	// INSERT, with named struct and batch insert
	{
		_, error := tx.NamedExec(
			`INSERT INTO users (name, age) VALUES (:name, :age)`,
			[]UserRow{
				{Name: "John", Age: sql.NullInt64{Int64: 30, Valid: true}},
				{Name: "Jack", Age: sql.NullInt64{Int64: 31, Valid: true}},
			},
			// Can also use map:
			// map[string]any{"name": "John", "Age": 30},
		)
		if err != nil {
			return error
		}
	}

	// Queryx()
	{
		rows, err := tx.Queryx(`SELECT id, name, age FROM users`)
		if err != nil {
			return err
		}
		defer rows.Close()

		// Next(), StructScan() into struct
		var user UserRow  // scan into the same struct every time
		for rows.Next() {
			err = rows.StructScan(&user)
			if err != nil {
				return err
			}
			fmt.Printf("Userx: %v\n", user)
		}
	}
	
	// Get() to load one row: into struct, or scannable scalar
	{
		var user UserRow
		err := tx.Get(&user, `SELECT id, name, age FROM users WHERE id=$1`, 1)
		if err != nil {
			return err 
		}
		
		fmt.Printf("Get() user: %v\n", user)
	}

	// Select() to load multiple rows into a slice
	// WARNING: it will load the entire result set into memory at once!
	{
		var users []UserRow
		err := tx.Select(&users, `SELECT id, name, age FROM users`)
		if err != nil {
			return err
		}

		fmt.Printf("Select() users: %v\n", users)
	}
	
	// NamedQuery() allows the use of named parameters from maps and structs
	{
		stmt, err := tx.PrepareNamed(`SELECT id, name, age FROM users WHERE name=:name`)
		if err != nil {
			return err
		}
		
		params := map[string]any{
			"name": "kolypto",
		}
		var user UserRow
		err  = stmt.Get(&user, params)
		if err != nil {
			return err
		}

		fmt.Printf("NamedQuery() user: %v\n", user)
	}

	// In() expands slice, returning the modified query string and a new arg list that can be executed.
	// The query should use the "?" bind var.
	{
		query, args, error := sqlx.In(`SELECT * FROM users WHERE id IN ?;`, []int{1, 2, 3})
		if err != nil {
			return error
		}
		fmt.Printf("Query=%s, args=%v\n", query, args)
	}

	// Named() returns a new query with :name :name replaced with `?` `?` and actual values represented as an array
	{
		query, args, error := sqlx.Named(`INSERT INTO users (name, age) VALUES(:name, :age);`, map[string]any{"name": "me", "age": 0})
		if err != nil {
			return error 
		}
		fmt.Printf("Query=%s, args=%v\n", query, args)
	}

	
	return nil
}


```



# go/04-database/orm.go

```go
// ORM libraries

package main

import (
	"context"
	"database/sql"
	_ "embed"
	"fmt"
	"time"

	"entgo.io/ent/dialect"
	sq "github.com/Masterminds/squirrel"
	"github.com/doug-martin/goqu/v9"
	_ "github.com/doug-martin/goqu/v9/dialect/postgres"
	"github.com/jmoiron/sqlx"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"

	"goplay/database/sql/ent/ent"
	"goplay/database/sql/ent/ent/car"
	"goplay/database/sql/ent/ent/user"
	"goplay/database/sql/sqlboiler/models"
	"goplay/database/sql/sqlc/dbs"

	entsql "entgo.io/ent/dialect/sql"
)

func PlayOrm() error {
	var playgrounds = []SimplePlayFunc{
		// TODO: learn GORM. 
		// But people advice against it and favor:
		// * raw SQL: learn once, use everywhere. Simplicity and more control.
		// * query builders: because with raw SQL you'd resort to templating
		// * code generators: because they give you type-safe code
		// {"GORM", playGorm},

		{"Squirrel", playSquirrel},
		{"goqu", playGoqu},
		{"sqlc", playSqlc},
		{"sqlboiler", playSqlboiler},
		{"ent", playEntityFramework},
	}
	
	for _, playfunc := range playgrounds {
		fmt.Printf("### %s\n", playfunc.Name)
		err := playfunc.Func()
		if err != nil {
			return fmt.Errorf("%s failed: %s", playfunc.Name, err)
		}
	}

	return nil
}

func playGorm() error {
	return nil
}

func playSquirrel() error {
	// Cache
	// dbCache := sq.NewStmtCache(db)
	// mydb := sq.StatementBuilder.RunWith(dbCache)

	// Postgres
	// psql := sq.StatementBuilder.PlaceholderFormat(sq.Dollar)

	// SELECT, JOIN, WHERE
	users := sq.Select(`*`).From(`users`).Join(`emails USING (email_id)`)
	activeUsers := users.Where(sq.Eq{
		"deleted_at": nil,  // IS NULL
	})
	if true {
		activeUsers = activeUsers.Where("age > ?", 18)
	}

	sql, args, err := activeUsers.ToSql()
	if err != nil {
		return err
	}
	
	fmt.Printf("SQL: %s %v\n", sql, args)

	// INSERT
	sql, args, err = sq.
		Insert("users").Columns("name", "age").
		Values("moe", 13).
		Values("larry", sq.Expr("? + 5", 12)).
		Suffix(`RETURNING id`).
		ToSql()
	if err != nil {
		return err	
	}
	fmt.Printf("SQL: %s %v\n", sql, args)
	

	// Run immediately:
	// query = query.RunWith(m.db).PlaceholderFormat(sq.Dollar)
	// query.QueryRow().Scan(&node.id)

	return nil
}

func playGoqu() error {
	// We tried a few other sql builders but each was a thin wrapper around sql fragments that we found error prone. 
	// We created an expressive DSL that would find common errors with SQL at compile time
	
	// Dialect
	pg := goqu.Dialect("postgres")

	// Use it on a DB
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}
	defer db.Close()
	pgdb := pg.DB(db)

	db.MustExec(usersSchema)

	// SELECT
	query, args, err := pg.From(`test`).
		Where(goqu.Ex{
			"d": []string{"a", "b", "c"},  // WHERE d IN ('a', 'b', 'c') !
		}).
		ToSQL()
	if err != nil {
		return err
	}
	fmt.Printf("SQL: %s %v\n", query, args)

	// Count(), type-safe
	if count, err := pgdb.From("users").Count(); err != nil {
		return err
	} else {
		fmt.Printf("Count: %d\n", count)
	}

	// Clause methods:
	// Ex{}: map: identifier => value (WHERE)
	// ExOr{}: OR version 
	// S(), T(), C(): Schema, Table, Column
	// I(): Table.Column
	// L: SQL literal
	// V: Value to be used

	// Ex{}, Op{}
	{
		sql, _, _ := pgdb.From(`items`).Where(goqu.Ex{
			"a": "a",  					// a == "a'"
			"b": goqu.Op{"neq": 1}, 	// b != 1
			"c": nil,  					// c IS NULL
			"d": []int{1,2,3},  		// d IN (1,2,3)
		}).ToSQL()
		fmt.Printf("SQL: %s\n", sql)
	}
	
	// S(), T(), C()
	{
		t := goqu.T("users")
		sql, _, _ := pgdb.From(t).Select(
			t.Col("id"),  // SELECT users.id
		).Where(
			goqu.C("age").Gte(18),  // age >= 18
		).ToSQL()
		fmt.Printf("SQL: %s\n", sql)
	}

	// I()
	{
		id := goqu.I("users.id") // "table.column", or just "column"
		sql, _, _ := pgdb.From(id.GetTable()).Select(id).ToSQL()
		fmt.Printf("SQL: %s\n", sql)
	}

	// L(), V()
	{
		sql, args, _ := pgdb.From("users").Select(
			goqu.V(true).As("is_verified"),  // literal value
		).Where(
			goqu.L(`age >= ?`, 18),  // literal expr
		).ToSQL()
		fmt.Printf("SQL: %s %v\n", sql, args)
	}

	// TODO: See further: SELECT , INSERT, UPDATE, DELETE dataset, PREPAREd statements, Database, Time


	return nil
}
	
func playSqlc() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this

	// Create tables
	tx := db.MustBegin()
	defer tx.Rollback()
	tx.MustExec(sqlcSchema)

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Get queries
	queries := dbs.New(tx)

	// Create a user
	{
		createdUser, err := queries.CreateUser(ctx, dbs.CreateUserParams{
			Login: "kolypto",
			Age: sql.NullInt32{35, true},
		})
		if err != nil {
			return err
		}
		fmt.Printf("Created user id: %d\n", createdUser.ID)
	}

	// List users
	{
		users, err := queries.ListUsers(ctx)
		if err != nil {
			return err
		}
		fmt.Printf("Users: %v\n", users)
	}

	ctx.Done()
	return nil
}


func playSqlboiler() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this
	
	// Set global database for G() methods
	boil.SetDB(db)

	// Create tables
	tx := db.MustBegin()
	defer tx.Rollback()
	tx.MustExec(sqlboilerSchema)

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Users.Count()
	{
		count, err := models.Users().Count(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Count: %d\n", count)
	}

	// Users().All(), Limit()
	{
		users, err := models.Users(
			qm.Limit(5),
		).All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Users: %v\n", users[0])
	}

	// Users.DeleteAll()
	{
		n, err := models.Users(
			models.UserWhere.ID.GT(100),
		).DeleteAll(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Deleted: %d rows\n", n)
	}
	
	// NewQuery(): custom query
	{
		rows, err := models.NewQuery(qm.From(`busers`)).QueryContext(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("NewQuery(): %v\n", rows)
		rows.Close()
	}

	// Query Mods
	{
		// qm.SQL(): raw sql
		users, err := models.Users(qm.SQL(`SELECT * FROM busers WHERE id=$1`, 1)).All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("SQL(): %v\n", users)

		// qm.Select(), qm.From()
		users, err = models.Users(
			// qm.From("busers"),
			
			// Columns: by name, or by constant
			qm.Select(
				"id",
				models.UserColumns.Login,
			),
			// Where: string, or expression
			qm.Or2(qm.Expr(
				qm.Where("id > ?", 0),
				models.UserWhere.ID.GT(0),
			)),

			// Eager loading
			qm.Load(models.UserRels.AuthoredVideos),
		).All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Users(qm): %v\n", users)
	}

	// Finishers: One(), all() ; Count(), Exists() ; UpdateAll(), DeleteAll(); Exec(); Bind() , Query(), QueryRow()
	// Bind() finisher
	{
		var users []models.User  // or a custom struct
		err := queries.Raw(`SELECT * FROM busers`).Bind(ctx, tx, &users)
		if err != nil {
			return err
		}
		fmt.Printf("Raw().Bind(): %v\n", users)
	}

	// Relationships
	{
		// Get one user
		user, err := models.FindUser(ctx, tx, 1)
		if err != nil {
			return err
		}
		
		// Get related
		videos, err := user.AuthoredVideos().All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Related videos: %v\n", videos)
	}

	// Hooks
	models.AddUserHook(boil.AfterInsertHook, func(ctx context.Context, exec boil.ContextExecutor, p *models.User) error {
		return nil
	})

	ctx.Done()
	return nil
}


func playEntityFramework() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this
	
	// Create an ent.Driver from `db`
	driver := entsql.OpenDB(dialect.Postgres, db.DB)
	client := ent.NewClient(ent.Driver(driver))

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Transaction
	tx, err := client.Tx(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Run the auto-migration tool
	if err := tx.Client().Schema.Create(ctx); err != nil {
		return errors.WithMessage(err, "Migration failed")
	}

	// User.Create()
	{
		user, err := tx.User.Create().
			SetLogin("john").
			SetAge(30).
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to create a user")
		}
		fmt.Printf("User created: id=%d\n", user.ID)
	}

	// User.Query() 
	{
		user, err := tx.User.Query(). 
				Where(user.Login("john")).  // same as: 
			Only(ctx)  // Assert: exactly 1 user 
		if err != nil {
			return errors.WithMessage(err, "User not found")
		}
		fmt.Printf("User: %v\n", user)
	}
	
	// User.AddCars() relationship
	{
		tesla, err := tx.Car. 
			Create(). 
			SetModel("Tesla"). 
			SetRegisteredAt(time.Now()).
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to create a car")
		}

		ford, err := tx.Car. 
			Create(). 
			SetModel("Ford"). 
			SetRegisteredAt(time.Now()). 
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to create a car")
		}

		// AddCars()
		owner, err := tx.User. 
			Create(). 
			SetAge(30). 
			SetLogin("Owner"). 
			AddCars(tesla, ford).
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to save a user")
		}

		fmt.Printf("Created User(id=%d) with cars [Tesla(id=%d), Ford(id=%d)]\n", owner.ID, tesla.ID, ford.ID)

		// QueryCars()
		cars, err := owner.QueryCars().Where(
			car.ModelNotIn("Lada"),
		).All(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to load user cars")
		}
		fmt.Printf("QueryCars(): %v\n", cars)

		// Car.QueryOwner()
		owner, err = tesla.QueryOwner().Only(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to find owner's car")
		}

		fmt.Printf("Car %q owner: %q\n", cars[0], owner)
	}

	// Traverse graph
	{
		cars, err := tx.User.Query(). 
			Where(user.HasCars()). 
			QueryCars(). 
			All(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to find cars for user #1")
		}

		fmt.Printf("Cars for User: %v\n", cars)
	}

	ctx.Done()
	return nil
}

//go:embed sqlc/schema.sql
var sqlcSchema string

//go:embed sqlboiler/schema.sql
var sqlboilerSchema string 

```





# go/04-database/ent
# Entity Framework

> go install entgo.io/ent/cmd/ent@latest

Create schema:

```console
$ ent init user
$ vim ent/schema/user.go
$ ent generate ent/schema
or
$ go generate ./ent
```





# go/04-database/ent/ent/schema


# go/04-database/ent/ent/schema/group.go

```go
package schema

import (
	"regexp"

	"entgo.io/ent"
	"entgo.io/ent/schema/field"
)

// Group holds the schema definition for the Group entity.
type Group struct {
	ent.Schema
}

// Fields of the Group.
func (Group) Fields() []ent.Field {
	return []ent.Field{
		field.String("name").Match(regexp.MustCompile("[\\w_]+$")),
	}
}

// Edges of the Group.
func (Group) Edges() []ent.Edge {
	return nil
}

```



# go/04-database/ent/ent/schema/car.go

```go
package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
)

// Car holds the schema definition for the Car entity.
type Car struct {
	ent.Schema
}

// Fields of the Car.
func (Car) Fields() []ent.Field {
	return []ent.Field{
		field.String("model"),
		field.Time("registered_at"),
	}
}

// Edges of the Car.
func (Car) Edges() []ent.Edge {
	return []ent.Edge{
		// backref
		edge.From("owner", User.Type).Ref("cars").Unique(),  // a car has one owner
	}
}

```



# go/04-database/ent/ent/schema/user.go

```go
package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/field"
)

// User holds the schema definition for the User entity.
type User struct {
	ent.Schema
}

// Fields of the User.
func (User) Fields() []ent.Field {
	return []ent.Field {
		// All fields are required by default, unless Optional().
		// "id" is built in
		field.Int("age").Positive(),
		field.String("login").Default(""),
	}
}

// Edges of the User.
func (User) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("cars", Car.Type),
	}
}

```





# go/04-database/sqlboiler
# Sqlboiler

1. Create DB schema, give explicit names to foreign keys (used to name relationships!)
2. `sqlboiler.toml` (see Viper)
3. Initial generation: `$ sqlboiler psql`



# go/04-database/sqlboiler/sqlboiler.toml

```toml
output = "models"
wipe = true 

add-global-variants	= true
add-panic-variants = true
add-enum-types = true
add-soft-deletes = true

[psql]
# psql: host, port, user, pass, sslmode
# psql: schema, dbname
# pqsl: whitelist[], blacklist[]
# Environment: PSQL_*, e.g. "PSQL_DBNAME"
host = "localhost"
port = 5432
user = "postgres"
pass = "postgres"
dbname = "postgres"
sslmode = "disable"
blacklist = ["migrations", "*._*"]  # ignore table, columns by pattern


# Rename tables, columns
[aliases.tables.busers]
up_plural     = "Users"
up_singular   = "User"
down_plural   = "users"
down_singular = "user"

  [aliases.tables.busers.columns]
  login = "Login"

[aliases.tables.barticles]
up_plural = "Articles"
up_singular = "Article"
down_plural = "articles"
down_singular = "article"

# Configure a relationship
[aliases.tables.barticles.relationships.fk_author]  # also see alternative syntax
# Foreign: the table that the foreign key points to
foreign = "author"  # Default: Author
# Local: the table with the foreign key
local = "AuthoredVideos"  # Default: AuthorVideos


# Auto-set ctime & mtime
[auto-columns]
created = "created_at"
updated = "updated_at"


# Override types
#[[types]]
# match: type "null.String", nullablt=True
# replace: "mynull.String"

```



# go/04-database/sqlboiler/schema.sql

```sql
DROP TABLE IF EXISTS busers CASCADE;
CREATE TABLE busers (
    id bigserial,
    login varchar NOT NULL,
    PRIMARY KEY(id)
);

DROP TABLE IF EXISTS barticles CASCADE;
CREATE TABLE barticles (
    id bigserial,
    author_id bigint,
    title text,
    body text,
    PRIMARY KEY(id),
    CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES busers(id)
);

DROP TABLE IF EXISTS btags CASCADE;
CREATE TABLE btags (
    id bigserial,
    name varchar,
    PRIMARY KEY(id)
);

DROP TABLE IF EXISTS barticle_tags CASCADE;
CREATE TABLE barticle_tags (
    article_id bigint,
    tag_id bigint,
    PRIMARY KEY(article_id, tag_id),
    CONSTRAINT fk_article_id FOREIGN KEY (article_id) REFERENCES barticles(id),
    CONSTRAINT fk_tag_id FOREIGN KEY (tag_id) REFERENCES btags(id)
);

INSERT INTO busers (id, login) VALUES 
    (1, 'A'),
    (2, 'B'),
    (3, 'C')
;
INSERT INTO barticles (author_id, title) VALUES 
    (1, 'First'),
    (1, 'Second'),
    (1, 'Third'),
    (2, 'Red'),
    (2, 'Green'),
    (3, 'Blah')
;

INSERT INTO btags (id, name) VALUES
    (1, '#a'),
    (2, '#b')
;

INSERT INTO barticle_tags (article_id, tag_id) VALUES 
    (1, 1),
    (1, 2),
    (2, 1),
    (3, 2)
;
```



# go/04-database/sqlboiler/schema-create.sh

```bash
#! /usr/bin/env bash 

cat schema.sql | psql postgres://postgres:postgres@localhost:5432/postgres 

```





# go/04-database/sqlc
# Installation

```console
$ go install github.com/kyleconroy/sqlc/cmd/sqlc@latest
```

sqlc needs to know your schema and queries:

* `sqlc.yaml` is the config
* `schema.sql` is the DB schema
* `query.sql` are your queries

Every query has a name, and a command:

* `:one` returns one row: `Object`
* `:many` returns many rows: `[]Object` slice
* `:exec` returns nothing; only `err`
* `:execresult` returns `sql.Result`
* `:execrows` returns `int` number of affected rows
* `:execlastid` returns `int` last insert id 
* `:batchexec` will receive a list of objects and return a `pgx.BatchResults`
* `:batchmany` will let you process `[]Object` rows with a callback
* `:batchone` will let you process `Object` row with a callback
* `:copyfrom` to use the Copy Protocol: inserts many rows faster

Now go:

```console
$ sqlc generate
```

Notes:
* Use `sqlc.arg(name)` or `@name` to override a name (named parameter)
* Use `sqlc.narg(name)` to force non-nullable arg

sqlc parses `CREATE TABLE` and `ALTER TABLE` statements. 
sqlc parses migration files in lexicographic order. 

sqlc is able to ignore *down* migrations from: dbmate, golang-migrate, goose, sql-migrate, tern




# go/04-database/sqlc/sqlc.yaml

```yaml
version: "2"
sql:
  - engine: "postgresql"
    queries: "query.sql"  # directory of queries, or one file, or list of paths
    schema: "schema.sql"  # directory of migrations, or one file, or list of paths
    gen:
      go:
        package: "dbs"
        out: "dbs"
        
        sql_package: "database/sql"  # or "pgx/v5"
        emit_db_tags: true  # struct tags "db:"
        emit_json_tags: true 

        # Override column names if something goes wrong
        rename:
          age: "Age"
        # Custom types
        overrides:
        - db_type: "uuid"
          go_type: "github.com/gofrs/uuid.UUID"

      # Python: generate Pydantic models and SqlAlchemy core executions
      python:
        package: "dbs"
        out: "pydbs"
        emit_async_querier: true 
        emit_pydantic_models: true  # pydantic? or dataclass?

      # json:
        # out: "./"

```



# go/04-database/sqlc/query.sql

```sql
-- name: GetUser :one
SELECT * FROM users
WHERE id = $1 LIMIT 1;

-- name: ListUsers :many
SELECT * FROM users
ORDER BY login;

-- name: CreateUser :one
INSERT INTO users (login, age) VALUES(@login, @age)
RETURNING *;

-- name: DeleteUser :exec
DELETE FROM users
WHERE id = $1;

```



# go/04-database/sqlc/schema.sql

```sql
CREATE TABLE users (
    id bigserial,
    login varchar NOT NULL,
    age int,
    PRIMARY KEY(id)
);

CREATE TABLE articles (
    id bigserial,
    author_id bigint NOT NULL REFERENCES users(id),
    title varchar NOT NULL,
    body text,
    PRIMARY KEY(id)
);

```





# cockroachdb-errors


# cockroachdb-errors/cockroachdb_errors.go

```go
package main

import (
	// "errors"
	"context"
	"fmt"

	// "github.com/pkg/errors"
	"github.com/cockroachdb/errors"
)

func playCockroachdbErrors() error {
	// Construct errors.New() + error leaf constructors
	
	// Wrap with errors.Wrap() + see other wrappers
	// Test identity with errors.Is(), errors.IsAny()
	
	// Encode with errors.EncodeError() / errors.DecodeError()
	// Sentry reports: errors.BuildSentryReport() / errors.ReportError()

	// Extract PII-free safe details: errors.GetSafeDetails() (Personally Identifiable Information)
	// Extract user-facing hints and details: errors.FlattenHints(), errors.FlattenDetails()

	err := errors.New("Failed")
	fmt.Printf("Error: %+v\n", err)  // "+v" with stack trace (same with pkg/errors)

	// Error leafs
	{
		// New(), Newf(), Errorf()
		// Leaf error with message
		// Use: common error cases
		err = errors.Newf("Failed with id=%d", 1)
		fmt.Printf("Newf(): %+v\n", err)

		// AssertionFailedf(), NewAssertionFailureWithWrappedErrf(), WithAssertionFailure()
		// Signals an assertion error / programming error
		// Use: invariant is violated ; unreachable code path is reached
		err = errors.AssertionFailedf("Impossible")
		err = errors.WithAssertionFailure(err)
		fmt.Printf("AssertionFailedf(): %+v, IsAssertionFailure()=%t\n", err, errors.IsAssertionFailure(err))

		// Handled(), Opaque(), HandledWithMessage()
		// Capture an error but make it invisible to Unwrap() / Is()
		// Use: a new error occurs while handling another one, and the original error must be hidden
		err = errors.Handled(err)
		fmt.Printf("Handled(assertion): %+v IsAssertionFailure()=%t\n", err, errors.IsAssertionFailure(err))
		
		// UnimplementedError(), WithIssueLink()
		// Captures a message string and URL to a Jira issue
		// Use: inform PM user that the feature is not yet implemented
		err = errors.UnimplementedError(errors.IssueLink{IssueURL: "app.jira.com/APP-001"}, "This feature is not implemented")
		err = errors.WithIssueLink(errors.New("Not implemented"), errors.IssueLink{IssueURL: "APP-001"})
		fmt.Printf("UnimplementedError(): %+v\n", err)
	}

	// Wrappers
	{
		// Wrap()
		// Combines message, stack, and safe details
		// Use: on error return paths
		err = errors.New("!")
		err = errors.Wrap(err, "Failed")
		fmt.Printf("Wrap(): %+v\n", err)

		// CombineErrors(), WithSecondaryErrors()
		// Use: combine -- when two errors occur, and they need to pass the Is() check
		// Use: secondary --  when an additional error occurs, and it should be hidden from the Is() check
		err = errors.WithSecondaryError(errors.New("!"), err)
		fmt.Printf("WithSecondaryError(): %+v\n", err)

		// Mark()
		// Give the identity of one error to another error
		err = errors.Mark(err, myErrType)
		fmt.Printf("Is(myErrType)=%t\n", errors.Is(err, myErrType))

		// WithStack(): Annotate with stack trace. 
		// WithMessage(): Annotate with message prefix
		// Use: never. Use Wrap() instead
		err = errors.WithStack(myErrType) // Use case: when returning a sentinel

		// WithDetail()
		// WithHint()
		// User-facing details with contextual information / hint with suggestion for action to take
		// Use: Message to be presented to a human
		err = errors.New("DB failure")
		err = errors.WithDetail(err, "Cannot find the user") // negative (context)
		err = errors.WithHint(err, "Check your input") // positive (what to do)
		fmt.Printf("Detail and Hint: details=%v, hints=%v\n", errors.GetAllDetails(err), errors.GetAllHints(err))

		// WithTelemetry()
		// Annotate with a key suitable for telemetry
		err = errors.WithTelemetry(err, "ray-id:12345")
		fmt.Printf("Telemetry: %v\n", errors.GetTelemetryKeys(err))

		// WithDomain(), HandledInDomain(), HandledInDomainWithMessage()
		// Annotate with an origin package
		// Use: at package boundaries
		err = errors.WithDomain(err, "example.com")
		fmt.Printf("Not in example.com: %t\n", errors.NotInDomain(err, "example.com"))

		// WithContextTags()
		// Annotate with key/value tags attached to a context.Context -- using `logtags` package
		// Use: when context is available
		ctx := context.WithValue(context.Background(), "something", "anything")
		err = errors.WithContextTags(err, ctx)
		fmt.Printf("Context: %v\n", errors.GetContextTags(err))
	}

	// PII-free details (Personally Identifiable Information)
	{
		// * By default, many strings are considered to be PII-unsafe: they are stripped out when building a Sentry report
		// * Some fields are assumed to be PII-safe: type, stack trace, issue link, telemetry, domain, context, format strings, argument types
		// To opt additional in to Sentry reporting:
		// * implement errors.SafeDetailer: func SafeDetails() []string
		// * use errors.Safe() wrapper
		// * use errors.WithSafeDetails()
		err = errors.Newf("Failed with user id=%d", errors.Safe(1))
		fmt.Printf("Safe: %v\n", errors.GetAllSafeDetails(err))
	}

	return nil 
}


var myErrType = errors.New("My Err")


// Example custom type
type httpErrorType struct {
	code int 
	tag string 
}

// Implements: Error interface
func (e *httpErrorType) Error() string {
	return fmt.Sprintf("#%d: %s", e.code, e.tag)
}

// Implements: Formatter()
// This enables %+v recursive application
func (w *httpErrorType) Format(s fmt.State, verb rune) { 
	errors.FormatError(w, s, verb) 
}


// Implements: SafeDetailer(): mark all fields as Safe
func (e *httpErrorType) SafeDetails() []string {
	return []string{
		fmt.Sprintf("%d", e.code),
		e.tag,
	}	
}

```





# protobuf
# Protocol Buffers

## Introduction

A serialization format for packets of typed, structured data, that are up to a few megabytes in size. 
Suitable for: both network traffic and long-term data storage: 

Language support: 
* Direct support for: C, C#, Java, Kotlin, Objective-C, PHP, Python, Ruby
* Plugin support for: Dart, Go
* Other languages: third-party modules

Sample message:

```protobuf
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```

generated code:

```java
Person john = Person.newBuilder()
    .setId(1234)
    .setName("John Doe")
    .setEmail("jdoe@example.com")
    .build();
output = new FileOutputStream(args[0]);
john.writeTo(output);
```

Fields can be added and removed without breaking existing services: without invalidating existing data or requiring code to be updated!

As long as you follow some simple practices, it's going to be forward compatible: old code will read new messages without issues,
ignoring newly added fields. To the old code, fields that were deleted will have their default value, and deleted repeated fields will be empty.

## Syntax

A field has:

* A field can be: `optional`, `repeated`, `singular` (proto3).
* Then goes the type: scalar type (int, bool, float), or complex: `message` (nested structure), `enum` (set of values), `oneof` (at most one field will be set), `map` (key/value pairs)
* Field number. A field reserves a number.

Common types: `Duration` (seconds), `Timestamp` (int), `Interval` (two timestamps), `Date`, `Month`, `DayOfWeek`, `TimeOfDay`, `Money` (amount + currency), `LatLng` (lat/long pair), `PostalAddress` (string), `Color` (RGBA).

## Proto3 syntax

### Message and Fields
Example search request:

```protobuf
// "proto2" is a default. Override.
syntax = "proto3"

// Message with 3 fields
message SearchRequest {
    // Field numbers: unique numbers that identify fields. Should not be changed.
    // Use 1..15 for frequent fields (use only 1 byte)
    string query = 1;
    int32 page_number = 2;
    int32 results_per_page = 3;
}

message SearchResponse {
    /* ... */
}
```

*Field numbers* identify fields in the [Message Binary Format](https://developers.google.com/protocol-buffers/docs/encoding).

Field numbers 1..15 take one byte to encode the field number and the type. Field numbers 16..2047 use two bytes.

Hint: reserve 1..15 for ver frequently occurring message elements. Leave some room for future frequent elements.

Message fields can be one of:

* `singular`: a message can have 0 or 1 of this field.
* `optional`: same as `singular`, except that you can check to see if the value was explicitly set.
* `repeated`: this field can be repeated 0+ times. An array. The order will be preserved.
* `map`: key/value pairs

When unserialized:

* with a `singular` field, you cannot determine whether it was parsed from the wire or gotten the default value.
* with an `optional` field, you can check to see if the value was explicitly set

When serialized to the wire:

* a `singular` field is serialized to the wire unless it is the default value
* an `optional` field is serialized if it was explicitly set

When a `singular` field is present, you cannot determine whether it was parsed from the wire or gotten the default value. 
It will be serialized to the wire unless it is the default value. See more in [Field presence](https://github.com/protocolbuffers/protobuf/blob/main/docs/field_presence.md)

An `optional` field is either 1) set and contains a value that was explicitly set, or 2) the field is unset, and will return the default value.
An unset field will not be serialized to the wire.

Here's how you indicate that some field numbers have been used by deleted fields:

```protobuf
message Foo {
    reserved 2, 15, 9 to 11; // indicate field numbers that cannot be used
    reserved "foo", "bar"; // indicate field names (used when mapping to JSON serialization)
}
```

## Scalar Value Types

Floats: `float`, `double` (64)

Integers: `int32`, `int64` (inefficient for negative numbers; use `sint` instead); 
signed `sint32`, `sint64` ; Unsigned: `uint32`, `uint64` (more efficient).

Big integers: `fixed32`, `fixed64`, `sfixed32`, `sfixed64` (more efficient when values are often greater than 2^28)

NOTE: `int` types use variable-length encoding and thus are inefficient with negative numbers. Use `sint` explicitly.

NOTE: `fixed` types are more efficient when your integers are always large: they use fixed 4-byte/8-byte encoding rather than variable-length encoding.

Other: `bool`, `string` (UTF-8 or ASCII), `bytes` (up to 2^32) 

Default values are type-specific, like in Go: `string ""`, `bool false`, `int 0`, `enum (value 0)`, `message (not set)`.

## Enums

```protobuf
enum Corpus {
  // You can define aliases: assign the same value to different enum constants.
  // All aliases are valid, but the first value is used when serializing
  option allow_alias = true;  

  CORPUS_UNSPECIFIED = 0;  // the default value. Must have. Must be first.
  CORPUS_UNIVERSAL = 1;
  CORPUS_WEB = 2;
  CORPUS_IMAGES = 3;
  CORPUS_LOCAL = 4;
  CORPUS_NEWS = 5;
  CORPUS_PRODUCTS = 6;
  CORPUS_VIDEO = 7;

  reserved 8, 15, 9 to 11, 40 to max; // indicate that some values cannot be used
  reserved "FOO", "BAR"; // indicate that some names cannot be used.
}
message SearchRequest {
  string query = 1;
  int32 page_number = 2;
  int32 result_per_page = 3;

  // enum field
  Corpus corpus = 4;
}
```

NOTE: enum values use variant encoding on the wire, so negative numbers are inefficient and thus not recommended.

Deserialization of unrecognized enum values is language-dependent, but in general, these values will be preserved in the message.
With language that support open enum types with values outside the range of specified symbols (C++ and Go), the unknown enum value
is simply stored as its underlying integer representation.

## Nested Messages

Use other messages as field types:

```protobuf
message SearchResponse {
  repeated Result results = 1;
}

message Result {
  string url = 1;
  string title = 2;
  repeated string snippets = 3;
}
```

Or define nested types:

```protobuf
message SearchResponse {
  message Result {
    string url = 1;
    string title = 2;
    repeated string snippets = 3;
  }
  repeated Result results = 1;
}
```

Now the nested type should be referred to as `SearchResponse.Result`.

## Imports

```protobuf
import "myproject/other_protos.proto";
```

## Updating a Message Type

Rules to update message types without breaking any of your existing code:

* Don't change the field numbers
* Add new fields. Old binaries will simply ignore them
* Remove old fields. Reserve their field numbers, or renamed them to "obsolete_"
* `int32/64`, `uint32/64` and `bool` are all compatible: you can change the type without breaking code
* `sint32` and `sint64` are compatible with each other but not with other int types
* `string` and `bytes` are compatible as long as the bytes are valid UTF-8
* Embedded messages are compatible with `bytes` if the bytes contain an encoded version of the message
* `fixed32` is compatible with `sfixed32`, and `fixed64` with `sfixed64`
* `singular` fields are compatible with `repeated` fields for `string`, `bytes`, `message` fields
* `singular` fields are NOT compatible with `repeated` numeric fields: they can be serialized in the *packed* format and not parsed correctly
* `enum` is compatible with `int32/64`, `uint32/64` (but be aware that client code may treat the value differently)
* Changing a single `optional` field into a member of a `oneof` is safe. 
* Changing a single field `oneof` to an `optional` field is safe.
* Moving multiple fields into a new `oneof` may be safe if you are sure that no code sets more than one at a time.
* Moving any fields into an existing `oneof` is not safe. 

## Any

The `Any` message type allows to use any message type, identified by its globally unique identifier URL that resolves the message type:

```protobuf
import "google/protobuf/any.proto";

message ErrorStatus {
  string message = 1;
  repeated google.protobuf.Any details = 2;
}
```

Language implementations will support runtime helpers to pack/unpack `Any` values.

With `proto2`, it was called "extensions".

## Oneof

In cases where at most one field will be set at the same time.
This saves some memory by sharing.

You can use `case()` of `WhichOneOf()` to check which value is set.

```protobuf
message SampleMessage {
  oneof test_oneof {
    string name = 4;
    SubMessage sub_message = 9;
  }
}
```

Features: 

* Setting a `oneof` field automatically clears all other members
* If multiple values are on the wire, only the last member seen is used
* Cannot be repeated; cannot contain `map` and `repeated` fields.
* If you set a field to its default value, the "case" is set, and the default value will be serialized on the wire

Issues:

* If case=`None/NOT_SET`, it could mean that 1) the oneof ahs not been set, or 2) set to an unknown field. There's no way to tell the difference.

## Maps

Key/value map.
Key: any integral (any scalar, but float and bytes) or string type. Enum cannot be a key.
Value: any type, except another map.

```protobuf
message SampleMessage {
    map<string, Project> projects = 3;
}
```

Notes:

* Keys cannot be enums
* Values cannot be maps
* Map fields cannot be repeated
* Map is unordered
* When a duplicate key is encountered, the last value is used
* Implemented as `repeated message { key_type key = 1 ; value_type value = 2; }`

## Packages

You can specify a `package` and refer to its messages:

```protobuf
package foo.bar;
message Open { ... }

// ...

message Foo {
    foo.bar.Open open = 1;
}
```

In Go, the package is used as the Go package name, unless an explicit `go_package` is provided.

In Python, it's ignored, since Python modules are organized according to their location in the file system.

## Defining Services

Define RPC services:

```protobuf
service SearchService {
    rpc Search(SearchRequest) returns (SearchResponse);
}
```

The most straightforward system is gRPC.

## JSON Mapping

Proto3 supports JSON encoding. 

* If a value is missing in the JSON source, or is `null`, a default will be used
* `enum` -> strings
* `repeated` -> arrays
* `bytes` -> base64 string
* `int32` -> Number, `int64` -> string number
* `float` -> 1.1, `"NaN"`, `"Infinity"`, `"-Infinity"`
* `Any` -> `{"@type": "url", ...}`
* `Timestamp` -> RFC3339, Z-normalized: `"1972-01-01T10:00:20.021Z"`
* `Duration` -> string `"1.003s"`

Options:

* Emit default values: yes/no. Fields with default values are omitted by default.
* Ignore unknown fields: reject (default) / ignore
* Convert to camelCase: yes (default) / no. The parser will accept both as its input.
* Emit enum values as: strings (default) / integers 

## Generation

```console
$ protoc --proto_path=IMPORT_PATH --python_out=DST_DIR --go_out=DST_DIR path/to/file.proto
```

## Style Guide

* Use CamelCase for message names, enums, services
* Use snake_case for field names
* Use plural words for repeated fields


## Encoding: Wire Format

Use [protoscope](https://github.com/protocolbuffers/protoscope?utm_source=developers.google.com&utm_medium=referral) to inspect low-level wire format.

### Integers and Field Numbers

Example:

```protobuf
message Test1 {
    optional int32 a = 1;
}
```

Encode `{ a: 150 }` and you'll see:

> 08 96 01

Wire encoding uses varints: variable-width integers. They allow encoding unsigned 64-bit integers using 1-10 bytes, with small values using fewer bytes.

Each byte in the varint has a continuation bit: indicates if the byte that follows it is part of the varint.

```
0000 0001
^ msb
```

Here is `150` encoded as `9601`:

```
10010110 00000001
^ msb    ^ msb
```

How to figure out 150? 
Drop the MSBs, then concatenate the 7-bit payloads, and interpret it as a little-endian 64-bit unsigned integer.

```
data:
10010110 00000001   // remove msb
 0010110  0000001   // put into little-endian order 
 0000001  0010110   // concatenate
 00000010010110     // interpret at integer
```

The type of scheme is called TLV (tag-length-value).
There are 6 wire types:

* `0` varint (all ints)
* `1` i64 (fixed64, double)
* `2` len (string, bytes, nested message, packed repeated fields)
* `5` i32 (fixed32, float)

The "tag" is a varint: field number + wire type 

> (field_number << 3) | wire_type

So: 

* Wire type: 3 bits + Field number: 4 bits + MSB: 1 bit; or (field numbers: 1..16)
* Wire type: 3 bits + Field number: 11 bits + MSG: 1+1 bits (field numbers: 1..2048)

Now, in our message, `08` is:

> 00001000

So it's type=0 field_number=1

### Length

Consider 

```protobuf
message Test2 {
  optional string b = 2;
}
```

Message `{b: "testing"}` will be encoded:

> 12 07 [74 65 73 74 69 6e 67]

```
12 = 00010 010  // type=2 (LEN), field number=1
07 = varint 7
next 7 bytes: string
```

Submessages also use the `LEN` wire type, indicating the length of the nested struct.

### Optional and Repeated Elements

Optional fields are just left out when not present. This means that "huge" protos with only a few fields set are quire sparse.

Repeated fields are just, well, repeated. 

A non-`repeated` field normally has only one instance. If the same field appears multiple times, the parser accepts the *last* value it sees.
The effect is that parsing the concatenation of two encoded messages produces exactly the same result as if you merged the resulting objects:

> message.ParseFromString(str1 + str2) == message.MergeFrom(message2)

This property is occasionally useful: e.g. for non-zero defaults?

Starting in v2.1.0 in proto3, `repeated` fields of scalar integer type can be packed. They are encoded as a single LEN record that contains each element concatenated.

### Maps

Maps are just a shorthand for a special kind of repeated field:

```protobuf
message Test6 {
  map<string, int32> g = 7;
}
```


this is actually the same as

```protobuf
message Test6 {
  message g_Entry {
    optional string key = 1;
    optional int32 value = 2;
  }
  repeated g_Entry g = 7;
}
```

### Field Order

Field numbers have no effect on serialization order. There is no guaranteed order. 

Do not assume the byte output of a serialized message is stable

## Techniques

### Streaming Multiple Messages

It's up to you to keep track of where one message ends and the next begins. The protobuf wire format is not self-delimiting.
Easiest way: write the message size before each message.

### Large Data Sets

Protocol Buffers are not designed to handle large messages. If you have >1Mb data, consider an alternative strategy.

Protocol Buffers are great for handling individual messages *within* a large data set: that is, a collection of small pieces.

### Self-Describing Messages

Protocol Buffers are not self-describing: a raw message does not mean much without a corresponding `.proto` file.
However, a self-describing message may be implemented using `google.protobuf.FileDescriptorSet` + `google.protobuf.Any`.

See [`src/google/protobuf/descriptor.proto`](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto).

The reason that this functionality is not included in the Protocol Buffer library is because we have never had a use for it inside Google.







# gRPC

Two types of RPC services:

* Simple RPC: waits for a response to come back
* Server-side streaming RPC: the client gets a stream to read a sequence of messages back
* Client-side streaming RPC: client writes a sequence of messages 
* Bi-directional streaming RPC: both sides send a sequence of messages



# protobuf/go.mod

```
module goplay/protobuf

go 1.19

require (
	github.com/golang/protobuf v1.5.2
	google.golang.org/protobuf v1.28.1
)

require google.golang.org/grpc/cmd/protoc-gen-go-grpc v1.2.0 // indirect

```



# protobuf/main.go

```go
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

```



# protobuf/protobuf.go

```go
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



```





# protobuf/proto


# protobuf/proto/example.proto

```protobuf
syntax = "proto3";
package goplaypb;

// option go_package = "goplay/protobuf/goplaypb";
option go_package = "goplay/protobuf/goplaypb";

import "google/protobuf/timestamp.proto";

message UserInfo {
    uint64 id = 1;
    string login = 2;
    string email = 3;

    optional uint32 age = 4; 
    google.protobuf.Timestamp last_updated = 5;
}






message GetUserInfoArgs {
    uint64 user_id = 1;
}

message GetUserInfoResult {
    UserInfo user = 1;
}


service Users {
    rpc GetUserInfo(GetUserInfoArgs) returns (GetUserInfoResult) {}
}

```





# protobuf


# protobuf/grpc.go

```go
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

```





# flatbuffers
# Flatbuffers

Main feature: memory efficient. Reads directly from the message. No parsing step required.

Define schema: `monster.fbs`. Then use

> $ flatc -o protoc --go proto/monster.fbs






# flatbuffers/proto


# flatbuffers/proto/monster.fbs

```flatbuffers
// IDL file: Interface Definition Language

// Package for the generated code
namespace goplay.fb;

// Enum, represented as 1 byte.
enum Color:byte {
    Red = 0,
    Green,  // default enum behavior: increment when unspecified
    Blue = 2,
}

// Union: any of
union Equipment {
    Weapon
    // .. add more tables
}

// Structs are ideal for data structures that will not change
// They use less memory and have faster lookup
struct Location {
    x: float;
    y: float;
    z: float;
}

// Tables 
table Monster {
    // Nested struct 
    position: Location;

    // Fieldswith default values.
    // Other fields, when unspecified, receive 0, null
    hp: short = 100;
    mana: short = 150;
    color: Color = Red;

    equipped: Equipment; // union

    // Deprecated value.
    // You cannot delete from a table, but for backwards-compatibility, you can mark fields as (deprecated).
    // This prevents the generation of accessors for this field. 
    friendly: bool = false (deprecated);

    // Arrays 
    inventory: [ubyte]; // array of scalars
    path: [Location];   // array of structs
    weapons: [Weapon];  // array of tables
}

// Table 
table Weapon {
    name: string; // string
    damage: short;
}


// The root type declares what will be the root table for the serialized data 
root_type Monster;

```





# flatbuffers


# flatbuffers/go.mod

```
module goplay/flatbuffers

go 1.19

require github.com/google/flatbuffers v22.11.23+incompatible // indirect

```



# flatbuffers/main.go

```go
package main

// $ go get github.com/google/flatbuffers/go

//go:generate flatc -o protoc --go proto/monster.fbs

import (
	"fmt"
	"goplay/flatbuffers/protoc/goplay/fb"
	"log"

	flatbuffers "github.com/google/flatbuffers/go"
)

func main(){
	if err := playFlatbuffers(); err != nil {
		log.Fatalf("playFlatbuffers() failed: %+v", err)
	}
}


func playFlatbuffers() error {
	// Create a monster
	b := flatbuffers.NewBuilder(1024)

	// Objects cannot be nested.
	// So we first create our strings and arrays, and then write the struct.
	name := b.CreateString("Sword")
	fb.WeaponStart(b)
	fb.WeaponAddName(b, name)
	fb.WeaponAddDamage(b, 3)
	sword := fb.WeaponEnd(b)

	// Create an array: inventory.
	fb.MonsterStartInventoryVector(b, 2)
	b.PrependByte(0)
	b.PrependByte(1)
	inv := b.EndVector(2)

	// Create an array: weapons. Build it, specify offset
	fb.MonsterStartWeaponsVector(b, 1)
	b.PrependUOffsetT(sword)
	weapons := b.EndVector(1)

	// Create an array: path
	fb.MonsterStartPathVector(b, 2)
	fb.CreateLocation(b, 1.0, 2.0, 3.0)
	fb.CreateLocation(b, 4.0, 5.0, 6.0)
	path := b.EndVector(2)

	// Create the monster

	fb.MonsterStart(b)
	fb.MonsterAddPosition(b, fb.CreateLocation(b, 1.0, 2.0, 3.0))
	fb.MonsterAddHp(b, 300)
	fb.MonsterAddColor(b, fb.ColorRed)

	// Add union: _type + value
	fb.MonsterAddEquippedType(b, fb.EquipmentWeapon)
	fb.MonsterAddEquipped(b, sword)
	
	// Add arrays
	fb.MonsterAddInventory(b, inv)
	fb.MonsterAddWeapons(b, weapons)
	fb.MonsterAddPath(b, path)

	// Done
	orc := fb.MonsterEnd(b)
	b.Finish(orc)

	// Serialize
	buf := b.FinishedBytes()
	fmt.Printf("Binary orc: %q\n", buf)
	
	// Unserialize
	monster := fb.GetRootAsMonster(buf, 0)
	fmt.Printf("Monster hp: %d\n", monster.Hp())

	return nil
}
```

