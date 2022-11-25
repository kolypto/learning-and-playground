package main

import (
	"context"
	"fmt"

	"github.com/jmoiron/sqlx" // drop-in replacement
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
	
	// Beginx(), BeginTxx()
	tx, err := db.BeginTxx(ctx, nil)
	if err != nil {
		return err
	}

	// Queryx()
	{
		rows, err := tx.Queryx(`SELECT id, name, age FROM users`)
		if err != nil {
			return err
		}
		defer rows.Close()

		// Next(), StructScan() into struct
		for rows.Next() {
			var user UserRow
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

	// Select() to load multiple rows
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

	
	return nil
}

