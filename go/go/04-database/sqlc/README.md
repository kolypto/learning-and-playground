# Installation

```console
$ go install github.com/kyleconroy/sqlc/cmd/sqlc@latest
```

sqlc needs to know your schema and queries:

* `sqlc.yaml` is the config
* `schema.sql` is the DB schema
* `query.sql` are your queries

Every query has a name, and a command:

* `:one` returns one row: `Object`
* `:many` returns many rows: `[]Object` slice
* `:exec` returns nothing; only `err`
* `:execresult` returns `sql.Result`
* `:execrows` returns `int` number of affected rows
* `:execlastid` returns `int` last insert id 
* `:batchexec` will receive a list of objects and return a `pgx.BatchResults`
* `:batchmany` will let you process `[]Object` rows with a callback
* `:batchone` will let you process `Object` row with a callback
* `:copyfrom` to use the Copy Protocol: inserts many rows faster

Now go:

```console
$ sqlc generate
```

Notes:
* Use `sqlc.arg(name)` or `@name` to override a name (named parameter)
* Use `sqlc.narg(name)` to force non-nullable arg

sqlc parses `CREATE TABLE` and `ALTER TABLE` statements. 
sqlc parses migration files in lexicographic order. 

sqlc is able to ignore *down* migrations from: dbmate, golang-migrate, goose, sql-migrate, tern

