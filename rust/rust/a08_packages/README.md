## Packages, Creates, Modules, Paths

Module system includes:

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

A *crate* is the smallest amount of code that Rust compiler considers at a time.
A single source file is a crate. 

A *crate* can come in one of two forms:

* a *binary crate*: programs that compile into an executable. It must have a `fn main()`
* a *library crate*: define functionality to be shared with multiple projects.
  When Rustaceans say "create", they mean library crate.

The *crate root* is a source file that the Rust compiler starts from.
It makes up the root module of your crate.

A *package* is a bundle of one or more crates: it has a `Cargo.toml` file that describes 
how to build those crates. A package can contain many binary crates, but at most one library crate.

In `Cargo.toml`, there's no reference to `src/main.rs`: cargo follows a convention that this file is
the crate root of a binary crate, named after the package.

If a package contains both `src/main.rs` and `src/lib.rs`, it has two crates: a binary crate, and a library crate.
A package can have multiple binary crates by placing files in the `src/bin` directory: 
each file will be a separate binary crate.

### Modules Cheat Sheet

How modules work:

* When compiling a crate, the compiler first looks in the crate root file (usually `src/main.rs` or `src/lib.rs`).
* In the crate root file, you can declare new modules:
  
  ```rust
  mod garden;
  ```

  The compiler will look for the module's code in: `mod garden { ... }`, or in `src/garden.rs`, or in `src/garden/mod.rs`.

* A submodule can be defined as `mod vegetables { ... }`, or in `src/garden/vegetables.rs`, or in `src/garden/vegetables/mod.rs` (older style).
* Module code is private by default. Use `pub mod` to declare a module public.
* You can refer to submodule types as `crate::garden::vegetables::Asparagus`.
* Bring types into scope with `use crate::garden::vegetables::Asparagus` to reduce typing.

