# Protocol Buffers

## Introduction

A serialization format for packets of typed, structured data, that are up to a few megabytes in size. 
Suitable for: both network traffic and long-term data storage: 

Language support: 
* Direct support for: C, C#, Java, Kotlin, Objective-C, PHP, Python, Ruby
* Plugin support for: Dart, Go
* Other languages: third-party modules

Sample message:

```protobuf
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```

generated code:

```java
Person john = Person.newBuilder()
    .setId(1234)
    .setName("John Doe")
    .setEmail("jdoe@example.com")
    .build();
output = new FileOutputStream(args[0]);
john.writeTo(output);
```

Fields can be added and removed without breaking existing services: without invalidating existing data or requiring code to be updated!

As long as you follow some simple practices, it's going to be forward compatible: old code will read new messages without issues,
ignoring newly added fields. To the old code, fields that were deleted will have their default value, and deleted repeated fields will be empty.

## Syntax

A field has:

* A field can be: `optional`, `repeated`, `singular` (proto3).
* Then goes the type: scalar type (int, bool, float), or complex: `message` (nested structure), `enum` (set of values), `oneof` (at most one field will be set), `map` (key/value pairs)
* Field number. A field reserves a number.

Common types: `Duration` (seconds), `Timestamp` (int), `Interval` (two timestamps), `Date`, `Month`, `DayOfWeek`, `TimeOfDay`, `Money` (amount + currency), `LatLng` (lat/long pair), `PostalAddress` (string), `Color` (RGBA).

## Proto3 syntax

### Message and Fields
Example search request:

```protobuf
// "proto2" is a default. Override.
syntax = "proto3"

// Message with 3 fields
message SearchRequest {
    // Field numbers: unique numbers that identify fields. Should not be changed.
    // Use 1..15 for frequent fields (use only 1 byte)
    string query = 1;
    int32 page_number = 2;
    int32 results_per_page = 3;
}

message SearchResponse {
    /* ... */
}
```

*Field numbers* identify fields in the [Message Binary Format](https://developers.google.com/protocol-buffers/docs/encoding).

Field numbers 1..15 take one byte to encode the field number and the type. Field numbers 16..2047 use two bytes.

Hint: reserve 1..15 for ver frequently occurring message elements. Leave some room for future frequent elements.

Message fields can be one of:

* `singular`: a message can have 0 or 1 of this field.
* `optional`: same as `singular`, except that you can check to see if the value was explicitly set.
* `repeated`: this field can be repeated 0+ times. An array. The order will be preserved.
* `map`: key/value pairs

When unserialized:

* with a `singular` field, you cannot determine whether it was parsed from the wire or gotten the default value.
* with an `optional` field, you can check to see if the value was explicitly set

When serialized to the wire:

* a `singular` field is serialized to the wire unless it is the default value
* an `optional` field is serialized if it was explicitly set

When a `singular` field is present, you cannot determine whether it was parsed from the wire or gotten the default value. 
It will be serialized to the wire unless it is the default value. See more in [Field presence](https://github.com/protocolbuffers/protobuf/blob/main/docs/field_presence.md)

An `optional` field is either 1) set and contains a value that was explicitly set, or 2) the field is unset, and will return the default value.
An unset field will not be serialized to the wire.

Here's how you indicate that some field numbers have been used by deleted fields:

```protobuf
message Foo {
    reserved 2, 15, 9 to 11; // indicate field numbers that cannot be used
    reserved "foo", "bar"; // indicate field names (used when mapping to JSON serialization)
}
```

## Scalar Value Types

Floats: `float`, `double` (64)

Integers: `int32`, `int64` (inefficient for negative numbers; use `sint` instead); 
signed `sint32`, `sint64` ; Unsigned: `uint32`, `uint64` (more efficient).

Big integers: `fixed32`, `fixed64`, `sfixed32`, `sfixed64` (more efficient when values are often greater than 2^28)

NOTE: `int` types use variable-length encoding and thus are inefficient with negative numbers. Use `sint` explicitly.

NOTE: `fixed` types are more efficient when your integers are always large: they use fixed 4-byte/8-byte encoding rather than variable-length encoding.

Other: `bool`, `string` (UTF-8 or ASCII), `bytes` (up to 2^32) 

Default values are type-specific, like in Go: `string ""`, `bool false`, `int 0`, `enum (value 0)`, `message (not set)`.

## Enums

```protobuf
enum Corpus {
  // You can define aliases: assign the same value to different enum constants.
  // All aliases are valid, but the first value is used when serializing
  option allow_alias = true;  

  CORPUS_UNSPECIFIED = 0;  // the default value. Must have. Must be first.
  CORPUS_UNIVERSAL = 1;
  CORPUS_WEB = 2;
  CORPUS_IMAGES = 3;
  CORPUS_LOCAL = 4;
  CORPUS_NEWS = 5;
  CORPUS_PRODUCTS = 6;
  CORPUS_VIDEO = 7;

  reserved 8, 15, 9 to 11, 40 to max; // indicate that some values cannot be used
  reserved "FOO", "BAR"; // indicate that some names cannot be used.
}
message SearchRequest {
  string query = 1;
  int32 page_number = 2;
  int32 result_per_page = 3;

  // enum field
  Corpus corpus = 4;
}
```

NOTE: enum values use variant encoding on the wire, so negative numbers are inefficient and thus not recommended.

Deserialization of unrecognized enum values is language-dependent, but in general, these values will be preserved in the message.
With language that support open enum types with values outside the range of specified symbols (C++ and Go), the unknown enum value
is simply stored as its underlying integer representation.

## Nested Messages

Use other messages as field types:

```protobuf
message SearchResponse {
  repeated Result results = 1;
}

message Result {
  string url = 1;
  string title = 2;
  repeated string snippets = 3;
}
```

Or define nested types:

```protobuf
message SearchResponse {
  message Result {
    string url = 1;
    string title = 2;
    repeated string snippets = 3;
  }
  repeated Result results = 1;
}
```

Now the nested type should be referred to as `SearchResponse.Result`.

## Imports

```protobuf
import "myproject/other_protos.proto";
```

## Updating a Message Type

Rules to update message types without breaking any of your existing code:

* Don't change the field numbers
* Add new fields. Old binaries will simply ignore them
* Remove old fields. Reserve their field numbers, or renamed them to "obsolete_"
* `int32/64`, `uint32/64` and `bool` are all compatible: you can change the type without breaking code
* `sint32` and `sint64` are compatible with each other but not with other int types
* `string` and `bytes` are compatible as long as the bytes are valid UTF-8
* Embedded messages are compatible with `bytes` if the bytes contain an encoded version of the message
* `fixed32` is compatible with `sfixed32`, and `fixed64` with `sfixed64`
* `singular` fields are compatible with `repeated` fields for `string`, `bytes`, `message` fields
* `singular` fields are NOT compatible with `repeated` numeric fields: they can be serialized in the *packed* format and not parsed correctly
* `enum` is compatible with `int32/64`, `uint32/64` (but be aware that client code may treat the value differently)
* Changing a single `optional` field into a member of a `oneof` is safe. 
* Changing a single field `oneof` to an `optional` field is safe.
* Moving multiple fields into a new `oneof` may be safe if you are sure that no code sets more than one at a time.
* Moving any fields into an existing `oneof` is not safe. 

## Any

The `Any` message type allows to use any message type, identified by its globally unique identifier URL that resolves the message type:

```protobuf
import "google/protobuf/any.proto";

message ErrorStatus {
  string message = 1;
  repeated google.protobuf.Any details = 2;
}
```

Language implementations will support runtime helpers to pack/unpack `Any` values.

With `proto2`, it was called "extensions".

## Oneof

In cases where at most one field will be set at the same time.
This saves some memory by sharing.

You can use `case()` of `WhichOneOf()` to check which value is set.

```protobuf
message SampleMessage {
  oneof test_oneof {
    string name = 4;
    SubMessage sub_message = 9;
  }
}
```

Features: 

* Setting a `oneof` field automatically clears all other members
* If multiple values are on the wire, only the last member seen is used
* Cannot be repeated; cannot contain `map` and `repeated` fields.
* If you set a field to its default value, the "case" is set, and the default value will be serialized on the wire

Issues:

* If case=`None/NOT_SET`, it could mean that 1) the oneof ahs not been set, or 2) set to an unknown field. There's no way to tell the difference.

## Maps

Key/value map.
Key: any integral (any scalar, but float and bytes) or string type. Enum cannot be a key.
Value: any type, except another map.

```protobuf
message SampleMessage {
    map<string, Project> projects = 3;
}
```

Notes:

* Keys cannot be enums
* Values cannot be maps
* Map fields cannot be repeated
* Map is unordered
* When a duplicate key is encountered, the last value is used
* Implemented as `repeated message { key_type key = 1 ; value_type value = 2; }`

## Packages

You can specify a `package` and refer to its messages:

```protobuf
package foo.bar;
message Open { ... }

// ...

message Foo {
    foo.bar.Open open = 1;
}
```

In Go, the package is used as the Go package name, unless an explicit `go_package` is provided.

In Python, it's ignored, since Python modules are organized according to their location in the file system.

## Defining Services

## JSON Mapping

## Options

## Generation

## File location

## Supported Platforms

