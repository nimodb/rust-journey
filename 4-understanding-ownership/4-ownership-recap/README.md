# 4.4 Ownership Recap

This chapter introduced many new concepts like **ownership**, **borrowing**, and **slices**. If you're not familiar with systems programming, it also introduced concepts such as **memory allocation**, the **stack vs. the heap, pointers**, and the potential dangers of **undefined behavior**. Before we move forward with Rust, let's take a moment to review and practice these key ideas.

### Ownership versus Garbage Collection

To better understand Rust's ownership model, let's compare it with a more common memory management technique: **garbage collection**. Many popular languages, including Python, Javascript, Java, and Go, use a garbage collector (GC) to manage memory. A garbage collector operates alongside the running program, continuously scanning memory to identify and deallocate data that is no longer accessible or needed.

The primary benefit of garbage collection is that is automatically prevents issues like **use-after-free** and **double frees**. which are common sources of bugs in languages like C or C++. By not requiring manual memory management, GC also reduces the complexity of the system. However, this convenience comes with some drawbacks:

1. **Performance overhead:** Whether frequent and small (like reverence counting in Python) or sporadic and large (like tracing collectors in languages such as Java) GC inevitably adds overhead to the program.
2. **Unpredictably:** It's often difficult to determine when exactly memory will be freed. This can lead to latency spikes in application when the garbage collector runs at unexpected times.

Let's compare the two approaches by implementing a simple `Document` type in a garbage-collected language like Python:

```python
class Document:
    def __init__(self, words: List[str]):
        """Create a new document"""
        self.words = words

    def add_word(self, word: str):
        """Add a word to the document"""
        self.words.append(word)

    def get_words(self) -> List[str]:
        """Get a list of all the words in the document"""
        return self.words
```

Consider the following usage of this `Document` class:

```python
words = ["Hello"]
d = Document(words)

d2 = Document(d.get_words())
d2.add_word("world")
```

Now ask yourself:

1. **When is the `words` array deallocated?**
   Since both `d` and `d2` contain references to the same array, the memory won't be deallocated until all references to it go out of scope. This kind of implicit sharing is typical in garbage-collected languages, but ut can make memory behavior unpredictable.
2. **What are the contents of `d` after mutating?**
   After calling `d2.add_word("world")`, the `d` document is also modified. Since `d2` contains a reference to the same array as `d`, the addition of `"world"` mutates both documents. This implicit mutability can lead to unintended side effects and hard-to-track bugs.

This issue isn't unique to Python —— it appears in many garbage-collected languages like Java and Javascript. Even though these languages abstract away pointer, they still exist behind the scenes, and developers often encounter them in unexpected ways. Let's now translate this example into Rust, which explicitly ownership and borrowing:

```rust
type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}
```

Here's what's different in Rust implementation:

1. **Ownership and Scope:**
   The `new_document` function **takes ownership** of the `words`, meaning it controls the vector's lifecycle. The vector is deallocated when the owning `Document` goes out of scope. This predictable deallocation eliminates the uncertainty of garbage collectio.
2. **Explicit Mutability:**
   The `add_word` function take a **mutable reference** (`&mut Document`). Mutability must be explicitly granted in Rust, reducing the likelihood of unintended modifications. Additionally, ownership of the `word` is transferred to the document, preventing any external modifications to that word.
3. **No Implicit Sharing:**
   The `get_words` function returns an **immutable reference** (`&[String]`) to the words within the document. If you want to create a new document based on this data, you must **Clone** it, ensuring that changes to one document don't affect another:

   ```rust
   fn main() {
        let words = vec!["hello".to_string()];
        let d = new_document(words);

        // Clone the words to create a new, independent document
        let words_copy = get_words(&d).to_vec();
        let mut d2 = new_document(words_copy);
        add_word(&mut d2, "world".to_string());

        // Changes to d2 do not affect d
        assert!(!get_words(&d).contains(&"world".into()));
    }
   ```

Rust's ownership model makes memory behavior **predictable and explicit**, While garbage collection hides memory management from the developer, Rust brings it to the forefront, ensuring:

1. **Predictable performance** by avoiding the overhead of garbage collection.
2. **Safer memory management**, preventing common bugs such as dangling pointer or memory leaks.
3. **Controlled mutability** by requiring explicit mutable reference.

Ownership is a core concept in Rust and influences many other features in language. As you move forward, you'll see how it ties into Rust's type system, concurrency model, and more. Let's continue exploring these concept and see how the shape Rust's design.

### The Concepts of Ownership

Next, let’s review the concepts of ownership. This review will be quick — the goal is to remind you of the relevant concepts. If you realize you forgot or didn’t understand a concept, then we will link you to the relevant chapters which you can review.

#### Ownership at Runtime

We’ll start by reviewing how Rust uses memory at runtime:

- Rust allocates local variables in stack frames, which are allocated when a function is called and deallocated when the call ends.
- Local variables can hold either data (like numbers, booleans, tuples, etc.) or pointers.
- Pointers can be created either through boxes (pointers owning data on the heap) or references (non-owning pointers).

#### Ownership at Compile-time

Rust tracks **R (read)**, **W (write)**, and **O (own)** permissions on each variable. Rust requires that a variable has appropriate permissions to perform a given operation. As a basic example, if a variable is not declared as `let mut`, then it is missing the **W** permission and cannot be mutated.

A variable’s permissions can be changed if it is **moved** or **borrowed**. A move of a variable with a non-copyable type (like `Box<T>` or `String`) requires the **RO** permissions, and the move eliminates all permissions on the variable. That rule prevents the use of moved variables.

Borrowing a variable (creating a reference to it) temporarily removes some of the variable’s permissions. An immutable borrow creates an immutable reference, and also disables the borrowed data from being mutated or moved. For example, printing an immutable reference is ok:

```rust
fn main() {
    let s = String::from("hello");
    let r = &s; // Immutable borrow
    println!("{}", r); // ok, prints "hello"
}
```

But mutating an immutable reference is not ok:

```rust
fn main() {
    let s String::from("hello");
    let r = &s; // Immutable borrow
    r.push_str(", world!"); // Error: cannot borrow immutable reference as mutable
}
```

And mutating the immutably borrowed data is not ok:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // Immutable borrow
    let r2 = &mut s; // Error: cannot borrow `s` as mutable because  it is also borrowed as immutable
}
```

And moving data out of the reference is not ok:

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    let s_moved = *r; // Error: cannot move out of `*r` which is behind a reference
}
```

A mutable borrow creates a mutable reference, which disables the borrowed data from being read, written, or moved. For example, mutating a mutable reference is ok:

```rust
fn main() {
    let mut s = String::from("hello");
    let r = &mut s; // Mutable borrow
    r.push_str(", world!"); // ok, mutates `s`
    println!("{}", r); // prints "hello, world!"
}
```

But accessing the mutably borrowed data is not ok:

```rust
fn main() {
    let mut s = String::from("hello");
    let r = &mut s; // Mutable borrow
    println!("{}", s); // Error: cannot use `s` because it was mutably borrowed
}
```

#### Connecting Ownership between Compile-time and Runtime

Rust’s permissions are designed to prevent undefined behavior. For example, one kind of undefined behavior is a use-after-free where freed memory is read or written. Immutable borrows remove the **W** permission to avoid use-after-free, like in this case:

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    drop(s); // Error: cannot move out of `s` because it is borrowed
    println!("{}", r); // This would be use-after-free if allowed
}
```

Another kind of undefined behavior is a double-free where memory is freed twice. Dereferences of references to non-copyable data do not have the **O** permission to avoid double-frees, like in this case:

```rust
fn main() {
    let b = Box::new(5);
    let r = &b;
    let b_moved = *r; // Error: cannot move out of `*r` which is behind a reference
    drop(b); // Double-free error if allowed
}
```

### The Rest of Ownership

As we introduce additional features, those features will have specific interactions with ownership. This chapter provides the essential foundation for understanding those interactions — the concepts of memory, pointers, undefined behavior, and permissions will help us talk about the more advanced parts of Rust in future chapters.

## Question

### Question 1

Say you are writing a function with the following spec:

`round_all` takes as input a list of floating point numbers, and it rounds each number in-place to the nearest integer.

Which of the following is the most appropriate type signature for a function implementing this spec?

1. ```rust
   fn round_all(v: &mut Vec<f32>);
   ```
2. ```rust
   fn round_all(v: &Vec<f32>) -> Vec<f32>;
   ```
3. ```rust
   fn round_all(v: &Vec<f32>);
   ```
4. ```rust
   fn round_all(v: Vec<f32>);
   ```

**Answer:** _Number 1._ The spec calls for the input to be mutated in-place, therefore the most appropriate type signature accepts a mutable reference to the input. An immutable reference or an owned vector are both inappropriate for this spec.

### Question 2

Say you are writing a function with the following spec:

`find_contains` takes as input a collection of strings and a target substring. It returns a list of all the strings in the collection that contain the target substring.

Which of the following is the most appropriate type signature for a function implementing this spec?

1. ```rust
   fn find_contains(haystack: &Vec<String>, needle: &str) -> &[String];
   ```
2. ```rust
   fn find_contains(haystack: &[String], needle: &str) -> Vec<String>;
   ```
3. ```rust
   fn find_contains(haystack: &Vec<String>, needle: String) -> Vec<String>;
   ```
4. ```rust
   fn find_contains(haystack: &[String], needle: &str) -> Vec<&String>;
   ```

**Answer:** _Number 4_. For `haystack`, the slice type `&[String]` can accept more inputs than `&Vec<String>`, so it is preferred. For `needle`, the target substring does not need to be heap-allocated, so `&str` is preferred to `String`. For the return type, `Vec<String>` is not desirable because it would require cloning the input strings. `&[String]` is not desirable because it can only return a contiguous subsequence of the input. `Vec<&String>` is the most preferable because it only incurs the cost of allocating the vector, not the strings themselves.

### Question 3

Rust normally disallows multiple mutable accesses to the same array, even when those accesses are disjoint. For example, this function does not compile:

```rust
fn main() {
  let mut v = vec![0, 1, 2, 3];
  let (r0, r1) = (&mut v[0..2], &mut v[2..4]);
  r0[0] += 1;
  r1[0] += 1;
}
```

However, the Rust standard library has a function `slice::split_at_mut` that does permit this functionality:

```rust
fn main() {
  let mut v = vec![0, 1, 2, 3];
  let (r0, r1) = v.split_at_mut(2);
  r0[0] += 1;
  r1[0] += 1;
}
```

Which of the following best describes how it's possible to implement `split_at_mut`?

1. [ ] `split_at_mut` uses unsafe code to disable the borrow checker from checking the safety of mutable references
2. [ ] `split_at_mut` calls into a C library that can't be analyzed by Rust
3. [x] `split_at_mut` uses unsafe code to circumvent the borrow checker with raw pointers
4. [ ] `split_at_mut` is a special compiler primitive that cannot be implemented within the language

**Answer:** _Number 3_. As discussed, functions like `split_at_mut` are implemented with the unsafe feature. This feature doesn't completely disable the borrow checker, but rather enables the use of specific unsafe features like raw pointers.

### Question 4

Consider the permissions in the following program:

```rust
let s = String::new();

let s_ref = &s;
```

Which of the following best explains why `*s_ref` does not have the **O (own)** permission?

1. [x] Ownership permits moving, and moving out of a reference can cause a double-free
2. [ ] Ownership permits reading, and reading `*s_ref` can cause a use-after-free
3. [ ] Ownership permits mutation, and mutating `*s_ref` can cause a use-after-free
4. [ ] Ownership permits borrowing, and reborrowing `*s_ref` can cause a double-free

**Answer:** _Number 1_. The **O** permission represents ownership of an object. There can only exist one owner of an object, so it is important that references cannot transfer ownership of non-copyable types like `String`. If two variables thought they owned the same string, then they would both attempt to deallocate it, causing a double-free.

### Question 5

Consider the set of Rust programs that contain no unsafe code. Select each of the following statements that is true about the kinds of programs accepted and rejected by the borrow checker:

1. [ ] The borrow checker always accepts programs without undefined behavior
2. [ ] The borrow checker sometimes accepts programs with undefined behavior
3. [x] The borrow checker sometimes rejects programs without undefined behavior
4. [x] The borrow checker always rejects programs with undefined behavior

**Answer:** _Numbers 3 and 4_. The borrow checker always rejects programs with undefined behavior, but may sometimes reject programs without undefined behavior (i.e., are perfectly safe). In technical terms, the borrow checker is a sound and incomplete analysis.

### Question 6

The function extract is rejected by the borrow checker:

```rust
fn extract(b: &Box<i32>) -> i32 {
    let b2: Box<i32> = *b;
    *b2
}
```

Imagine that the borrow checker did not reject this function. Determine whether there exists an input such that the function would cause undefined behavior if executed on that input.

1. [x] This function COULD cause undefined behavior
2. [ ] This function could NOT cause undefined behavior

Answer: _Number 1_. This function would cause a double-free on any input.

### Question 7

The function `transfer_string` is rejected by the borrow checker:

```rust
fn get_first(strs: &mut (String, String)) -> &mut String {
    &mut strs.0
}

fn get_second(strs: &mut (String, String)) -> &mut String {
    &mut strs.1
}

fn transfer_string(strs: &mut (String, String)) {
    let fst = get_first(strs);
    let snd = get_second(strs);
    fst.push_str(snd);
    snd.clear();
}
```

Imagine that the borrow checker did not reject this function. Determine whether there exists an input such that the function would cause undefined behavior if executed on that input.

1. [x] This function could NOT cause undefined behavior
2. [ ] This function COULD cause undefined behavior

Answer: _Number 1_. The borrow checker rejects this function because it assumes that `get_first` and `get_second` could return a mutable reference to either component of the tuple, and so `fst` and `snd` could possibly point to the same value. But they are always distinct in this program, so this function is actually safe.

> In fact, the original invention of ownership types wasn’t about memory safety at all. It was about preventing leaks of mutable references to data structure internals in Java-like languages. If you’re curious to learn more about the history of ownership types, check out the paper “[Ownership Types for Flexible Alias Protection](https://dl.acm.org/doi/abs/10.1145/286936.286947)” (Clarke et al. 1998).
