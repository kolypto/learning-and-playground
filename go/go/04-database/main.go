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
