OpenFGA
=======

Date: 2025-05-27

Glossary:

* Authentication ensures a user's identity.
* Authorization determines if a user can perform a certain action on a particular resource.
* FGA: Fine-Grained Authorization

OpenFGA is built on Google Zanzibar's model, specifically designed for fine-grained, relationship-based access control (ReBAC).
You define relationships (e.g., "user X is editor of document Y," "document Y is in workspace Z," "user A is member of workspace Z")
and OpenFGA efficiently answers "can user X do action A on object O?". This is perfect for dynamic, hierarchical sharing.

* It runs as a service, but can also be used as a library in Go.
* It needs a back-end (memory, Postgres, MySQL, SQLite) to store relations
* It can list objects that you have access to. If they are many, it can stream as results come in
* Supports RBAC and ABAC models
* APIs: HTTP and gRPC

Introduction
============

## Permissions models

* **RBAC**: Role-Based Access Control. Example: you need to have the "editor" role to edit anything in the system.
  Such systems have: users, groups, roles, permissions.
* **ABAC**: Attribute-Based Access Control. Permissions are granted based on a set of attributes that a user or resource possesses.
  Example: a user assigned both "marketing" and "manager" attributes is entitled to publish and delete posts that have a "marketing" attribute.
  Such applications need to retrieve all data fields to make decisions.
* **PBAC**: Policy-Based Access Control. Centralized policy store. Most implementations of ABAC are also PBAC.
* **ReBAC**: Relationship-Based Access Control. Defined by (users) having (relationships) to (objects).
  ReBAC is a superset of RBAC: you can fully implement RBAC with ReBAC.
  It can also natively solve for ABAC when attributes can be expressed in the form of relationships:
  ‘a user’s manager’, ‘the parent folder’, ‘the owner of a document’, ‘the user’s department’.
  See extended support: Conditions, Contextual Tuples

## Concepts

A **Type** is a string. A class of objects. Examples: workspace, repository, organization, document.

A **type definition** defines all possible relations to this type:

```openfga
type document
  relations
    define viewer: [user]
    define commenter: [user]
    define editor: [user]
    define owner: [user]
```

Together with **relationship tuples**, the **authorization model** determines whether a relationship exists between a user and an object.

An **Object** is an entity in the system: a type plus an identifier:

```
workspace:fb83c013-3060-41f4-9590-d3233a67938f
repository:auth0/express-jwt
organization:org_ajUc9kJ
document:new-roadmap
```

A **User** is an entity in the system that can be related to an object.
In Zanzibar, a **User** is an entity granted access, while an **Object** is the entity being referenced.
A **User** is a combination of a type, an identifier, and an optional relation.
It can be:

* Any *identifier*: `user:anne`
* Any object: `workspace:fb83c013-3060-41f4-9590-d3233a67938f`
* A **userset** (set of objects): `organization:org_ajUc9kJ#members` (all users related to the organization)
* Everyone: `*`

A **Relation** is a string defined in the *type definition*: a possible relationship to this object.
A **Relation definition** lists the conditions under which a relationship is possible.

Example:
A `user` can be an `editor` or a `document` as a `user` (directly related),
or as a `team#member` (indirectly: a set relationship).
As a result:

* you can add `user:anne` as an `editor` to the `document:123`, or
* add `user:anne` to the `team:backend` as a `member` and grant them the `editor`.

```openfga
type document
  relations
    define viewer: [user]
    define commenter: [user]
    define editor: [team#member, user]
    define owner: [user]

type user

type team
  relations
    define member: [user]
```


A **relationship tuple** is a triplet of `(user, relation, object)`.

```js
[{
  "user": "user:anne",
  "relation": "editor",
  "object": "document:new-roadmap"
}]
```

A **conditional relationship tuple** is a relation that only exists if a **condition** holds true.
It calls function `less_than_hundred(x: 20)` and is only permissible when the condition evaluates to true:

```js
[{
  "user": "user:anne",
  "relation": "editor",
  "object": "document:new-roadmap",
  "condition": {
    "name": "less_than_hundred",
    "context": { "x": 20 }
  }
}]
```

A **direct relationship** exists if the authorization model allows that, and an exact tuple exists.
An **implied relationship** exists if the user relates through another object or objects:

```js
// Direct:
// User:Anne is in direct relationship
[{ user: "user:anne", relation: "owner",  object: "document:123" }]

// Everyone is in direct relationship
[{ user: "user:*",    relation: "viewer", object: "document:123" }]

// The userset is directly related AND Anne is a member of the userset (also direct)
[{ user: "team:dev#member", relation: "editor", object: "document:123" },
 { user: "user:anne", relation: "member", object: "team:dev#member"}
]

// Implied:
// Relationship "viewer" is implied from "editor"
[{ user: "user:anne", relation: "editor", object: "document:123" }]
```

An **authorization model** would be a collection of *type definitions*.
Example:

```openfga
model
  schema 1.1

type document
  relations
    define viewer:    [domain#member, user]
    define commenter: [domain#member, user]
    define editor:    [domain#member, user]
    define owner:     [domain#member, user]

type domain
  relations
    define member: [user]

type user
```

*Authorization models* are kept in a **store**.

A **Condition** is a function (defined in [Google Common Expression Language](https://github.com/google/cel-spec)) that evaluates to a boolean outcome:

```gcl
condition less_than_hundred(x: int) {
  x < 100
}
```

A **Contextual Tuple** are temporary (user,relation,object) tuples that are not persisted.
They can be added to requests (check, list, expand) to establish temporary (preview?) relationships.




## Queries

A **check request** is a call to OpenFGA endpoint to check whether the *relationship* exists between a *user* and an *object*:

```go
ctx := context.Background()

options := ClientCheckOptions{
    AuthorizationModelId: PtrString("01HVMMBCMGZNT3SED4Z17ECXCA"),
}

body := ClientCheckRequest{
    User:     "user:anne",
    Relation: "viewer",
    Object:   "document:new-roadmap",
}

data, err := fgaClient.Check(ctx).
    Body(body).
    Options(options).
    Execute()
// data = { allowed: true }
```

A **list object request** returns all *objects* of a given *type* that the *user* has a specified *relationship* with:

```go
ctx := context.Background()

options := ClientListObjectsOptions{
    AuthorizationModelId: PtrString("01HVMMBCMGZNT3SED4Z17ECXCA"),
}

body := ClientListObjectsRequest{
    User:     "user:anne",
    Relation: "viewer",
    Type:     "document",
}

data, err := fgaClient.ListObjects(ctx).
    Body(body).
    Options(options).
    Execute()

// data = { "objects": ["document:otherdoc", "document:planning"] }
```

A **list users request** lists all *users* of a given *type* that have a specified *relationship* with the *object*:

```go
ctx := context.Background()

options := ClientListUsersOptions{
    AuthorizationModelId: PtrString("01HVMMBCMGZNT3SED4Z17ECXCA"),
}

userFilters := []openfga.UserTypeFilter{{ Type:"user" }}

body := ClientListUsersRequest{
    Object:       openfga.Object{
        Type:    "document",
        Id:      "planning",
    },
    Relation:     "viewer",
    UserFilters:   userFilters,
}

data, err := fgaClient.ListUsers(ctx).
    Body(body).
    Options(options).
    Execute()

// data.Users = [{"object":{"type":"user","id":"anne"}}, {"object":{"type":"user","id":"beth"}}]
```


