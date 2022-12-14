// ORM libraries

package main

import (
	"context"
	"database/sql"
	_ "embed"
	"fmt"
	"time"

	"entgo.io/ent/dialect"
	sq "github.com/Masterminds/squirrel"
	"github.com/doug-martin/goqu/v9"
	_ "github.com/doug-martin/goqu/v9/dialect/postgres"
	"github.com/jmoiron/sqlx"
	"github.com/pkg/errors"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"

	"goplay/database/sql/ent/ent"
	"goplay/database/sql/ent/ent/car"
	"goplay/database/sql/ent/ent/user"
	"goplay/database/sql/sqlboiler/models"
	"goplay/database/sql/sqlc/dbs"

	entsql "entgo.io/ent/dialect/sql"
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
		{"sqlc", playSqlc},
		{"sqlboiler", playSqlboiler},
		{"ent", playEntityFramework},
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
	defer db.Close()
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
	
func playSqlc() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this

	// Create tables
	tx := db.MustBegin()
	defer tx.Rollback()
	tx.MustExec(sqlcSchema)

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Get queries
	queries := dbs.New(tx)

	// Create a user
	{
		createdUser, err := queries.CreateUser(ctx, dbs.CreateUserParams{
			Login: "kolypto",
			Age: sql.NullInt32{35, true},
		})
		if err != nil {
			return err
		}
		fmt.Printf("Created user id: %d\n", createdUser.ID)
	}

	// List users
	{
		users, err := queries.ListUsers(ctx)
		if err != nil {
			return err
		}
		fmt.Printf("Users: %v\n", users)
	}

	ctx.Done()
	return nil
}


func playSqlboiler() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this
	
	// Set global database for G() methods
	boil.SetDB(db)

	// Create tables
	tx := db.MustBegin()
	defer tx.Rollback()
	tx.MustExec(sqlboilerSchema)

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Users.Count()
	{
		count, err := models.Users().Count(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Count: %d\n", count)
	}

	// Users().All(), Limit()
	{
		users, err := models.Users(
			qm.Limit(5),
		).All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Users: %v\n", users[0])
	}

	// Users.DeleteAll()
	{
		n, err := models.Users(
			models.UserWhere.ID.GT(100),
		).DeleteAll(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Deleted: %d rows\n", n)
	}
	
	// NewQuery(): custom query
	{
		rows, err := models.NewQuery(qm.From(`busers`)).QueryContext(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("NewQuery(): %v\n", rows)
		rows.Close()
	}

	// Query Mods
	{
		// qm.SQL(): raw sql
		users, err := models.Users(qm.SQL(`SELECT * FROM busers WHERE id=$1`, 1)).All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("SQL(): %v\n", users)

		// qm.Select(), qm.From()
		users, err = models.Users(
			// qm.From("busers"),
			
			// Columns: by name, or by constant
			qm.Select(
				"id",
				models.UserColumns.Login,
			),
			// Where: string, or expression
			qm.Or2(qm.Expr(
				qm.Where("id > ?", 0),
				models.UserWhere.ID.GT(0),
			)),

			// Eager loading
			qm.Load(models.UserRels.AuthoredVideos),
		).All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Users(qm): %v\n", users)
	}

	// Finishers: One(), all() ; Count(), Exists() ; UpdateAll(), DeleteAll(); Exec(); Bind() , Query(), QueryRow()
	// Bind() finisher
	{
		var users []models.User  // or a custom struct
		err := queries.Raw(`SELECT * FROM busers`).Bind(ctx, tx, &users)
		if err != nil {
			return err
		}
		fmt.Printf("Raw().Bind(): %v\n", users)
	}

	// Relationships
	{
		// Get one user
		user, err := models.FindUser(ctx, tx, 1)
		if err != nil {
			return err
		}
		
		// Get related
		videos, err := user.AuthoredVideos().All(ctx, tx)
		if err != nil {
			return err
		}
		fmt.Printf("Related videos: %v\n", videos)
	}

	// Hooks
	models.AddUserHook(boil.AfterInsertHook, func(ctx context.Context, exec boil.ContextExecutor, p *models.User) error {
		return nil
	})

	ctx.Done()
	return nil
}


func playEntityFramework() error {
	// Set up the pool
	db, err := sqlx.Open("pgx", "postgres://postgres:postgres@localhost:5432")
	if err != nil {
		return err
	}	
	defer db.Close() // you rarely need this
	
	// Create an ent.Driver from `db`
	driver := entsql.OpenDB(dialect.Postgres, db.DB)
	client := ent.NewClient(ent.Driver(driver))

	// Prepare context
	// It will stop any running queries in case we quit. That's structured concurrency.
	ctx, stop := context.WithCancel(context.Background())
	defer stop()

	// Transaction
	tx, err := client.Tx(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Run the auto-migration tool
	if err := tx.Client().Schema.Create(ctx); err != nil {
		return errors.WithMessage(err, "Migration failed")
	}

	// User.Create()
	{
		user, err := tx.User.Create().
			SetLogin("john").
			SetAge(30).
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to create a user")
		}
		fmt.Printf("User created: id=%d\n", user.ID)
	}

	// User.Query() 
	{
		user, err := tx.User.Query(). 
				Where(user.Login("john")).  // same as: 
			Only(ctx)  // Assert: exactly 1 user 
		if err != nil {
			return errors.WithMessage(err, "User not found")
		}
		fmt.Printf("User: %v\n", user)
	}
	
	// User.AddCars() relationship
	{
		tesla, err := tx.Car. 
			Create(). 
			SetModel("Tesla"). 
			SetRegisteredAt(time.Now()).
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to create a car")
		}

		ford, err := tx.Car. 
			Create(). 
			SetModel("Ford"). 
			SetRegisteredAt(time.Now()). 
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to create a car")
		}

		// AddCars()
		owner, err := tx.User. 
			Create(). 
			SetAge(30). 
			SetLogin("Owner"). 
			AddCars(tesla, ford).
			Save(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to save a user")
		}

		fmt.Printf("Created User(id=%d) with cars [Tesla(id=%d), Ford(id=%d)]\n", owner.ID, tesla.ID, ford.ID)

		// QueryCars()
		cars, err := owner.QueryCars().Where(
			car.ModelNotIn("Lada"),
		).All(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to load user cars")
		}
		fmt.Printf("QueryCars(): %v\n", cars)

		// Car.QueryOwner()
		owner, err = tesla.QueryOwner().Only(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to find owner's car")
		}

		fmt.Printf("Car %q owner: %q\n", cars[0], owner)
	}

	// Traverse graph
	{
		cars, err := tx.User.Query(). 
			Where(user.HasCars()). 
			QueryCars(). 
			All(ctx)
		if err != nil {
			return errors.WithMessage(err, "Failed to find cars for user #1")
		}

		fmt.Printf("Cars for User: %v\n", cars)
	}

	ctx.Done()
	return nil
}

//go:embed sqlc/schema.sql
var sqlcSchema string

//go:embed sqlboiler/schema.sql
var sqlboilerSchema string 
