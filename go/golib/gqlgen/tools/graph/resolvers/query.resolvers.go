package resolvers

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.
// Code generated by github.com/99designs/gqlgen version v0.17.43

import (
	"context"

	"github.com/kolypto/play/gqlgen/tools/graph"
	"github.com/kolypto/play/gqlgen/tools/graph/model"
)

// Query.todos
func (r *queryResolver) Todos(ctx context.Context) ([]*model.Todo, error) {
	// Fetch from DB and return
	return []*model.Todo{
		{ID: "1", Text: "one", Done: false, User: &model.User{ID: "1", Name: "test"}},
	}, nil
}

// Query.todos.user
func (r *todoResolver) User(ctx context.Context, obj *model.Todo) (*model.User, error) {
	// Load the user (only if requested)
	// Use the parent `obj.UserID` to know which one to load
	return &model.User{ID: obj.UserID, Name: "test"}, nil
}

// Query returns graph.QueryResolver implementation.
func (r *Resolver) Query() graph.QueryResolver { return &queryResolver{r} }

// Todo returns graph.TodoResolver implementation.
func (r *Resolver) Todo() graph.TodoResolver { return &todoResolver{r} }

type queryResolver struct{ *Resolver }
type todoResolver struct{ *Resolver }
