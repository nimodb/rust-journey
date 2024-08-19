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

Ir Rust, arrays provide a way to group multiply values of the **same type** into a **fixed-size**, **stack-allocated** collection. Unlike tuple, all elements in an array must be of the same type, and once an array's size is set, it can't be changed.

You can create an array by writing the values in a comma-separated list within square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are particularly useful when you know the number of elements won't change. For instance, if you were storing the months of year, and array is appropriate because there will always be 12 elements:

```rust
let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
```

You can explicitly specify an array's type by listing the type of the elements, followed by a semicolon (`;`), and then the number of elements in the array:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, the array `a` has 5 elements are initialized to the same value, you can use a shorthand notation:

```rust
let a = [3; 5];
```

This creates an array with `5` elements, all set to `3`, equivalent to writing `let a = [3, 3, 3, 3, 3];`.

### Accessing Array Elements

You can access individual elements in an array using indexing:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("First: {}, Second: {}", first, second);
}
```

In this example, `first` will be `1` and `second` will be `2`, corresponding to the values at index `[0]` and `[1]`, respectively.

**Invalid Array Element Access**

Rust's strict handling of array bounds is a key feature that enhances memory safety. When you attempt to access an element outside the bounds of a array, Rust will catch this mistake and immediately terminate the program with a runtime error rather that allowing potentially dangerous memory access.

```rust
fn main () {
    let a = [1, 2, 3, 4, 5];

    let out_of_bounds = a[10]; // This will cause a runtime error
}
```

- The array `a` contains 5 elements with indices ranging from `0` to `4`.
- The code tries to access the element at index `10`, which does not exist in the array.
- Rust's runtime check this access and finds that it's out of bounds, so it will stop the program and display an error message.

```bash
index out of bounds: the length is 5 but the index is 10
```

> This message indicates that the program tried to access an index that in beyond the length of the array, causing it to panic.

##### **Why this Matter**

In languages without strict bounds checking, accessing an array out of the bound might result in undefined behavior, such as reading garbage values or corrupting memory, leading to bugs that are difficult to track down. Rust's approach ensures that any such mistake is caught immediately, preventing these issues from occurring.

> Rust’s safety mechanisms, like array bounds checking, ensure that your programs are less prone to bugs related to invalid memory access. By catching these errors early, Rust helps you write more reliable and secure code.

#### Question 1:

```rust
fn main() {
  let message = "The temperature today is:";
  let x = [message, 100];

  println!("{} {}", x[0], x[1]);
}
```

**Analysis:**
This program will not pass the compiler because arrays in Rust must contain elements of a single type. The code attempts to mix a string (`message`) with a number (`100`), which is not allowed. The compiler will raise an error indicating a type mismatch.

#### Question 2:

```rust
fn main() {
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;

    println!("{}", a[0] + t.1[0]);
}
```

**Analysis:**
This program will compile successfully. Here's a breakdown of the operations:

- The array `[1; 2]` creates an array with two elements, both set to `1`.
- The array `[3; 4]` creates an array with four elements, all set to `3`.
- The tuple `t` contain these two arrays `([1, 1], [3, 3, 3, 3])`.
- `a` is bound to the first array `[1, 1]`, and `b` to `[3, 3, 3, 3]`.
- The expression `a[0]` adds the first element of `a` which is `1`.
- The expression `t.1[0]` adds the first element of `t.1` which is `3`.
- The program correctly sums `1 + 3` and prints `4`.
  ```bash
  4
  ```
