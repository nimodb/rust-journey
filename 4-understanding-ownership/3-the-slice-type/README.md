# The Slice Type

_Slice_ lets you reference a Contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it is a non-owning pointer.

## Problem: Finding the First Word in a String

We'll write a function that takes a string of words separated bt spaces and returns the first word it finds in that string. If there's no space, the whole string should be returned as the word.

First, without using slices, here's how we would attempt this:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();  // Convert the string to a byte array

    for (i, &item) in bytes.iter().enumerate() {  // Iterate over the byte array
        if item == b' ' {  // Check for the first space (byte value ' ')
            return i;  // Return the position of the space
        }
    }

    s.len()  // If no space found, return the length of the string
}
```

Because we need to go through the `String` element by element and check whether a value is a space, we’ll convert our `String` to an array of bytes using the `as_bytes` method.

```rust
    let bytes = s.as_bytes();
```

Next, we create an iterator over the array of bytes using the iter method:

```rust
    for (i, &item) in bytes.iter().enumerate() {
```

We’ll discuss iterators in more detail in [Chapter 13](). For now, know that `iter` is a method that returns each element in a collection and that `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead. The first element of the tuple returned from `enumerate` is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.

Because the `enumerate` method returns a tuple, we can use patterns to destructure that tuple. We’ll be discussing patterns more in [Chapter 6](). In the `for` loop, we specify a pattern that has `i` for the index in the tuple and `&item` for the single byte in the tuple. Because we get a reference to the element from `.iter().enumerate()`, we use `&` in the pattern.

Inside the `for` loop, we search for the byte that represents the space by using the byte literal syntax. If we find a space, we return the position. Otherwise, we return the length of the string by using `s.len()`.

```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

We now have a way to find out the index of the end of the first word in the string, but there’s a problem. We’re returning a `usize` on its own, but it’s only a meaningful number in the context of the `&String`. In other words, because it’s a separate value from the `String`, there’s no guarantee that it will still be valid in the future.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

This program compiles without any errors and would also do so if we used word after calling `s.clear()`. Because word isn’t connected to the state of `s` at all, `word` still contains the value `5`. We could use that value `5` with the variable `s` to try to extract the first word out, but this would be a bug because the contents of `s` have changed since we saved `5` in `word`.

Having to worry about the index in `word` getting out of sync with the data in `s` is tedious and error-prone! Managing these indices is even more brittle if we write a `second_word` function. Its signature would have to look like this:

```rust
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking a starting and an ending index, and we have even more values that were calculated from data in a particular state but aren’t tied to that state at all. We have three unrelated variables floating around that need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

### String Slices

To solve the problem, we can return a **string slice** instead of an index. A slice is tied to the string's data and ensures safety at compile time.

A **string slice** is a reference to portion of a `String`, and it looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];  // A slice referencing the first 5 characters of s
let world = &s[6..11]; // A slice referencing the last 5 characters of s
```

Rather than a reference to the entire `String`, `hello` is a reference to a portion of the `String`, specified in the extra `[0..5]` bit. We create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`. So, in the case of `let world = &s[6..11];`, world would be a slice that contains a pointer to the byte at index 6 of `s` with a length value of `5`.

![Three tables: a table representing the stack data of s, which points to the byte at index 0 in a table of the string data "hello world" on the heap. The third table rep-resents the stack data of the slice world, which has a length value of 5 and points to byte 6 of the heap data table.](../../img/trpl04-06.svg)

With Rust’s `..` range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the `String`, you can drop the trailing number. That means these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> **Note:** String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the [“Storing UTF-8 Encoded Text with Strings”]() section of Chapter 8.

With all this information in mind, let’s rewrite `first_word` function using a slice:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert to byte array

    for (i, &item) in bytes.iter().enumerate() { // Iterate over bytes
        if item == b' ' { // Look for space
            return &s[0..i]; // Return a slice from start to the space
        }
    }

    &s[..] // If no space found, return the entire string as a slice
}
```

We get the index for the end of the word, by looking for the first occurrence of a space. When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up because the compiler will ensure the references into the `String` remain valid. Remember the bug in the program, when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of `first_word` will throw a compile-time error:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // Borrow a slice to the first word

    s.clear(); // Clear the string

    // After clearing the string, using `word` is invalid
    // Rust will prevent this at compile-time:
    // println!("the first word is: {word}"); // This will cause an error
}
```

Whe trying to use `word` after the string is cleared, Rust throw a **compile-time error**:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {word}");
   |                                  ------ immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point. Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

#### String Literals as Slices

We previously discussed how string literals are stored inside the binary, and now, with an understanding of slice, we can properly comprehend how string literals work in Rust:

```rust
let s = "Hello, world!";
```

Here, The type of `s` here is `&str`: which is a string slice that points to the specific location in the binary where the literal is stored. This is also the reason why string literals are immutable——`&str` is an immutable reference.

#### String Slices as Parameters

Since we can take slices of both string literals and `String` values, we can improve the signature of the `first_word` function to make it more versatile:

```rust
fn first_word(s: &String) -> &str {
```

A more idiomatic way to write this function would be:

```rust
fn first_word(s: &str) -> &str {
```

This version of function is more flexible because it works with both `&String` and `&str`. If we have a string slice, we can pass it directly. If we have a `String`, we can pass a slice of it or a reference to it. This takes advantage of **deref coercions**, a feature that allows Rust to automatically convert a `&String` to `&str`. We'll discuss this further in the section on [“Implicit Deref Coercions with Functions and Methods”]() in Chapter 15.

By using `&str` instead of `&String`, our API becomes more general without losing any functionality. Here's how can use it.

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices

String slices are specific to strings, but slices can be used with other collections as well. For example, consider the following array:

```rust
let a = [1, 2, 3, 4, 5];
```

Just as we can take a slice of a string, we can take a slice of this array:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

The type of this slice is `&[i32]`, and it works similarly to string slices. It stores a reference to the first element and a length. This kind of slice is useful for various types of collections. We'' explore slices with vectors and other collection in Chapter 8.

### Question

#### Question 1

In the following program, the variables `s2` and `s3` are located in the stack frame of `main`. Each variable has a size in memory, excluding the size of the data they point to. Which statement is true about the sizes of `s2` and `s3`?

```rust
fn main() {
  let s = String::from("hello");
  let s2: &String = &s;
  let s3: &str = &s[..];
}
```

**Answer:**
The type `&String` is a simple reference consisting of a single pointer, which is 8 bytes on a 64-bit architecture. The type `&str` is a slice reference, consisting of both a pointer and a length, making it 16 bytes. Therefore, `s3` of type `&str` uses more memory than `s2` of type `&String`. You can verify this using [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html):

```rust
fn main() {
  println!(
    "&String={} &str={}",
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
  );
}
```

Additionally, Rust automatically converts string references to either `&String` or `&str` based on the context, so the expression `&s` can produce different values based on the expected type.

#### Question 2

Will the following program compile? If so, what will its output be?

```rust
fn main() {
  let mut s = String::from("hello");
  for &item in s.as_bytes().iter() {
    if item == b'l' {
      s.push_str(" world");
    }
  }
  println!("{s}");
}
```

**Answer:**
The program will now compile because `s.as_bytes()` produces an immutable reference to `s` making it illegal to mutate `s` (e.g using `push_str`) inside the loop. Therefore, this program results in a compiler error.

### Summary

Rust's ownership, borrowing, and slicing concepts provide memory safety at compile time. Rust gives you control over memory management, similar to other systems programming languages, but it automates memory cleanup when an owner goes out of scope, reducing the need for extra code and debugging.

Ownership will continue to be an important concept in Rust, and we'll see it impact other features throughout the rest of the book. Up next, let's explore how to group pieces of data together in `structs` in chapter5.
