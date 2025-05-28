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

Alternatives:

* OpenFGA: strongest community and backing by CNCF and Auth0
* https://github.com/authzed/spicedb ⭐
* https://github.com/ory/keto (has ory/kratos authentication: works in tandem)
* https://github.com/Permify/permify (supports ABAC for dynamic attributes)
* https://github.com/aserto-dev/topaz
* https://github.com/warrant-dev/warrant

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

**Public access** means that every user can have this relationship:
i.e. it is possible to make the object "public".
NOTE: it is possible to add exceptions using `but not`.
Therefore, if `user:*` exists, it does not necessarily mean that a specific user has access. Use check!

```openfga
type user

type group
  relations
    define member: [ user, user:* ]
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


## Configuration Language

Describes the relations possible for an object of a given type and lists the conditions under which one is related to that object.

Example for direct relationships:

```openfga
model
  schema 1.1

type user

# Object
type domain
  relations
    # Every relation has a type restriction: which objects can be "users" for this object.
    # Syntax: "define <relation-name>: who can get it"
    # These allow direct relationships:
    # - define member: [type]        -- allows (type:id, member, domain:id) to be added
    # - define member: [type:*]      -- allows (type:*, member, domain:id) to be added
    # - define member: [type:rel]    -- allows (type:id#rel, member, domain:id) to be added: called "a userset"
    #                                   i.t. all "members" of another domain will also be members of this one
    # This relation only allows direct relationships
    define member: [user]
```

Another example: you can take a team (e.g. "team:noob") and:

* Give a specific `user:1` membership in this team: `("user:1", "member", "team:noob")`
* Say that all users are members of "noob" by default: `("user:*", "member", "team:noob")` (*public access*)
* Make all members of a team also have "member" of a super-team (hierarchy): `("team:pro#member", "member", "team:noob")` (*usersets*)

```openfga
type team
  relations
    define member: [user, user:*, team#member]
```

```console
$ export FGA_STORE_ID (fga store create --model Model.fga | jq -r .store.id)
$ fga tuple write user:1 member team:noob
$ fga tuple write user:2 member team:pro
$ fga tuple write team:pro#member member team:noob
$ fga query check user:2 member team:noob  # through team membership
{ "allowed":true }
```

A relation can reference other relations:
this means that you implicitly get it together with another one:

```openfga
type document
  relations
    # A user can directly get this relation
    # Syntax: "define <relation-name>: who can get it"
    define editor: [user]

    # A user implicitly gets this relation if they are an editor
    define viewer: [user] or editor

    # A user cannot be directly assigned this relation: it must be inherited
    define can_rename: editor
```

```console
$ fga tuple write user:1 editor document:A
$ fga query check user:1 viewer document:A
$ fga query check user:1 can_rename document:A
{ "allowed":true }
```

You can also reference relations on related objects.
The syntax is `X from Y` (`Y` is called *tupleset*) and requires that:
* the other object is related to the current object as Y
* the user is related to another object as X

This relation is transitive:
* If a user is related as "viewer" to object "root",
* and object "root" is related to "folder" as "parent",
* then the user is related to "folder" as "viewer"

```openfga
type user

type folder
  relations
    # Who can get a "viewer" on a folder?
    # - a user, directly: (user:id, viewer, folder:id)
    # - all "viewers" of another folder can be added to also be viewers of this one: i.e. users
    define viewer: [user, folder#viewer]

type document
  relations
    # Another folder can be "parent_folder" for this one
    define parent_folder: [folder]

    # Who can be a "viewer" on a document?
    # - a user, directly
    # - all "viewers" from the parent folder: i.e. users:
    #   a folder relates to this document as "parent_folder", and all users that have "viewer" on it
    # Here: "from X" is called "tupleset"
    define viewer: [user] or viewer from parent_folder
```

```console
$ fga tuple write user:1 viewer folder:root
$ fga tuple write folder:root#viewer viewer folder:root/1
$ fga query check user:1 viewer folder:root/1
{ "allowed":true }

$ fga tuple write folder:root parent_folder document:README
$ fga tuple write folder:root/1 parent_folder document:one
$ fga query check user:1 viewer document:one
{ "allowed":true }
```

Unions (`or`) and intersections (`and`) and exclusions (`but not`):

```openfga
type document
  relations
    # relation "viewer": added directly, or through the "editor" tupleset
    define viewer: [user] or editor

type document
  relations
    # relation "viewer": authorized and through editor tupleset
    # - the userset of all users related to the object as "authorized_user"
    # - the userset of all users related to the object as "editor"
    # So "user:anne" must have both relations to become a "viewer"
    define viewer: authorized_user and editor

type document
  relations
    # Any user directly, except for those explicitly blocked
    define viewer: [user] but not blocked
```

**NOTE**: Relations with 'and' or 'but not' are particularly expensive to evaluate when listing!

Example:

```openfga
type user

type domain:
  relations
    define member: [ user, domain#member ]

type folder
  relations
    define can_share: writer
    define owner: [user, domain#member] or owner from parent_folder
    define parent_folder: [folder]
    define viewer: [user, domain#member] or writer or viewer from parent_folder
    define writer: [user, domain#member] or owner or writer from parent_folder

type document
  relations
    define can_share: writer
    define owner: [user, domain#member] or owner from parent_folder
    define parent_folder: [folder]
    define viewer: [user, domain#member] or writer or viewer from parent_folder
    define writer: [user, domain#member] or owner or writer from parent_folder
```

Example: users with the "viewer" relationship to a certain doc are any of:

* users directly related to this doc
* users related to this doc as "editor"
* the set of "viewers" of a doc (users that have "viewer" relationship to that doc) that has "parent" relationship to this one


```openfga
model
  schema 1.1

type doc
  relations
    define viewer: [user] or editor or viewer from parent
```












# CLI
Docker:

```console
$ docker pull openfga/openfga
$ docker run -p 8080:8080 -p 3000:3000 openfga/openfga run
```

Go:

```console
$ go install github.com/openfga/openfga/cmd/openfga
$ ./openfga run
```

Open playground: <http://localhost:3000/>

The first time you run or update:

```console
$ openfga migrate \
    --datastore-engine postgres \
    --datastore-uri 'postgres://postgres:password@postgres:5432/postgres?sslmode=disable'
$ openfga run \
    --datastore-engine postgres \
    --datastore-uri 'postgres://postgres:password@postgres:5432/postgres?sslmode=disable'
```

Authentication: no authentication by default; but you can configure an `Authorization: Bearer`
using pre-shared keys: with env or config:

```env
OPENFGA_AUTHN_METHOD=preshared
OPENFGA_AUTHN_PRESHARED_KEYS=key1,key2
```

Using CLI with Go:

```console
$ go install github.com/openfga/cli/cmd/fga@latest
$ fga
$ fga store create --name=test
$ export FGA_STORE_ID=$(fga store create --model Model.fga | jq -r .store.id)

$ fga model get --field id --format json
{ "id":"01HPJ8JZV091THNTDFE2SFYNNJ" }
```

or Docker:

```console
$ docker run --rm -it --network=openfga -e FGA_API_URL=http://openfga:8080  openfga/cli
```
















Modelling Guidelines
====================

Use Direct Access when:

* Users given direct access to an object
* Represent facts about related objects
* Feature flags

User Groups:

* Grant permissions to an organization
* Block users
* Share with a team
* Grant to followers only
* Restrict users in a certain locale

Roles and Permissions:

* Roles are assigned to users or a group of users
* Permissions allow users to access certain objects based on their specific roles
* Grant someone a role that allows them do something on an object
* use `can_do_something` format for permissions

Parent-Child relations:

* Indicate that user's relationships with one object may influence the user's relationship with another object.
* Let permissions propagate to children (can edit parent => can edit children)
* Example: managers of an employee can approve his requests
* Example: organization admin automatically have edit access on all repositories
* Example: users who subscribed to a plan get access to all of its features

Blocklists:

* Exclusions: block users or groups from accessing
* Block from following, like on social media
* Prevent them from sharing
* Reduce a users's access (e.g. when they're guests)

Public Access:

* Grant every user in the system access
* Sharing something publicly
* A public poll where everyone can vote

Multiple Restrictions:

* Define permissions based on roles: `can_delete: writer and member from owner`
* Requiring multiple authorizations to pass

Custom Roles ([see here](https://openfga.dev/docs/modeling/custom-roles)):

* Allow businesses to create custom roles in their space
* Create arbitrary sets of roles with different permissions

Conditions ([see here](https://openfga.dev/docs/modeling/conditions)):

* Build more complex authz scenarios like ABAC
* Temporal access: limit access within a window of time
* IP whitelists for geo-fencing
* Usage-based/feature-based (entitlements)
* Resource attribute-based
* Uses Google CEL expression language

Contextual Tuples ([see here](https://openfga.dev/docs/modeling/contextual-time-based-authorization) and [here](https://openfga.dev/docs/modeling/organization-context-authorization)):

* Access based on the context of the request
* Temporarily Elevated access
* Assume role and do something once
* Claims from JWT: don't have to write them
* Time-Based Authorization
* Contextual authorization: location, ip
* Example: employee only have access when using the corporate VPN (internal IP)
* Example: only during office hours
* Example: only when a specific "current" workspace is selected



## Modular Models

fga.mod:

```openfga
schema: '1.2'
contents:
  - core.fga
  - issue-tracker/projects.fga
  - issue-tracker/tickets.fga
  - wiki.fga
```

Each file is separate, but they can reference one another.

```openfga
module core

type user

type organization
  relations
    define member: [user]
    define admin: [user]

type group
  relations
    define member: [user]
```

Write them all:

```console
$ fga model write --store-id=$FGA_STORE_ID --file fga.mod
```

## Advanced Use-Cases

### Google Drive

Details: <https://openfga.dev/docs/modeling/advanced/gdrive>

```openfga
model
  schema 1.1

type user

type document
  relations
    define owner: [user, domain#member] or owner from parent
    define writer: [user, domain#member] or owner or writer from parent
    define commenter: [user, domain#member] or writer or commenter from parent
    define viewer: [user, user:*, domain#member] or commenter or viewer from parent
    define parent: [document]

type domain
  relations
    define member: [user]
```

### GitHub

Details: <https://openfga.dev/docs/modeling/advanced/github>

```openfga
model
  schema 1.1

type user

type repo
  relations
    define admin: [user, team#member, organization#member] or repo_admin from owner
    define maintainer: [user, team#member, organization#member] or admin
    define writer: [user, team#member, organization#member] or maintainer or writer from owner
    define triager: [user, team#member, organization#member] or writer
    define reader: [user, team#member, organization#member] or triager or reader from owner
    define owner: [organization]

type organization
  relations
    define owner: [organization]
    define repo_admin: [user, team#member, organization#member]
```

### Slack

Details: <https://openfga.dev/docs/modeling/advanced/slack>

```openfga
model
  schema 1.1

type user

type workspace
  relations
    define legacy_admin: [user]
    define channels_admin: [user] or legacy_admin
    define member: [user] or channels_admin or legacy_admin
    define guest: [user]

type channel
  relations
    define parent_workspace: [workspace]
    define writer: [user, workspace#legacy_admin, workspace#channels_admin, workspace#member, workspace#guest]
    define viewer: [user, workspace#legacy_admin, workspace#channels_admin, workspace#member, workspace#guest] or writer
```

### IoT

Details: <https://openfga.dev/docs/modeling/advanced/iot>

```openfga
model
  schema 1.1

type user

type device
  relations
    define it_admin: [user, device_group#it_admin]
    define security_guard: [user, device_group#security_guard]
    define live_video_viewer: it_admin or security_guard
    define recorded_video_viewer: it_admin or security_guard
    define device_renamer: it_admin

type device_group
  relations
    define it_admin: [user]
    define security_guard: [user]
```

### Entitlements (a/k/a subscription plans or feature flags)

Details: <https://openfga.dev/docs/modeling/advanced/entitlements>

```openfga
model
  schema 1.1

type user

type feature
  relations
    define associated_plan: [plan]
    define access: subscriber_member from associated_plan

type plan
  relations
    define subscriber: [organization]
    define subscriber_member: member from subscriber

type organization
  relations
    define member: [user]
```





## Search with Permissions

Details: <https://openfga.dev/docs/interacting/search-with-permissions>

There are three options:

1. Search, then check & filter out
2. Listen to the changes API and create a local index to do an intersection of two sets.
   Flatten & expand the changes.
3. Build a list of IDs, then search







# Testing Models

Create a `.fga.yml` file.

```yaml
name: Model Tests # optional


# model_file: ./model.fga # you can specify an external .fga file, or include it inline
# If you are using Modular Models, you need to use the fga.mod as the model_file.
model: |
  model
    schema 1.1

  type user

  type organization
     relations
       define member : [user]
       define admin : [user with non_expired_grant]

   condition non_expired_grant(current_time: timestamp, grant_time: timestamp, grant_duration: duration) {
     current_time < grant_time + grant_duration
  }

# tuple_file: ./tuples.yaml # you can specify an external file, include it inline, or both
tuples:

   # Anne is a member of the Acme organization
  - user: user:anne
    relation: member
    object: organization:acme

  # Peter has the admin role from February 2nd 2024 0AM to 1AM
  - user: user:peter
    relation: admin
    object: organization:acme
    condition:
      name: non_expired_grant
      context:
        grant_time : "2024-02-01T00:00:00Z"
        grant_duration : 1h

tests:
  - name: Test
    check:
      - user: user:anne
        object: organization:acme
        assertions:
          member: true
          admin: false

      - user: user:peter
        object: organization:acme
        context:
          current_time : "2024-02-01T00:10:00Z"
        assertions:
          member: false
          admin: true


    list_objects:
      - user: user:anne
        type: organization
        assertions:
            member:
                - organization:acme
            admin: []

      - user: user:peter
        type: organization
        context:
          current_time : "2024-02-01T00:10:00Z"

        assertions:
            member: []
            admin:
                - organization:acme
    list_users:
      - object: organization:acme
        user_filter:
          - type: user
        context:
          current_time : "2024-02-02T00:10:00Z"
        assertions:
            member:
              users:
                - user:anne
            admin:
              users: []

```

Now run it:

```console
$ fga model test --tests <filename>.fga.yaml
```





