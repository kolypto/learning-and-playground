package tools

import (
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/kolypto/play/gqlgen/tools/graph"
	"github.com/kolypto/play/gqlgen/tools/graph/resolvers"
)

func Main() {
	// Port number from environment
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	// Schema and server
	schema := graph.NewExecutableSchema(graph.Config{
		Resolvers: &resolvers.Resolver{},
	})
	srv := handler.NewDefaultServer(schema)

	// `net/http` handles it
	http.Handle("/", playground.Handler("GraphQL playground", "/query"))
	http.Handle("/query", srv)

	// ListenAndServe()
	log.Printf("connect to http://localhost:%s/ for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
