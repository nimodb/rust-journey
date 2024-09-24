# 4.2 Reference and Borrowing

In Rust, ownership, boxes, and moves create a solid foundation for managing memory, especially when dealing with heap-allocated data. However, the "move-only" approach can sometimes feel cumbersome. For instance, imagine you want to read strings more than once in your program:

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}
```

In this example, calling `greet` moves `m1` and `m2` into the function, making them invalid for further use in `main`. Once moved, Rust doesn't allow them to be accessed again, which leads to a compile-time error. To address this, we could try returning the values:

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
```

Although this code works, it's more verbose then necessary. Fortunately, Rust offers a more concise solution through references, allowing us to pass and reuse values without transferring ownership.

### References Are Non-Owning Pointers

A **reference** in Rust act as a pointer to data without owning it. Here's how we can rewrite the above example with references:

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}
```

By adding `&` before `m1` and `m2`, we create references to these values. The expression `&m1` and `&m2` uses the ampersand operator to create a reference to (or “borrow”) `m1` and `m2`.

The `greet` function then takes references (`&String`) instead of ownership of the strings. After the function ends, the original values in `main` remain accessible because the references do not own the data, so no deallocation occurs.

References are non-owning pointers, because they do not own the data they point to.

### Dereferencing a Pointer Accesses Its Data

To access or modify the data a reference points to, we use **dereferencing**. In Rust, dereferencing is done using the asterisk (`*`) operator. Here's an example demonstrating various ways of dereferencing:

```rust
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value,
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value

let r2: &i32 = &*x;      // r2 points to the heap value directly
let c: i32 = *r2;    // so only one dereference is needed to read it
```

In this example, `r1` points to a box on the stack, requiring two dereferences (`**r2`) to reach the heap value. However, `r2` points directly to the heap, so only one dereference is needed.

### Implicit Dereferencing in Method Calls

You won’t often see the dereference operator explicitly in Rust, because Rust automatically handles dereferencing in certain contexts like method calls. For example, this program shows two equivalent ways of calling the [i32::abs](https://doc.rust-lang.org/std/primitive.i32.html#method.abs) (absolute value) and [str::len](https://doc.rust-lang.org/std/primitive.str.html#method.len) (string length) functions:

```rust
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
```

Here, the `abs()` method automatically dereferences the value for you, and Rust converts types behind the scenes when needed. This makes the code more concise while still safe.

1. The `i32::abs` function expects an input of type `i32`. To call `abs` with a `Box<i32>`, you can explicitly dereference the box like `i32::abs(*x)`. You can also implicitly dereference the box using method-call syntax like `x.abs()`. The dot syntax is syntactic sugar for the function-call syntax.

2. This implicit conversion works for multiple layers of pointers. For example, calling `abs` on a reference to a box `r: &Box<i32>` will insert two dereferences.

3. This conversion also works the opposite direction. The function `str::len` expects a reference `&str`. If you call `len` on an owned `String`, then Rust will insert a single borrowing operator. (In fact, there is a further conversion from `String` to `str!`)

### Questions

#### Question 1

Consider the following program, showing the state of memory after the last line:

```rust
let x = Box::new(0);
let y = Box::new(&x);
```

How many dereferences are required to access the value `0` through `y`?

**Answered:** 3 dereferences are required (`***y` is the correct expression). The type of `y` is `Box<&Box<i32>>`, meaning you must dereferences there times to reach the heap value.

#### Question 2

Consider the following program, showing the state of memory after the last line:

```rust
fn get_first(vr: &Vec<i32>) -> i32 {
  vr[0]
}

fn main() {
  let mut v = vec![0, 1, 2];
  let n = get_first(&v);
  println!("{} {}", n, v[1]);
}
```

Which of the following best explains why `v` is not deallocated after calling `get_first`?

**Response**

- [ ] `get_first` returns a value of type `i32`, not the vector itself

- [ ] `vr` is not mutated within `get_first`

- [ ] `v` is used after calling `get_first` in the `println`

- [x] `vr` is a reference which does not own the vector it points to

**Answer:** The correct answer is: "`vr` is a reference which does not own the vector it points to". This means the reference does not move deallocate the data when passed to the function, allowing `v` to still be used after `get_after` returns.

### Rust Avoids Simultaneous Aliasing and Mutation

Pointer can be powerful, but they also introduce risks, particularly when aliasing (accessing the same data through different variables) is combined with mutation (changing the data). Here's how Rust manages this:

#### 1. Aliasing and Mutation Risks:

    Aliasing can lead to issues if one variable deallocates or modifies the data that another variable is accessing. For example, if a variable deallocates memory that another variable is still pointing to, this can lead to dereferencing invalid memory, causing undefined behavior.

#### 2. Example with Vector:

    As a running example, we are going to look at programs using the vector data structure, [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html). Unlike arrays which have a fixed length, vectors have a variable length by storing their elements in the heap. For example, [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) adds an element to the end of a vector, like this:

    ```rust
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    ```

    The macro `vec!` creates a vector with the elements between the brackets. The vector `v` has type `Vec<i32>`. The syntax `<i32>` means the elements of the vector have type `i32`.

    One important implementation detail is that `v` allocates a heap array of a certain capacity. We can peek into Vec’s internals and see this detail for ourselves:

    ```rust
    let mut v: Vec<i32> = vec![1, 2, 3];
    ```

    Notice that the vector has a length (`len`) of 3 and a capacity (`cap`) of 3. The vector is at capacity. So when we do a `push`, the vector has to create a new allocation with larger capacity, copy all the elements over, and deallocate the original heap array. The array `1 2 3 4` is in a (potentially) different memory location than the original array `1 2 3`.

    To tie this back to memory safety, let’s bring references into the mix. Say we created a reference to a vector’s heap data. Then that reference can be invalidated by a push, as simulated below:

    ```rust
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    println!("Third element is {}", *num);
    ```

    Initially, `v` points to an array with 3 elements on the heap. Then `num` is created as a reference to the third element. However, the operation `v.push(4)` resizes `v`. The resize will deallocate the previous array and allocate a new, bigger array. In the process, `num` is left pointing to invalid memory. Therefore, dereferencing `*num` reads invalid memory, causing undefined behavior.

    In more abstract terms, the issue is that the vector `v` is both aliased (by the reference `num`) and mutated (by the operation `v.push(4)`). So to avoid these kinds of issues, Rust follows a basic principle:

#### 3. Pointer Safety Principle:

    Rust enforces that data can either be aliased or mutated, but not both at the same time. For owned data (like with boxes), moving ownership ensures no aliases exist. References, being non-owning pointers, follow different rules, allowing temporary aliases while ensuring safety.

### Dangling References

A dangling reference occurs when a pointer refers to a memory location that has been deallocated. Rust prevents this through compile-time checks.

Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:

```rust
fn main() {
    let reference_to_nothing = dangle(); // Attempting to create a dangling reference
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // s goes out of scope here, leading to a dangling reference
}
```

The Compiler prevents this code from compiling, producing and error because `s` is dropped when it goes out of scope, leaving the reference pointing to invalid memory.

> this function's return type contains a borrowed value, but there is no value for it to be borrowed from

Let’s take a closer look at exactly what’s happening at each stage of our `dangle` code:

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s. Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
}
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid `String`. That’s no good! Rust won’t let us do this.

The solution here is to return the `String` directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

### The Rules of References

References provide the ability to read and write data without consuming ownership of it. References are created with borrows (`&` and `&mut`) and used with dereferences (`*`), often implicitly.

However, references can be easily misused. Rust’s borrow checker enforces a system of permissions that ensures references are used safely:

- All variables can read, own, and (optionally) write their data.
- Creating a reference transfers permissions from the original data to the reference.
- Permissions are restored once the reference goes out of scope.
- Data must remain valid for as log as references to it exist. The borrow checker ensures this, preventing dangling references and other unsafe behaviors.

Understanding these concepts and rules is essential for effectively using Rust’s memory management features while avoiding common pitfalls associated with pointers and references.
