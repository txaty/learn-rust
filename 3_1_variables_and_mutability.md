## Variables and Mutability

When a variable is immutable, once a value is bound to a name, you can't change that value.

Although variables are immutable by default, you can make then mutable by adding ```mut``` in front of the variable.

## Constants

Constants are values that are bound to a nam and are not allowed to change. But there are some differences:

- You are not allowed to use ```mut``` with constants. Constants are declared using ```const``` keyword.
- Constants can be declared in any scope, including global scope.
- Constants may be set only to a constant expression, not the result ofa a value that could only be computed at runtime.

Constants are valid for the entire time a program runs, within the scope they were declared in.
This property makes constants useful for values in the application domain that multiple parts of the program might need to know about.

## Shadowing

We can shadow a variable by using the same variable's name and repeating the use of the ```let``` keyword.
