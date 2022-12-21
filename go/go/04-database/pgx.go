// pgx: Postgres client

package main

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

func PlayPgx() error {
	// Context
	ctx, done := context.WithCancel(context.Background())
	defer done()

	// Pool
	// To disable prepared statements: ?default_query_exec_mode=simple_protocol
	db, err := pgxpool.New(ctx, "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}
	defer db.Close()

	// Transaction
	// Use Begin() on it to use SAVEPOINTs
	tx, err := db.BeginTx(ctx, pgx.TxOptions{})
	if err != nil {
		return err
	}

	// Use: BeginFunc() to exec a function in a transaction
	pgx.BeginFunc(ctx, db, func(tx pgx.Tx) error {
		// will COMMIT when done
		return nil
	})

	// Rollback() is safe to call even if the transaction is closed
	defer tx.Rollback(ctx)

	// Exec(): create schema
	{
		_, err := tx.Exec(ctx, usersSchema)
		if err != nil {
			return err
		}
	}

	// Insert some rows
	{
		var uid int 
		err := tx.QueryRow(ctx, 
			`INSERT INTO users (name, age) VALUES(@name, @age) RETURNING id;`,
			pgx.NamedArgs{
				"name": "kolypto",
				"age": 35,
			},
		).Scan(&uid)
		if err != nil {
			return err
		}
		fmt.Printf("INSERT id=%d\n", uid)
	}

	// CollectRow(): returns a value from the first row
	// CollectRows(): returns a slice
	{
		// SELECT
		rows, err := tx.Query(ctx, `SELECT name from users;`)
		if err != nil {
			return err
		}	

		// CollectRows() -> slice
		names, err := pgx.CollectRows(rows, pgx.RowTo[string])  // Generic, func() returns string
		if err != nil {
			return err
		}

		fmt.Printf("Names: %v\n", names)
	}
	
	// ForEachRow(): invoke callback on every row
	{
		// SELECT
		rows, err := tx.Query(ctx, `SELECT age from users;`)
		if err != nil {
			return err
		}	
		
		// ForEachRow(): aggregate max age
		var maxAge, age int 
		_, err = pgx.ForEachRow(rows, []any{&age}, func() error {
			if age > maxAge {
				maxAge = age 
			}
			return nil 
		})

		fmt.Printf("Max age: %d\n", maxAge)
	}

	// ToRow(): scan rows into maps, structs, etc
	// See: RowToStructByName() and RowToMap()
	{
		// SELECT
		rows, err := tx.Query(ctx, `SELECT id, name, age FROM users`)
		if err != nil {
			return err 
		}

		// RowToStructByName()
		users, err := pgx.CollectRows(rows, pgx.RowToStructByName[UserRow])
		if err != nil {
			return err
		}

		fmt.Printf("User: %v\n", users)
	}

	// QueryRow()
	{
		var (
			name string 
		)
		err = tx.QueryRow(ctx, `SELECT 'hey';`).Scan(&name)
		if err != nil {
			return err 
		}

		fmt.Printf("Scan(): name=%s\n", name)
	}

	return nil
}

const usersSchema = `
	CREATE TEMPORARY TABLE users (
		id SERIAL PRIMARY KEY,
		name varchar NOT NULL,
		age int
	);
`


// Struct for pgx.RowToStructByName()
type UserRow struct {
	Id int   // `db:id`  // field name override
	Name string 
	Age sql.NullInt64  // compatible with pgx
}