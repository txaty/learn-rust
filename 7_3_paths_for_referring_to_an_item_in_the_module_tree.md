# Paths for Referring to an Item in the Module Tree

A path can take two forms:

- An absolute path is the full path starting from a crate tool; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal ```crate```.
- A relative path starts from the current module and uses ```self```, ```super```, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons.

## Starting Relative Paths with ```super```

We can construct relative paths that begin in the parent module, rather than current module or the crate root, by using ```super``` at the start of the path.
This is like starting a filesystem path with the ```..``` syntax.

## Making Structs and Enum Public

If we use ```pub``` before a struct definition, we make the struct public, but the struct's fields will still be private.

In the contrast, if we make an enum public, all of its variants are then public.
We only need the ```pub``` before the ```enum``` keyword.
