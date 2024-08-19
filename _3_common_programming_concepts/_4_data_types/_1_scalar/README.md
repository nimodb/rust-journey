# 4.1 Scalar Types

Scalar types represent a single value. Rust has four primary scalar types: **integers**, **floating-point** **numbers**, **Booleans**, and _characters_.

## Integer

An integer is a number without a fractional component. In Rust, integers are categorized based on their size and whether they are signed or unsigned. The table below shows the built-in integer types in Rust.

Table 4-1: Integer Types in Rust

| Length  | Signed  | Unsigned |
| :-----: | :-----: | :------: |
|  8-bit  |  `i8`   |   `u8`   |
| 16-bit  |  `i16`  |  `u16`   |
| 32-bit  |  `i32`  |  `u32`   |
| 64-bit  |  `i64`  |  `u64`   |
| 128-bit | `i128`  |  `u128`  |
|  arch   | `isize` | `usize`  |

### Signed vs. Unsigned Integers

Each integer variant can be either **signed** or **unsigned**, with an explicit size:

- **Signed integers** can represent both positive and negative numbers.
- **Unsigned integers** can represent only non-negative numbers.

### Integer Ranges

Each signed variant can store numbers from **-(2<sup>n - 1</sup>)** to **2<sup>n - 1</sup> - 1**, where `n` is the number of bits that variant uses. For example:

- An `i8` can store numbers from **-(2<sup>7</sup>)** to **2<sup>7</sup> - 1**, which equals `-128` to `127`.

Unsigned variants can store numbers from 0 to **2<sup>n</sup> - 1**. For example:

- A `u8` can store numbers from `0` to **2<sup>8</sup> - 1**, which equals `0` to `255`.

### Special Cases: `isize` and `usize`

The `isize` and `usize` types depend on the architecture of the computer your program is running on, referred to as `arch` in Table 4-1:

- On a 64-bit architecture, these types are 64-bit.
- On a 32-bit architecture, they are 32-bit.

if you're unsure which of type of integer to use, Rust's defaults are generally a good starting point:

- integer default type to `i32`.

> **Integer Overflow**
>
> Integer overflow occurs when you attempt to store a value in an integer type that exceeds the type’s range. For example:
>
> - A `u8` can hold values between `0` and `255`. If you try to assign `256` to a `u8` variable, integer overflow will occur.
>
> **Handling Integer Overflow**
>
> 1. **Debug Mode**
>
> - Rust checks for integer overflow, causing the program to panic if it occurs.
> - Panic means the program exits with an error.
>
> 2. **Release Mode**
>
> - Rust does not check for integer overflow, leading to two’s complement wrapping.
> - For example, in a `u8`, the value `256` becomes `0`, `257` becomes `1`, and so on.
>
> > Note: Relying on integer overflow's wrapping behavior is generally considered an error.
>
> **Methods to Handle Overflow Explicitly**
> We define a variable value of type `u8` and set it to `255`, which is the maximum value a `u8` can hold.
>
> ```rust
> fn main() {
> // This is a u8 type, which can store values from 0 to 255.
> let mut value: u8 = 255;
> }
> ```
>
> To handle overflow situations explicitly, you can use methods provided by Rust’s standard library:
>
> - `wrapping_*` methods (e.g., `wrapping_add`): Wraps the value in all modes.
> - `wrapping_add` allows the addition to wrap around on overflow. So `255 + 1` becomes `0`.
>
> ```rust
> // Example of handling overflow using `wrapping_add`
> let wrapped_value = value.wrapping_add(1);
> println!("Wrapped value: {}", wrapped_value);
> ```
>
> - `checked_*` methods: Returns None if overflow occurs.
> - `checked_add` returns `None` if an overflow occurs, allowing you to handle the situation explicitly. Here, `None` indicates that the overflow occurred.
>
> ```rust
>  // Example of handling overflow using `checked_add`
>  match value.checked_add(1) {
>      Some(v) => println!("Checked value: {}", v),
>      None => println!("Overflow occurred!"),
>  }
> ```
>
> - `overflowing_*` methods: Returns the value and a boolean indicating whether overflow occurred.
> - `overflowing_add` returns a tuple: the result of the addition and a boolean that tells whether an overflow occurred. Here, the value wraps around to `0`, and the boolean `did_overflow` is `true`.
>
> ```rust
> // Example of handling overflow using `overflowing_add`
> let (overflowed_value, did_overflow) = value.overflowing_add(1);
> println!(
>   "Overflowed value: {}, Did overflow: {}",
>   overflowed_value, did_overflow
> );
> ```
>
> - `saturating_*` methods: Saturates at the minimum or maximum values if overflow occurs.
> - `saturating_add` prevents overflow by setting the value to the maximum possible value (`255` for `u8`) if overflow would occur.
>
> ```rust
> // Example of handling overflow using `saturating_add`
> let saturated_value = value.saturating_add(1);
> println!("Saturated value: {}", saturated_value);
> ```
>
> **Output**
>
> ```bash
> Initial value: 255
> Wrapped value: 0
> Overflow occurred!
> Overflowed value: 0, Did overflow: true
> Saturated value: 255
> ```

## Floating-Point Types

Rust has two primitive types for floating-point numbers, which are numbers that include a fractional component (i.e., numbers with decimal points). These types are `f32` and `f64`, corresponding to 32-bit and 64-bit floating-point numbers, respectively.

- `f32` (32-bit float): Single-precision floating-point number.
- `f64` (64-bit float): Double-precision floating-point number. This is the default floating-point type in Rust.
  - On modern CPUs, `f64` performs nearly as fast as `f32` but provides higher precision. Therefore, Rust defaults to `f64` for floating-point numbers.

Here’s an example that shows floating-point numbers in action:

```rust
fn main() {
    let x = 2.0;   // f64 by default
    let y: f32 = 3.0; // explicitly typed as f32
}
```

**Use Cases:**

- Use `f64` when you need precision and are not restricted by memory.
- Use `f32` when you need to save memory and can tolerate lower precision.

## Numeric Operations

Rust provides support for the basic mathematical operations that you would expect to use with all number types. These operations include addition, subtraction, multiplication, division, and remainder (modules). Each of these operations works as expected, with integer division truncation toward zero.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1, integer division truncates towards zero

    // remainder
    let remainder = 43 % 5;
}
```

> \*\* Notes on Division
>
> - **Integer Division:** When both operands are integers. division results in an integer, with any fractional part truncated toward zero. For example, `-5 / 3` results in `-1`.
> - **Floating-Point Division:** When using floating-point numbers, division returns a floating-point result

## Boolean Type

Rust, like most programming languages, include a Boolean type for representing truth values. In Rust, the Boolean type is represented by `bool` and has exactly two possible values: `true` and `false`.

```rust
fn main() {
let t = true;
let f: bool = false; // with explicit type annotation
}
```

It occupies one byte of memory and can be used with or without an explicit type annotation, as Rust can infer the type from the value.

The main way to use Boolean values is through conditionals, such as an if expression.

```rust
fn main() {
    let is_raining = true;

    if is_raining {
        println!("Don't forget your Babe <3")
    } else {
        println!("Enjoy the sunshine!")
    }
}
```

In this example, the value if `is_raining` determines which message is printed. This demonstrates how Boolean values are typically utilized to influence the flow of program execution based on conditions.

## The Character Type

In Rust, the `char` type is the most basic type for representing alphabetic characters. Unlike string literals, which use double quotes, `char` literals are enclosed in single quotes.

```rust
fn main() {
    let letter: char = 'z'; // English letter
    let mathematical_symbol: char = 'ℤ'; // Mathematical symbol
    let persian_letter: char = 'س'; // Persian letter
    let emoji: char = '🦀';

    println!("English letter: {}", letter);
    println!("Mathematical symbol: {}", mathematical_symbol);
    println!("Persian letter: {}", persian_letter);
    println!("Rust emoji: {}", emoji);
}
```

The `char` type is four bytes in size and represents a Unicode Scalar Value, allowing it to encompass a wide range of characters from various languages (such as Persian, Chinese, Japanese, and Korean), emojis, and zero-width spaces. The valid range for Unicode Scalar Values is from `U+000` to `U+D7FF` and `U+E000` to `U+10FFF`.

> It's important to note that the concept of a "character" in Unicode might not always align with the typical human understanding of a character.
