package main

import (
	"fmt"
	"log"
)

func main() {
	var playgrounds = []SimplePlayFunc{
		{"cockroachdb/errors", playCockroachdbErrors},
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
