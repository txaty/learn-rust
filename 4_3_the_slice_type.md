# The Slice Type

Slice let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

## String Slices

A string slice is a reference to part of a string.

String slice range indices must occur at valid UTF-8 character boundaries.
