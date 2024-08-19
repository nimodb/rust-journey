# 3.2 Constants

In Rust, constants are similar to immutable variables, but there are important differences. Constants are values that are bound to a name and cannot be changed after being a declared. They are always immutable and are declared using the `const` keyword. Unlike variables declared with `let`, constants must have their type explicitly annotated.

## Key Differences Between Constants and Variables

- Immutability: Constants are always immutable. You cannot use the `mut` keyword with constants.
- Declaration: Constants are declared with the `const` keyword instead of `let`.
- Type Annotation: Constants require an explicit type annotation.
- Scope: Constants can be declared in any scope, including global scope, making them accessible throughout the program.
- Constant Expression: Constants can only be assigned values that are constant expressions, which means they must be able to be evaluated at compile time.

## Benefits of Using Constants

- Code Clarity: Naming constants makes the code easier to read and understand.
- Maintainability: If you need to update a value used throughout your program, you only need to change it in one place.
- Safety: Constants ensure that critical values do not change unexpectedly during the execution of your program.
