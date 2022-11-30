package main

import (
	"context"
	"database/sql"
	"fmt"

	_ "github.com/jackc/pgx/v5"
	"github.com/jmoiron/sqlx" // drop-in replacement, and a superset of "database/sql"
)

func PlaySqlx() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this
	
	// MustExec() panics on error
	db.MustExec(usersSchema)
	db.MustExec(`INSERT INTO users (name, age) VALUES($1, $2)`, "kolypto", 35)

	// Prepare context
	ctx, stop := context.WithCancel(context.Background())
	defer stop()
	
	// Beginx(), BeginTxx(), MustBegin()
	tx, err := db.BeginTxx(ctx, nil)
	if err != nil {
		return err
	}

	// INSERT, with named struct and batch insert
	{
		_, error := tx.NamedExec(
			`INSERT INTO users (name, age) VALUES (:name, :age)`,
			[]UserRow{
				{Name: "John", Age: sql.NullInt64{Int64: 30, Valid: true}},
				{Name: "Jack", Age: sql.NullInt64{Int64: 31, Valid: true}},
			},
			// Can also use map:
			// map[string]any{"name": "John", "Age": 30},
		)
		if err != nil {
			return error
		}
	}

	// Queryx()
	{
		rows, err := tx.Queryx(`SELECT id, name, age FROM users`)
		if err != nil {
			return err
		}
		defer rows.Close()

		// Next(), StructScan() into struct
		var user UserRow  // scan into the same struct every time
		for rows.Next() {
			err = rows.StructScan(&user)
			if err != nil {
				return err
			}
			fmt.Printf("Userx: %v\n", user)
		}
	}
	
	// Get() to load one row: into struct, or scannable scalar
	{
		var user UserRow
		err := tx.Get(&user, `SELECT id, name, age FROM users WHERE id=$1`, 1)
		if err != nil {
			return err 
		}
		
		fmt.Printf("Get() user: %v\n", user)
	}

	// Select() to load multiple rows into a slice
	// WARNING: it will load the entire result set into memory at once!
	{
		var users []UserRow
		err := tx.Select(&users, `SELECT id, name, age FROM users`)
		if err != nil {
			return err
		}

		fmt.Printf("Select() users: %v\n", users)
	}
	
	// NamedQuery() allows the use of named parameters from maps and structs
	{
		stmt, err := tx.PrepareNamed(`SELECT id, name, age FROM users WHERE name=:name`)
		if err != nil {
			return err
		}
		
		params := map[string]any{
			"name": "kolypto",
		}
		var user UserRow
		err  = stmt.Get(&user, params)
		if err != nil {
			return err
		}

		fmt.Printf("NamedQuery() user: %v\n", user)
	}

	// In() expands slice, returning the modified query string and a new arg list that can be executed.
	// The query should use the "?" bind var.
	{
		query, args, error := sqlx.In(`SELECT * FROM users WHERE id IN ?;`, []int{1, 2, 3})
		if err != nil {
			return error
		}
		fmt.Printf("Query=%s, args=%v\n", query, args)
	}

	// Named() returns a new query with :name :name replaced with `?` `?` and actual values represented as an array
	{
		query, args, error := sqlx.Named(`INSERT INTO users (name, age) VALUES(:name, :age);`, map[string]any{"name": "me", "age": 0})
		if err != nil {
			return error 
		}
		fmt.Printf("Query=%s, args=%v\n", query, args)
	}

	
	return nil
}

