# 7.4 Brining Paths into Scope with the ```use``` Keyword

We brining the ```crate::front_of_house::hosting``` module into the scope of the ```eat_at_restaurant``` function so we only have to specify ```hosting::add_to_waitlist``` to call the ```add_to_waitlist``` function in ```eat_at_restaurant```.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Adding ```use``` and a path in a scope is similar to creating a symbolic link in the filesystem.
By adding ```use crate::front_of_house::hosting``` in the crate root, ```hosting``` is now a valid name in that scope, just as though the ```hosting``` module had been defined in the crate root.
Paths brought into scope with ```use``` also check privacy, like any other paths.

```use``` only creates the shortcut or the particular scope in which the ```use`` occur.

## Creating Idiomatic ```use``` Paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

## Providing New Names with the ```as``` Keyword

```rust
use std::io::Result as IoResult;
```

## Re-exporting Names with ```pub use```

WHen we bring a name into scope with the ```use``` keyword, the name available in the new scope is private.
To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine ```pub``` and ```use```.

## Using External Packages

Add the crate name to ```Cargo.toml```.

## Using Nested Paths to Clean Up Large ```use``` Lists

```rust
use std::{cmp::Ordering, io};
```

## The Glob Operator

To bring all public items defined in a path into scope, we can specify that path followed by the ```*``` glob operator.

```rust
use std::collections::*;
```
