package main

import "log"

func main() {
	err := PlayDatabaseSqlPostgres()
	if err != nil {
		log.Fatalf("Database got killed: %v", err)
	}
}