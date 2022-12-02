-- name: GetUser :one
SELECT * FROM users
WHERE id = $1 LIMIT 1;

-- name: ListUsers :many
SELECT * FROM users
ORDER BY login;

-- name: CreateUser :one
INSERT INTO users (login, age) VALUES(@login, @age)
RETURNING *;

-- name: DeleteUser :exec
DELETE FROM users
WHERE id = $1;
