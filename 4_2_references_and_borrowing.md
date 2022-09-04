# References and Borrowing

A reference is like a pointer in that it's an address we can follow to access the data stored at that address;
that data is owned by some other variable.
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

We call the action of creating a reference borrowing.

Just as variable are immutable by default, so are references.
We're not allowed to modify the data that a reference points to.

## Mutable References

We can create a mutable reference with the `&mut` syntax.
Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
The benefit of having this restriction is that Rust can prevent data races at compile time.

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you're trying to track them down at runtime;
Rust prevent this problem by refusing to compile code with data races.

Rust enforces a similar rule for combining mutable and immutable references.

Not that a reference's scope starts from where it is introduced and continues through the last time that reference is used.