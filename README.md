# accu-rust-talk

## Talk on Rust for the Oxford ACCU

https://www.meetup.com/ACCU-Oxford/events/267447759/

### Wednesday Jan 29, 2020.

This project is a brief introduction to the cool parts of the Rust programming language.

For a more in-depth discussion I recommend rust by example:

https://doc.rust-lang.org/rust-by-example/

And for those curious about the inner working of rust libraries:

https://doc.rust-lang.org/nomicon/

This talk focuses on these six examples that illustrate why rust is the obvious
next choice as a systems langauge after C++.

```
* 1-hello
    Hello world

* 2-hello_lib
    Hello world function and tests

* 3-server
    Super fast HTTP server

* 4-json
    Serialise and deserialise JSON data using procedual macros.

* 5-iterators
    Replace for loops with iterator expressions.

* 6-containers
    The most commonly used containers in Rust.
```

Getting started with Rust:

Go to https://rustup.rs/ and follow the instructions.

You may also need to get the Nightly toolchain to experience
the bleeding edge of Rust.

```
rustup toolchain install nightly
```

The working tool is `cargo` which downloads and installs libraries
very much like `npm` does for `node.js` 

To make a new project:

```
cargo new myproject
```

To make a new library:

```
cargo new mylib --lib
```

If you are using visual studio code, the `RLS` extension generates syntax checking
and tooltips and is strongly recommended. I have used `vim`, but it is not the same!

I also recommend installing `cargo-edit`

```
cargo install cargo-edit
```

This allows you to add dependencies quickly to your `Cargo.toml` file.

For this talk, I also use `cargo-expand`

```
cargo install cargo-expand
```

Which shows expanded macros in all their gory detail.

Rust compiles each library to a single object file which makes
linking much faster.

Code is always compiled from source except for some borrwings from the C
world such as `rusqlite`. Rustacians condsider such borrowings to be an
alien concept and seek to eliminate `C` from the civilised world, hence.

https://github.com/ctz/rustls

And friends.

Rust kernels and complete operating systems are in the pipeline, too.

Rust has been the **most loved programming langauge** on `Stack Overflow`
since 2016.

