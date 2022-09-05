# Method Syntax

Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always ```self```, which represents the instance of the struct the method is being called on.

To define the function within the context of a struct, we start ```impl``` block.
Everything within this ```impl``` block will be associated with the struct