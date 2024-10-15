# 5.2 An Example Program Using Structs

In this section, we'll explore the benefits of using structs by creating a program to calculate the area of a rectangle. We'll start with a basic approach using individual variables and incrementally refactor the code the use structs, enhancing clarity and organization along the way.

### Initial Implementation Using Variables

We'll begin by writing a simple Rust program that computes the area of a rectangle using individual variables for the rectangle's dimensions (width and height). Let's create a new binary project Cargo, called `rectangles`.

Here's the code in the project's `src/main.rs`:

```rust
fn main(){
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

Running this program with `cargo run` will output:

```bash
$ cargo run

The area of the rectangle is 1500 square pixels.
```

While this works, the `area` function takes two separate parameters (`width` and `height`), and there's no indication they belong to a single conceptual entity—the rectangle. It would be more meaningful to group these values together, which is where Rust's compound types come into play.

### Refactoring with Tuples

To improve our design, we can refactor the code to use a tuple. Tuples allow us to group values together in a single entity, making the connection between `width` and `height` clearer. We’ve already discussed one way we might do that in “[The Tuple Type](https://github.com/nimodb/rust-journey/tree/main/3-common-programming-concepts/4-data-types/2-compound#the-tuple-type)” section of Chapter 3: by using tuples.

Here's the revised program using a tuple:

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

Now, we pass just one argument—a tuple containing both `width` and `height`—to the `area` function. However, while this approach groups the values together, the lack of labels makes it les clear which value corresponds to the width and which corresponds to the height. This could lead to confusion, especially in more complex programs.

### Introducing Structs: Adding Clarity and Meaning

To make the code more readable and meaningful, we can use structs. Structs allow us to group related data and give each field a name, making the program easier to understand and maintain.

Here's the same program refactored to use a `Rectangle` struct:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Now, the dimensions are stored within an instance of the `Rectangle` struct, and we pass a reference to that instance to the `area` function. This refactor improves the clarity of our code. The `Rectangle` struct gives meaningful names to the rectangle's width and height, and it's clear that they are fields of a rectangle.

The `area` function now works specifically with `Rectangle` instances, and the function signature makes this relationship explicit.

### Printing Structs: Implementing the `Debug` Trait

While the refactored code is more readable, printing the `rect1` struct directly won't work with the `println!` macro in its default from. If we try:

```rust
println!("rect1 is {}", rect1)
```

We'll get an error because Rust doesn't know how to format the output for structs by default. To fix this, we can use Rust's `Debug` trait, which provides a way to output strcuts fro debugging purposes.

```bash
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```


The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption. The primitive types we’ve seen so far implement `Display` by default because there’s only one way you’d want to show a `1` or any other primitive type to a user. But with structs, the way `println!` should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of `Display` to use with `println!` and the `{}` placeholder.

If we continue reading the errors, we’ll find this helpful note:

```bash
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Let’s try it! The `println!` macro call will now look like `println!("rect1 is {rect1:?}");`. Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. The `Debug` trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

Compile the code with this change. Drat! We still get an error:

```bash
error[E0277]: `Rectangle` doesn't implement `Debug`
```

But again, the compiler gives us a helpful note:

```bash
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

Now when we run the program, we won’t get any errors, and we’ll see the following output:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }
```

Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string. In this example, using the `{:#?}` style will output the following:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Another way to print out a value using the `Debug` format is to use the [`dbg!` macro](https://doc.rust-lang.org/std/macro.dbg.html), which takes ownership of an expression (as opposed to `println!`, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

> Note: Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!`, which prints to the standard output console stream (`stdout`). We’ll talk more about `stderr` and `stdout` in the “Writing Error Messages to Standard Error Instead of Standard Output” section in Chapter 12.

Here’s an example where we’re interested in the value that gets assigned to the `width` field, as well as the value of the whole struct in `rect1`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

We can put `dbg!` around the expression `30 * scale` and, because `dbg!` returns ownership of the expression’s value, the `width` field will get the same value as if we didn’t have the `dbg!` call there. We don’t want `dbg!` to take ownership of `rect1`, so we use a reference to `rect1` in the next call. Here’s what the output of this example looks like:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

We can see the first bit of output came from src/main.rs line 10 where we’re debugging the expression `30 * scale`, and its resultant value is `60` (the `Debug` formatting implemented for integers is to print only their value). The `dbg!` call on line 14 of src/main.rs outputs the value of `&rect1`, which is the `Rectangle` struct. This output uses the pretty `Debug` formatting of the `Rectangle` type. The `dbg!` macro can be really helpful when you’re trying to figure out what your code is doing!

In addition to the `Debug` trait, Rust has provided a number of traits for us to use with the `derive` attribute that can add useful behavior to our custom types. Those traits and their behaviors are listed in Appendix C. We’ll cover how to implement these traits with custom behavior as well as how to create your own traits in Chapter 10. There are also many attributes other than `derive`; for more information, see the [“Attributes" section of the Rust Reference](https://doc.rust-lang.org/reference/attributes.html).

Our `area` function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our `Rectangle` struct because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the `area` function into an `area` method defined on our `Rectangle` type.

## Question

#### Question 1

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let a = area(rect1);
  println!("{} * {} = {}", rect1.width, rect1.height, a);
}

fn area(rectangle: Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
```

**Answer:** The `area` function takes ownership of its argument `rectangle`. Calling `area(rect1)` therefore moves `rect1`, meaning it cannot be used on the next line. So this program **does not** Compile.

#### Question 2

Which statement best describes a difference between the `Display` and `Debug` traits?

- [] `Display` is for printing values to the console, while `Debug` is for viewing values in a debugger
- [] `Display` cannot be implemented for structs, while `Debug` can be implemented for structs
- [] There is no difference, `Display` and `Debug` are aliases for the same trait.
- [x] `Display` is for presenting values to an end-user, while `Debug` is for developers' internal use
