# Cargo

Cargo is Rust's build system and package manager.

## Check installation

```bash
$ cargo --version
```

## Create a new project using Cargo

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

## Building and running a Cargo project

```bash
$ cargo build
```

More conveniently,

```bash
$ cargo run
```

Cargo also provides a command to quickly check the code to make sure it compiles but doesn't produce an executable:

```bash
$ cargo check
```

This command is suitable for periodically checking while writing Rust code.
