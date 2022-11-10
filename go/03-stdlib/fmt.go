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