## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

## The Tuple Type

In Rust, a tuple is a compound data type used to group together a fixed number of values with potentially different types into a single unit. Once a tuple is created, its size **can't be changed**, meaning you can't grow or shrink elements.

To create a tuple, list the values inside parentheses, separated by commas. Each element in the tuple can be of a different type, and the tuple's length is fixed at creation.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Here,`tup` is a tuple containing an `i32`, a `f32`, and a `u8`. To access individual values within a tuple, you can destructure it using pattern matching.

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

Alternatively, you can access elements directly by using dot (`.`) notation followed by the index of the element:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("Values: {}, {}, {}", five_hundred, six_point_four, one);
}
```

Rust also includes a special tuple with no values, known as the unit type `()`, which represents an empty value or an empty return type. If a function does not return any other value, it implicitly return `()`.

**Modifying Tuples**
If a tuple is mutable, you can modify its elements:

```rust
fn main() {
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;

    println!("Modified tuple: {:?}", x);
}
```

## The Array Type
