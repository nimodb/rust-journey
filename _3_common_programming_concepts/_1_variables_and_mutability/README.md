# 3.1 Variables and Mutability

This section cover the concept of the variables and mutability, using a simple example to demonstrate how Rust enforces immutability by default and how you can opt into mutability when you needed.

## Immutability in Rust

- In Rust, variables are immutable by default. This means that once a value is bound to a variable, it cannot be changed, which helps prevent bugs and makes the code safer.

```rust
fn main() {
    let x = 5;
    println!("The value of x is {x}");
    // Uncommenting the next line will cause a compilation error
    // x = 6;
    // println!("The value of x is: {x}");
}
```

- If you run this code by saving it in a file and running `cargo run`, you'll encounter a compile-time error because `x` is immutable. The error message will indicated that you're attempting to assign a new value to an immutable variable.

## Making a Variable Mutable

- If you need to change the value of a variable, you must explicitly opt into mutability by using the `mut` keyword. This makes your intentions clear in the code.

```rust
fn main() {
    let mut y = 5;
    println!("The value of y is {y}");
    y = 6;
    println!("The value of y is {y}");
}
```

- Now when you run the program, it will compile successfully, and you'll see the output:

```csharp
The value of y is: 5
The value of y is: 6
```

- By using `mut`, Rust allows you to change the value of `y` from 5 to 6.
