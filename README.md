# Wordify

Generate a random phrase from a [JSON][] template.

Example:

```sh
# run wordify with `examples/hi.json` template
$ wordify < examples/hi.json
hi bob!
```

## Templates

A template contains a string and parameters.  Wordify finds parameters
in the template string and replaces them with a randomly-chosen
parameter value.

Here is an example template:

```json
{
  "string": "{{greet}} {{name}}!",
  "params": {
    "greet": ["hi", "hello", "howdy"],
    "name": ["alice", "bob", "paul"]
  }
}
```

Example output produced by this template:

```sh
# run wordify with `examples/hi.json` template
$ wordify < examples/hi.json
hello paul!
```

Template properties:

- `string`: Template string.  Parameters are enclosed in `{{..}}`.
  Example: `"hello {name}"`.
- `params`: Map of parameter name to replacement values.  Example:
  `{ "fruit": ["apple", "banana", "grape"] }`.

A [JSON schema][] for the templates is available in `schema.json`.

## Build

Use `cargo build`.  Examples:

```sh
# create release build in `target/release/wordify`
$ cargo build --release

# create debug build (used for development)
$ cargo build
```

You can also compile a static binary by adding the necessary with
[rustup][], like so:

```sh
# install static target with rustup
$ rustup target add x86_64-unknown-linux-musl

# create static x86-64 linux binary
# (saved to `target/x86_64-unknown-linux-musl/release/wordify`)
$ cargo build --release --target x86_64-unknown-linux-musl
```

## Install

Run `cargo install` to build and install the `wordify` binary.

Example:

```sh
$ cargo install --path .
```

## Tests

Run `cargo test` to run the unit tests:

```sh
$ cargo test
```

Run `cargo clippy` to run the linter:

```sh
$ cargo clippy
```

[mad libs]: https://en.wikipedia.org/wiki/Mad_Libs
  "Mad Libs"
[json]: https://en.wikipedia.org/wiki/JSON
  "JavaScript Object Notation (JSON)"
[cargo]: https://doc.rust-lang.org/stable/cargo/
  "cargo: Rust package manager"
[rustup]: https://rustup.rs/
  "rustup: Rust installer."
[json schema]: https://json-schema.org/
  "JSON schema"
