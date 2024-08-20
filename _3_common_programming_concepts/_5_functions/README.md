# 3.5. Functions

Functions are prevalent in Rust code. You've already seen one of the most important function in the language, the `main` function, which is the entry point of many programs. you've also seen the `fn` keyword, which allows you to declare new functions.

Rust follows the _snake case_ naming convention for function and variable names. In snake case, all letters are lowercase, and words are separated by underscores (`_`). This style improves readability and consistency across your codebase.

Here’s a program that contains an example function definition:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

- Defining a Function:

  - The function `another_function` is defined using the `fn` keyword followed by the function name and parentheses `()`.
  - The function body is enclosed in curly brackets `{}`.

- Calling a Function:
  - Functions are called by their name followed by parentheses. In this case, `another_function()` is called inside the `main` function.
  - Rust executes functions in the order they are called, regardless of the order in which they are defined in the code.

When you run the above program, it executes the `main` function, which first prints `Hello, world!` and the call `another_function`, which prints `Another function.`.

```bash
$ cargo run
    Compiling functions v0.1.0 (file:///projects/functions)
     Finished dev [unoptimized + debuginfo] target(s) in 0.28s
      Running `target/debug/functions`
Hello, world!
Another function.
```

This example illustrates how functions can be defined and used to organize and modularize your code. Whether you define a function before or after the `main` function doesn’t matter as long as it’s in the same scope. Rust will compile and execute your code correctly. Functions are essential in managing complexity and reusing code efficiently in Rust programs.
