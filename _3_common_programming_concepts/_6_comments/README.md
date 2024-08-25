# Comments

Comments are essential in making your code more understandable, especially for others who may read or maintain it later. They provide context, explain complex logic, or note important considerations. In Rust, comments are ignored by the compiler meaning they do not affect the behavior or performance of the code. Let's explore the different ways you can write comments in Rust.

## Single-Line Comments

The most common type of comment in Rust is a single-line comment. It begins with two forward slashes (`//`) and extends to the end of the line. Here's an example:

```rust
// This is a single-line comment
fn main() {
    println!("Hello, world!");
}
```

In this example, the comment provides a brief note above the `main` function. The compiler ignores this line, but it's there to help someone reading the code understand what's happening.

## Multi-Line Comments

When you need to write a comment that spans multiple lines, you can use multiple single-line comments:

```rust
fn main() {
    // This is a longer explanation that spans
    // multiple lines. Each line begins with //
    // and continues until the end of the line.
    println!("Hello, world!");
}
```

Alternatively, you can use Rust's block comment syntax, which is similar to C-style comments:

```rust
fn main() {
    /* This is a block comment that spans multiple lines.
       It begins with /* and ends with */. Everything in
       between is ignored by the compiler. */
    println!("Hello, world!");
}
```

Block comments are useful when you need to temporarily comment out chunks of code or when you prefer a different style for longer explanations. However, using single-line comments is often more idiomatic in Rust.

## Inline Comments

You can also place comments at the end of a line of code. These are called inline comments and are useful for brief notes or explanations:

```rust
fn main() {
    let lucky_number = 7; // This is a an inline comment
}
```

In this case, the comment explains what the variable `lucky_number` represents or why it was chosen, without interrupting the flow of the code.

## Comments Above Code

Another common pattern is placing comments on a separate line above the code they are explaining. This makes the code more readable and the comments more noticeable:

```rust
fn main() {
    // This variable stores my lucky number
    let lucky_number = 7;
}
```

By placing the comment on its own line, it’s clear that the comment is referring to the variable `lucky_number`.

## Documentation Comments

Rust also supports a special type of comment for documentation purposes, known as documentation comments. We'll discuss documentation comments in detail later when we cover [Publishing a Crate to Crates.io](https://github.com/nimodb/rust-journey/).

---

Comments are a simple yet powerful tool for making your code clearer and easier to understand. Use them wisely to explain your intentions, especially in complex or non-obvious pars of your code.
