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
