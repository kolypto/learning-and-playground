# TypeSpec

* TypeSpec: <https://typespec.io/>
* Playground: <https://typespec.io/playground>
* OpenAPI transition: <https://typespec.io/docs/getting-started/typespec-for-openapi-dev>

Version: 0.61.2 (2024-10)

# Init

```console
$ npm install -g @typespec/compiler
$ tsp init
Generic REST API
Name: mobili
@typespec/http and @typespec/openapi3 selected
$ tsp install
$ tsp compile .
```

.gitignore:

```gitignore
# Default TypeSpec output
/tsp-output/
/dist/

# Dependency directories
/node_modules/
```


# Docker

User from Docker:
<https://github.com/microsoft/typespec/tree/main/docker>


# Example: REST Service

```ts
import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";
import "@typespec/versioning";

using TypeSpec.Http;
using TypeSpec.Versioning;

// A REST service.
// OpenAPI: Top-Level `info`
@service({
    title: "Pet Store",
})

// Top-level info
@info({
  contact: {
    name: "API Support",
    email: "contact@contoso.com",
  },
  license: {
    name: "Apache 2.0",
    url: "https://www.apache.org/licenses/LICENSE-2.0.html",
  },
})

// OpenAPI: top-level `servers`: endpoints you can connect to
@server("https://example.com/", "Single server endpoint")

// Define a top-level namespace: all models and operations will be defined within this namespace.
// OpenAPI: tags
// Subsequent namespaces will require {}
namespace PetStore;

// Model: data structure

model Pet {
    // All fields are required by default
    // OpenAPI: type:integer, format:int64
    @key id: int64; // marked as primary key

    // Validation
    // OpenAPI: `minLength:`
    @minLength(1)
    name: string;

    // Nullable field
    // OpenAPI: nullable:true
    @maxValue(100)
    age: uint8 | null = null; // With default value

    // Enum: referenced type
    // OpenAPI: $ref, enum
    kind: PetType;
}


// Enum type
// OpenAPI: type string, enum
enum PetType {
    // Names: only used in TS
    // Values: actual values in the protocol
    // OpenAPI: names ignored, values become a list
    dog: "dog";
    cat: "cat";
    fish: "fish-dish";
};



// Another namespace
// Namespaced models become `<namespace>.<model>`
// OpenAPI: prefix "extra."
namespace extra {
    // extra.Troll
    model Troll {
        id: int64;
        face: string;
    }

};

// @tag
// OpenAPI: tag
@tag("Gadgets")
// @route: decorator for HTTP route
// OpenAPI: `paths` item
// Without this decorator, operations get default "/"
@route("/ping")
// @get, @post, ...: decorator for HTTP method
// OpenAPI: path method
// Without: operations get default "get"
@get
// Documentation
// Also: @param, @returns, @template
@doc("Docstring")
@doc("""Docstring""")
// op: operationId
// OpenAPI: operation id
op pingping(
    // @path: required path argument. Added even if not mentioned.
    @path strength: int64,
    // @query: required query argument
    // Without: defaults to body json
    @query perPage: uint64 | null,
    // @body: JSON body, unnamed schema (DON'T!)
    @body ball: { size: uint64 }
): {
    // Response declaration: code + corresponding response
    // OpenAPI: responses.200
    @statusCode _: 200;
    // OpenAPI: responses.200.content.application/json.schema, type:array
    @body values: string[];
} | {
    // Another response
    @statusCode _: 404 | 500;
    @body error: NotFoundError | InternalServerError;
};



// @error: indicate that these models are error responses
// These models will go into the `default` response.
@error
model NotFoundError {
    code: "NOT_FOUND";
    message: string;
}
@error
model InternalServerError {
    code: "INTERNAL_SERVER_ERROR";
    message: string;
}


// HTTP @route
// OpenAPI: group of paths
@route("/pets")
// Namespace: allows to apply a decorator to a group of objects
// Namespaces are prepended to OperationID
namespace Pets {
  // Reuse some common parameters
  model CommonParameters {
    @header
    requestId: string;

    @query
    locale?: string; // nullable
  }


  // OpenAPI:
  //    path: /pets/list
  //    operationId: Pets_listPets
  @route("/list")  // sub-path
  @get
  op listPets(...CommonParameters): {  // common parameters
    @statusCode statusCode: 200;
    @body pets: Pet[];
  };

  // @path will auto-append path
  @get
  op getPet(@path petId: int32): {
    @statusCode statusCode: 200;
    @body pet: Pet;
  };

  @post
  op createPet(@body pet: Pet): {
    @statusCode statusCode: 201;
    @body newPet: Pet;
  };

  @put
  op updatePet(@path petId: int32, @body pet: Pet): {
    @statusCode statusCode: 200;
    @body updatedPet: Pet;
  };

  @delete
  op deletePet(@path petId: int32): {
    @statusCode statusCode: 204;
  };
}

```


# Authentication

```ts

// Use predefined auth
// OpenAPI: security: [ BearerAuth ], components.securitySchemes: BearerAuth
@useAuth(BearerAuth | BasicAuth)
op requiresAuth(): {};
```

# Versioning

```ts
import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";
import "@typespec/versioning";

using TypeSpec.Http;
using TypeSpec.Versioning;

@service({
  title: "Pet Store",
})
@server("https://example.com", "Single server endpoint")

// Will generate two OpenAPI files:
// * openapi.1.0.yaml
// * openapi.2.0.yaml
@versioned(Versions)
namespace PetStore;

enum Versions {
  v1: "1.0",
  v2: "2.0",
}

// Indicate that a model or operation was added in a specific version of the API.
@added(Versions.v2)
model Toy {
  id: int32;
  name: string;
}
```

# Custom response models

```ts

model PetListResponse {
  // Code 200
  ...OkResponse;
  // Type parameter: => body: string[]
  ...Body<string[]>;
}

model PetErrorResponse {
  ...BadRequestResponse;
  ...Body<ValidationError>;
}

model ValidationError {}
```


# Enums

Two ways:

```ts
enum Color {
  "red",
  "blue",
  "green",
}

// Or union:

size?: "small" | "medium" | "large" | "x-large";
```



# CSV File

```ts
@post
op csvFile(...CsvBody): CsvBody;

model CsvBody {
    @header contentType: "text/csv";
    @body _: string;
}

```



# Inheritance

```ts
model Pet {
  name: string;
}

model Cat extends Pet {
  meow: int32;
}

model Dog extends Pet {
  bark: string;
}
```

with a discriminator:

```ts
@discriminator("kind")
model Pet {
  name: string;
  weight?: float32;
}
model Cat extends Pet {
  kind: "cat";
  meow?: int32;
}
model Dog extends Pet {
  kind: "dog";
  bark?: string;
}
```




# Define scalars

```ts
scalar ternary;
scalar Password extends string;

// With template parameters
@doc(Type)
scalar Unreal<Type extends string>;
```




# Model `is`

```ts
// All fiels have type <string>
model Person is Record<string> {
  name: string;
}

// A type that is an exact copy
model StringThing is Thing<string>;

```


# Model Template
model Page<Item> {
  size: number;
  item: Item[];
}


# Reuse operator signature


```ts
op Delete(id: string): void;
op deletePet is Delete;
```

or with a template:

```ts
op ReadResource<T>(id: string): T;
```

# Interfaces

Interfaces are useful for grouping and reusing operations:

```ts
interface SampleInterface {
  foo(): int32;
  bar(): string;
}

// Composition
interface C extends A, B {
  c(): string;
}

// Template
interface ReadWrite<T> {
  read(): T;
  write(t: T): void;
}

// Implement as an operation
alias MyReadWrite = ReadWrite<string>;
op myWrite is MyReadWrite.write<int32>;
```


# Named Union

```ts
union Breed {
  beagle: Beagle,
  shepherd: GermanShepherd,
  retriever: GoldenRetriever,
}
```

# Intersection

```ts
alias Dog = Animal & Pet;

// same as

alias Dog = {
  ...Animal;
  ...Pet;
};
```


# String Literal

```ts
alias Str = "Hello World!";
```


