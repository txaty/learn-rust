# Data Types

Rust is statically typed language, which means that it must know the types of all variables at compile time.
The compiler can usually infer what type we want to use based on the value and how we use it.

## Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

### Integer types

An integer is a number without a fractional component.

Built-in integer types:

|Length|Signed|Unsigned|
|:----:|:----:|:------:|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

Signed numbers are stored using two0s complement representation.

Additionally, the ```isze`` and ```usize``` types depend on the architecture of the computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.

In debug mode, integer overflow triggers panic, in release mode, it is handled by two's complement wrapping.

To explicitly handle the possibility of overflow, you can use the methods by standard library for primitive numeric types:

- Wrap in all modes with the ```wrapping_*``` methods, such as ```wrapping_add```
- Return the ```None``` value if there is overflow with the ```checked_*``` methods
- Return the value and a boolean indicating whether there was overflow with the ```overflowing_*``` methods
- Saturate at the value's minimum or maximum values with ```saturating_*``` methods.

### Floating-Point Types

All floating point types are signed.

### The Character Type

We specify ```char``` literals with single quotes, as opposed to string literals, which use double quotes.

## Compound Types

Compound types can group multiple values into one type.
Rust has two primitive compound types: tuples and arrays.

### The Tuple Type

Tuples have a foxed length: once declared, they cannot grow or shrink in size.

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.
We can also access a tuple element directly by using a period (```.```) followed by the index of the value we want to access.

The tuple without any values  has a special name, unit.
This value and its corresponding type are both written ```()``` and represent an empty value or an empty return type.
Expressions implicitly return the unit value if they don't return any other value.

### The Array Type

Unlike a tuple, every element of an array must have the same type.

Unlike arrays in some other languages, arrays in Rust have a fixed length.

Arrays are useful when you want your data allocated on the stack rather than the heap.

#### Accessing Array Elements

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.

YOu can access elements of an array using indexing.

#### Invalid Array Element Access

When you attempt to access an element using indexing, Rust will check that the index you've specified is less than the array length.
If the index is greater than or equal to the length, Rust will panic.