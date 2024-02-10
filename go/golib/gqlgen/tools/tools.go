// $ cd tools/
// $ go run github.com/99designs/gqlgen init

//go:build tools
// +build tools

//go:generate go run github.com/99designs/gqlgen generate

// Now generate me with:
// $ go generate ./...

package tools

import (
	_ "github.com/99designs/gqlgen"
	_ "github.com/99designs/gqlgen/graphql/introspection"
)
