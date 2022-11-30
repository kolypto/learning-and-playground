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