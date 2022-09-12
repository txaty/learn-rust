# Paths for Referring to an Item in the Module Tree

A path can take two forms:

- An absolute path is the full path starting from a crate tool; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal ```crate```.
- A relative path starts from the current module and uses ```self```, ```super```, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons.
