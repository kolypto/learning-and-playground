package main

import (
	"context"
	"database/sql"
	"fmt"

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
	defer db.Close()

	// Prepare context
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Verify that a connection can be made
	if err := db.PingContext(ctx); err != nil {
		return err
	}
	
	// BEGIN transaction
	tx, err := db.BeginTx(ctx, nil)
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
		if err != nil {
			return err
		}

		// Last insert id
		fmt.Printf("User id: %d\n",	id)
	}

	// QueryRows(): retrieve many results
	{
		// SELECT
		rows, err := tx.QueryContext(ctx, "SELECT name, age FROM users;")
		if err != nil {
			return err
		}
		defer rows.Close()
		
		// Fetch
		for rows.Next() {
			var (
				name string
				age sql.NullString  // NOTE: nullable type!
			)

			// Scan a row.
			// NOTE: you can pass a pointer, but be careful: it requires extra memory allocations and will degrade performance!
			if err := rows.Scan(&name, &age); err != nil {
				return err 
			}
			fmt.Printf("Person: name=%s age=%d\n", name, age)
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