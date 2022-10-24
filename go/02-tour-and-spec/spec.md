# The Go Programming Language Spec

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

















# Effective Go

