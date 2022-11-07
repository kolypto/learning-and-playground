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

