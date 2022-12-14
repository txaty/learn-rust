# Defining Modules to Control Scope and Privacy

## Modules Cheat SHeet

### Start from the crate root

When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.

### Declaring modules

In the crate root file, you can declare new modules.

### Declaring submodules

In any file other than the crate root, you can declare submodules.

### Paths to code in modules

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code.

### Private vs public

Code within a module is private from its parent modules by default.
To make a module public, declare it with ```pub mod``` instead of ```mod```.
To make items within a public module public as well, use ```pub``` before their declaration.

### The ```use``` keyword

Within a scope, the ```use``` keyword creates shortcuts to items to reduce repetition of long paths.

## Grouping Related Code in Modules

Modules let us organize code within a crate for readability and easy reuse.
Modules also allow us to control the privacy of items, because code within a module is private by default.
Private items are internal implementation details not available for outside use.