# Method Syntax

Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always ```self```, which represents the instance of the struct the method is being called on.

To define the function within the context of a struct, we start ```impl``` block.
Everything within this ```impl``` block will be associated with the struct.

Often, but not always, whn we give method with the same name as a field we want it only return the value in the field and do nothing else.
Method like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do.
Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type's public API.

## What's the -> Operator?

Rust has a feature called automatic referencing and dereferencing.
When you call a method with ```object.something()```, Rust automatically adds in ```&```, ```&mut```, or ```*``` so ```object``` matches the signature of the method.

## Associate Functions

All functions defined within an ```impl``` block are called associated functions because they're associated with the type named after the ```impl```.
We can define associated functions that don't need an instance of the type to work with.
