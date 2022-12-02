// Code generated by sqlc. DO NOT EDIT.
// versions:
//   sqlc v1.16.0
// source: query.sql

package dbs

import (
	"context"
	"database/sql"
)

const createUser = `-- name: CreateUser :one
INSERT INTO users (login, age) VALUES($1, $2)
RETURNING id, login, age
`

type CreateUserParams struct {
	Login string        `db:"login" json:"login"`
	Age   sql.NullInt32 `db:"age" json:"age"`
}

func (q *Queries) CreateUser(ctx context.Context, arg CreateUserParams) (User, error) {
	row := q.db.QueryRowContext(ctx, createUser, arg.Login, arg.Age)
	var i User
	err := row.Scan(&i.ID, &i.Login, &i.Age)
	return i, err
}

const deleteUser = `-- name: DeleteUser :exec
DELETE FROM users
WHERE id = $1
`

func (q *Queries) DeleteUser(ctx context.Context, id int64) error {
	_, err := q.db.ExecContext(ctx, deleteUser, id)
	return err
}

const getUser = `-- name: GetUser :one
SELECT id, login, age FROM users
WHERE id = $1 LIMIT 1
`

func (q *Queries) GetUser(ctx context.Context, id int64) (User, error) {
	row := q.db.QueryRowContext(ctx, getUser, id)
	var i User
	err := row.Scan(&i.ID, &i.Login, &i.Age)
	return i, err
}

const listUsers = `-- name: ListUsers :many
SELECT id, login, age FROM users
ORDER BY login
`

func (q *Queries) ListUsers(ctx context.Context) ([]User, error) {
	rows, err := q.db.QueryContext(ctx, listUsers)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var items []User
	for rows.Next() {
		var i User
		if err := rows.Scan(&i.ID, &i.Login, &i.Age); err != nil {
			return nil, err
		}
		items = append(items, i)
	}
	if err := rows.Close(); err != nil {
		return nil, err
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return items, nil
}
