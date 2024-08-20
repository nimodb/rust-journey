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

Hello, world!
Another function.
```

This example illustrates how functions can be defined and used to organize and modularize your code. Whether you define a function before or after the `main` function doesn’t matter as long as it’s in the same scope. Rust will compile and execute your code correctly. Functions are essential in managing complexity and reusing code efficiently in Rust programs.

## Parameters

Functions in Rust can take parameters, which are special variables that allow the function to operate on different inputs each time it’s called. When a function is defined with parameters, you provide specific values (arguments) when calling the function. Although "parameter" and "argument" are technically different, they are often used interchangeably.

In this version of `another_function` we add a parameter:

```rust
fn main() {
    print_value(5);
}

fn print_value(x: i32) {
    println!("The value of x is: {x}");
}
```

The function `print_value` is defined to take one parameter, `x`, of type `i32`. When calling `print_value(5);`, the argument `5` is passed to the function binding it to `x`.

```bash
$ cargo run

The value of x is: 5
```

This output is produced because the value `5` replaces `{x}` in the `println!` macro.

### Type Annotations for Parameters

In Rust, every parameter in a function's signature must have an explicitly declared type. This design choice ensures that the compiler can infer types in other parts of the code and produce helpful error messages when necessary.

**Functions with Multiple Parameters**

You can define functions with multiple parameters, each with its own type, separated by commas:

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The Measurement is: {value}{unit_label}");
}
```

`print_labeled_measurement` takes two parameters: `value`of type `i32` `unit_label`of type `char`.

Because we called the function with `5` as the value for `value` and `'h'` as the value for `unit_label`, the program output contains those values.

```bash
$ cargo run

The measurement is: 5h
```

> #### Missing Type Annotations
>
> In Rust, forgetting to declare the type of a parameter in a function will result in a compile-time error. Let's examine a common mistake:
>
> ```rust
> fn f(x) {
>   println!("{x}");
> }
>
> fn main() {
>   f(0);
> }
> ```
>
> **Context:** This code fail to compile because the function `f` lacks a type annotation for the parameter `x`.
>
> **Corrected Version:**
>
> ```rust
> fn f(x: i32) {
>   println!("{x}");
> }
>
> fn main() {
>   f(0);
> }
> ```
>
> By adding the type annotation `i32` to the parameter `x`, the function will now compile and run correctly printing `0` as expected. Rust's requirement for explicit type annotations helps catch errors early in the development process, ensuring safer and more predictable code.

## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression. So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

**Statements**

- Statements are instructions that perform some action and do not return a value.
- For example, creating a variable with `let y = 6;` is a statement. It sets up a binding, but the statement itself doesn't produce a value.

```rust
fn main() {
    let y = 6; // This is a statement
}
```

In the above code, `let y = 6;` is a statement that assigns the value `6` to `y`. However, because it's a statement, is doesn't return a value, so you can't do something like this:

```rust
fn main() {
    let x = (let y = 6); // This will cause an error
}
```

If you try to run this code, Rust will produce an error because a `let` statement doesn't return a value and thus can't be assigned to `x`.

When you run this program, the error you’ll get looks like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 3 previous errors; 1 warning emitted
```

**Expressions**

- Expressions, on the other hand, evaluate to a value.
- Most of the code you write is Rust will be expressions. For example, `5 + 6` is an expression that evaluates to `11`.
- Function calls, macros, and block (enclosed in `{}`) are also expressions.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Here's what's happening:

- The block `{ let x = 3; x + 1 }` is an expression.
- The expression evaluates to `4` because the last line inside the block is `x + 1`, which equals `4` (since `x` is `3`).
- This value (`4`) is then assigned to `y`.

**Key Point:** Notice that the line `x + 1` inside the block doesn't end with a semicolon (`;`). That's because, in Rust, if you add a semicolon at the end of an expression, it turns the expression into a statement, which **no longer returns a value**.

## Functions with Return Values

In Rust, functions can return values, allowing you to pass data from the function back to the calling code. When a function returns a value, you must specify the return type after an arrow (`->`) in the function signature. The return value of the function is typically the result of the final expression in the function’s body. Unlike some other languages, Rust does not require you to use the `return` keyword to return a value at the end of a function—although you can use `return` to return early.

Here's an example of a simple function that returns a value:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

- **Function Signature:** The function `five` is declared to return an `i32` type value, which is indicated by the `-> i32` in its signature.
- **Function Body:** The body of `five` consists only of the number `5`. Since there is **no semicolon** after the `5`, it’s treated as an expression, and its value is returned automatically.
- **Using the Return Value:** In `main`, the return value of `five()` is assigned to the variable `x`. Since `five` returns `5`, this is equivalent to `let x = 5;`.

When you run this code, you'll see the following output:

```bash
$ cargo run

The value of x is: 5
```

### Returning Values from Functions

Here's another example where a function takes a parameter and returns a value:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

- **Function Signature:** `plus_one` takes an `i32` argument and returns an `i32`. The return type is specified with `-> i32`.
- **Function Body:** The function returns the value of `x + 1`. Since the `x + 1` expression is the last expression and lacks a semicolon, its value is returned by the function.
- **Using the Return Value:** In `main`, `plus_one(5)` returns `6`, which is then printed.

> **Key Points to Remember:**
>
> - **Return Type:** You must declare the return type of a function after the `->` symbol in the function signature.
> - **Implicit Return:** If the last expression in a function’s body is not followed by a semicolon, it is returned automatically.
> - **Using `return`:** You can use the `return` keyword to return a value early from a function, but it’s not required at the end.

## Questions

### Question 1

In Rust, a curly-brace block like `{ /_ ... _/ }` is:

1. An expression
2. A statement
3. A syntactic scope

**Answer**:

**1. An expression** and **3. A syntactic scope**

A block in Rust is an expression because it evaluates to a value, which is typically the value of the last expression inside it. Additionally, a block defines a syntactic scope for any variables declared with let inside it, meaning those variables are only accessible within that block.

### Question 2

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

```rust
fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}
```

**Answer:**
Yes, the program will pass the compiler.

- The block `{ let y = 1; y + 1 }` inside `f` is an expression that evaluates to `2`.
- The value `2` is then passed as an argument to the function `f`, which returns `3` `(2 + 1)`.
- The output of the program will be:

```bash
3
```
