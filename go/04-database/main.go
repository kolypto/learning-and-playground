package main

import "log"

func main() {
	err := PlayDatabaseSqlPostgres()
	if err != nil {
		log.Fatalf("Database got killed: %v", err)
	}

	err = PlayPgx()
	if err != nil {
		log.Fatalf("Pgx got itself killed: %v", err)
	}

	err = PlaySqlx()
	if err != nil {
		log.Fatalf("Sqlx got itself killed: %v", err)
	}
}