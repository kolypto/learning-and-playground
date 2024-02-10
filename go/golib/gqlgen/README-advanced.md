

Inline Config With Directives
=============================

`gqlgen` has builtin directives that allow you to customize Go code inline:

```graphql
# Model configuration: e.g. map to a structure
directive @goModel(
	model: String
	models: [String!]
	forceGenerate: Boolean
) on OBJECT | INPUT_OBJECT | SCALAR | ENUM | INTERFACE | UNION

# Field configuation, e.g. generate a resolver
directive @goField(
	forceResolver: Boolean
	name: String
	omittable: Boolean
) on INPUT_FIELD_DEFINITION | FIELD_DEFINITION

# Add tags to struct fields, e.g. "xorm" or "yaml"
directive @goTag(
	key: String!
	value: String
) on INPUT_FIELD_DEFINITION | FIELD_DEFINITION
```

now use them:

```graphql
type User @goModel(model: "github.com/my/app/models.User") {
	id: ID! @goField(name: "todoId")
	name: String!
		@goField(forceResolver: true)
		@goTag(key: "xorm", value: "-")
		@goTag(key: "yaml")
}

# This make sense when autobind activated.
type Person @goModel(forceGenerate: true) {
	id: ID!
	name: String!
}
```


APQ: Automatic Persisted Queries
================================

When you work with GraphQL by default your queries are transferred with every request. That can waste significant bandwidth. To avoid that you can use Automatic Persisted Queries (APQ).

With APQ you send only query hash to the server. If hash is not found on a server then client makes a second request to register query hash with original query on a server.

See: <https://gqlgen.com/reference/apq/>

Changesets
==========

Occasionally you need to distinguish presence from nil (undefined vs null).
Use `map[string]any` for this.

See: <https://gqlgen.com/reference/changesets/>

Dataloader
==========

Solves the N+1 problem.

It pre-loads all referenced users by id when top-level entities are loaded,
then child resolvers just fetch it from the dataloader.

See: <https://gqlgen.com/reference/dataloaders/>

Field Collection
================

Know which fields were queried: only fetch required fields, without over-fetching.

Use `graphql.CollectFields()` and `graphql.CollectFieldsCtx()`.

See: <https://gqlgen.com/reference/field-collection/>

File Uploads
============

See: <https://gqlgen.com/reference/file-upload/>

Customized Errors
=================

The assumption is that any error message returned here is appropriate for end users.

```go
package foo

import (
	"context"
	"errors"

	"github.com/vektah/gqlparser/v2/gqlerror"
	"github.com/99designs/gqlgen/graphql"
)

// Resolver: adds errors
func (r Query) DoThings(ctx context.Context) (bool, error) {
	// String error
	graphql.AddErrorf(ctx, "Error %d", 1)

	// Pass an existing error
    err := gqlerror.Errorf("zzzzzt")
	graphql.AddError(ctx, err)

	// Error with custom fields
	graphql.AddError(ctx, &gqlerror.Error{
		Path:       graphql.GetPath(ctx),
		Message:    "A descriptive error message",
        // Custom fields
		Extensions: map[string]interface{}{
			"code": "10-4",
		},
	})

    // Alternatively, return a list of errors:
    errList := gqlerror.List{}

	// And you can still return an error if you need
	return false, gqlerror.Errorf("BOOM! Headshot")
}
```

All errors are passed through a hook that can customize their presentation:

```go

// Will be called with the same resolver `ctx` contxt, so you can extract the current resolver path
// and whatever state you might want to notify the client about.
server.SetErrorPresenter(func(ctx context.Context, e error) *gqlerror.Error {
    err := graphql.DefaultErrorPresenter(ctx, e)

    var myErr *MyError
    if errors.As(e, &myErr) {
        err.Message = "Eeek!"
    }

    return err
})
```

Panic handler:

```go
server.SetRecoverFunc(func(ctx context.Context, err interface{}) error {
    // notify bug tracker...
    return gqlerror.Errorf("Internal server error!")
})
```

Introspection
=============

Disable introspection:

```go
srv := handler.New(es)

srv.AddTransport(transport.Options{})
srv.AddTransport(transport.POST{})

if os.Getenv("ENVIRONMENT") == "development" {
    srv.Use(extension.Introspection{})
}
```

Disable introspection based on authentication:

```go
srv := handler.NewDefaultServer(es)
srv.AroundOperations(func(ctx context.Context, next graphql.OperationHandler) graphql.ResponseHandler {
    if !userForContext(ctx).IsAdmin {
        graphql.GetOperationContext(ctx).DisableIntrospection = true
    }

    return next(ctx)
})
```

Code Generation Plugins
=======================

Customize code generation.

See: <https://gqlgen.com/reference/plugins/>

See: <https://gqlgen.com/recipes/modelgen-hook/>

Query Complexity
================

Limit the complexity of queries.

See: <https://gqlgen.com/reference/complexity/>

Bind Structs and Methods to GraphQL fields
==========================================

See: <https://gqlgen.com/reference/resolvers/>

Scalars
=======

Built-in scalars: `Time`, `Any`, `Upload`, `Map`:

```graphql
scalar Time
```

Also, `UUID` and `Duration`:

```yaml
models:
  UUID:
    model:
      - github.com/99designs/gqlgen/graphql.UUID
  Duration:
    model:
      - github.com/99designs/gqlgen/graphql.Duration
```

Custom scalars: see <https://gqlgen.com/reference/scalars/>


Directives
==========

See: <https://gqlgen.com/reference/directives/>

```graphql
type Mutation {
	deleteUser(userID: ID!): Bool @hasRole(role: ADMIN)
}

directive @hasRole(role: Role!) on FIELD_DEFINITION

enum Role {
    ADMIN
    USER
}
```

Code will be generated.


Recipes
=======

* Authentication: <https://gqlgen.com/recipes/authentication/>
* CORS: <https://gqlgen.com/recipes/cors/>
* Subscriptions: <https://gqlgen.com/recipes/subscriptions/>
