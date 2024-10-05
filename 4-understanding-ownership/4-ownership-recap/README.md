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

_In Processing_
