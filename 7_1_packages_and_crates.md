# Packages and Crates

A crate is the smallest amount of code that the Rust compiler considers at a time.
Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.

A crate can come in one of two forms: a binary crate or a library crate.
Binary crates are programs you can compile to an executable that you can run, such as a command-line program of a server.
Each must have a function called ```main``` that defines what happens when the executable runs.

Library crates don't have a ```main``` function, and they don't compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.

The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

A package is a bundle of one or more crates that provides a set of functionality.
