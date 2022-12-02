// ORM libraries

package main

import (
	"context"
	"database/sql"
	"fmt"

	sq "github.com/Masterminds/squirrel"
	"github.com/doug-martin/goqu/v9"
	_ "github.com/doug-martin/goqu/v9/dialect/postgres"
	"github.com/jmoiron/sqlx"
)

func PlayOrm() error {
	var playgrounds = []SimplePlayFunc{
		// TODO: learn GORM. 
		// But people advice against it and favor:
		// * raw SQL: learn once, use everywhere. Simplicity and more control.
		// * query builders: because with raw SQL you'd resort to templating
		// * code generators: because they give you type-safe code
		// {"GORM", playGorm},
		{"Squirrel", playSquirrel},
		{"goqu", playGoqu},
	}
	
	for _, playfunc := range playgrounds {
		fmt.Printf("### %s\n", playfunc.Name)
		err := playfunc.Func()
		if err != nil {
			return fmt.Errorf("%s failed: %s", playfunc.Name, err)
		}
	}

	return nil
}

func playGorm() error {
	return nil
}

func playSquirrel() error {
	// Cache
	// dbCache := sq.NewStmtCache(db)
	// mydb := sq.StatementBuilder.RunWith(dbCache)

	// Postgres
	// psql := sq.StatementBuilder.PlaceholderFormat(sq.Dollar)

	// SELECT, JOIN, WHERE
	users := sq.Select(`*`).From(`users`).Join(`emails USING (email_id)`)
	activeUsers := users.Where(sq.Eq{
		"deleted_at": nil,  // IS NULL
	})
	if true {
		activeUsers = activeUsers.Where("age > ?", 18)
	}

	sql, args, err := activeUsers.ToSql()
	if err != nil {
		return err
	}
	
	fmt.Printf("SQL: %s %v\n", sql, args)

	// INSERT
	sql, args, err = sq.
		Insert("users").Columns("name", "age").
		Values("moe", 13).
		Values("larry", sq.Expr("? + 5", 12)).
		Suffix(`RETURNING id`).
		ToSql()
	if err != nil {
		return err	
	}
	fmt.Printf("SQL: %s %v\n", sql, args)
	

	// Run immediately:
	// query = query.RunWith(m.db).PlaceholderFormat(sq.Dollar)
	// query.QueryRow().Scan(&node.id)

	return nil
}

func playGoqu() error {
	// We tried a few other sql builders but each was a thin wrapper around sql fragments that we found error prone. 
	// We created an expressive DSL that would find common errors with SQL at compile time
	
	// Dialect
	pg := goqu.Dialect("postgres")

	// Use it on a DB
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}
	pgdb := pg.DB(db)

	db.MustExec(usersSchema)

	// SELECT
	query, args, err := pg.From(`test`).
		Where(goqu.Ex{
			"d": []string{"a", "b", "c"},  // WHERE d IN ('a', 'b', 'c') !
		}).
		ToSQL()
	if err != nil {
		return err
	}
	fmt.Printf("SQL: %s %v\n", query, args)

	// Count(), type-safe
	if count, err := pgdb.From("users").Count(); err != nil {
		return err
	} else {
		fmt.Printf("Count: %d\n", count)
	}

	// Clause methods:
	// Ex{}: map: identifier => value (WHERE)
	// ExOr{}: OR version 
	// S(), T(), C(): Schema, Table, Column
	// I(): Table.Column
	// L: SQL literal
	// V: Value to be used

	// Ex{}, Op{}
	{
		sql, _, _ := pgdb.From(`items`).Where(goqu.Ex{
			"a": "a",  					// a == "a'"
			"b": goqu.Op{"neq": 1}, 	// b != 1
			"c": nil,  					// c IS NULL
			"d": []int{1,2,3},  		// d IN (1,2,3)
		}).ToSQL()
		fmt.Printf("SQL: %s\n", sql)
	}
	
	// S(), T(), C()
	{
		t := goqu.T("users")
		sql, _, _ := pgdb.From(t).Select(
			t.Col("id"),  // SELECT users.id
		).Where(
			goqu.C("age").Gte(18),  // age >= 18
		).ToSQL()
		fmt.Printf("SQL: %s\n", sql)
	}

	// I()
	{
		id := goqu.I("users.id") // "table.column", or just "column"
		sql, _, _ := pgdb.From(id.GetTable()).Select(id).ToSQL()
		fmt.Printf("SQL: %s\n", sql)
	}

	// L(), V()
	{
		sql, args, _ := pgdb.From("users").Select(
			goqu.V(true).As("is_verified"),  // literal value
		).Where(
			goqu.L(`age >= ?`, 18),  // literal expr
		).ToSQL()
		fmt.Printf("SQL: %s %v\n", sql, args)
	}

	// TODO: See further: SELECT , INSERT, UPDATE, DELETE dataset, PREPAREd statements, Database, Time


	return nil
}
	
func playS() error {
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

	ctx.Done()

	return nil
}