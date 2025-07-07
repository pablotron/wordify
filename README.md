# Wordify

Generate a random phrase from a [JSON][] template.

Example:

```sh
# run wordify using `examples/hi.json` template
$ wordify < examples/hi.json
hi bob!

# run wordify again to produce different output
$ wordify < examples/hi.json
hello paul!
```

## Templates

A template contains a string with parameter blocks and a set of values
for each parameter.  The parameters in the template string are marked
with `{{` and `}}`.

Wordify finds parameter blocks in the template string and replaces each
one with a randomly-chosen parameter value.

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

A [JSON schema][] for Wordify templates is available in [`schema.json`][schema].

## Build

To build Wordify using [Cargo][]:

```sh
# create release binary in `target/release/wordify`
$ cargo build --release
```

To create a debug build:

```sh
# create debug build (used for development)
$ cargo build
```

To create a statically-linked binary:

1. Use [rustup][] to add the `musl` target for your architecture.
   Examples: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`.
2. Run `cargo build` with the given target.

Example:

```sh
# install static target with rustup
$ rustup target add $(arch)-unknown-linux-musl

# create static linux binary
# (saved to `target/$(arch)-unknown-linux-musl/release/wordify`)
$ cargo build --release --target $(arch)-unknown-linux-musl

# verify that binary is statically-linked
$ ldd target/$(arch)-unknown-linux-musl/release/wordify
        statically linked

# run static binary with `examples/hi.json` template
$ target/$(arch)-unknown-linux-musl/release/wordify < examples/hi.json
hi paul!
```

## Install

Run `cargo install` to build and install the `wordify` binary:

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

## Container

Wordify can be built and run as a container image with [Podman][] or
[Docker][].

To build the container image:

```sh
# build wordify container image
$ podman build -t wordify:latest .
```

To run the container image:

```sh
# run `wordify:latest` image twice with `examples/hi.json` template
$ podman run --rm -i wordify:latest < examples/hi.json
howdy alice!
$ podman run --rm -i wordify:latest < examples/hi.json
hi bob!
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
[podman]: https://podman.io
  "Podman container orchestrator."
[schema]: ./schema.json
  "Wordify template JSON schema."
[docker]: https://docker.com/
  "Docker container orchestrator."
