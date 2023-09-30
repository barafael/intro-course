---
marp: true
theme: rhea
color: "dark-gray"

---

# First steps

**_Prerequisite:_**  working Rust toolchain setup

Run in a shell:

```
$> cargo new exploring-rust

$> tree exploring-rust
exploring-rust/
├── Cargo.toml
└── src
    └── main.rs
```

---

# Hello World

In `exploring-rust/src/main.rs`, a classic:

```rust
fn main() {
    println!("Hello world!");
}
```

We can see: `fn` keyword, parens/braces, semicolon.

The `main` function is the default entry point for binaries. For libraries, it's different.

---

# Function basics

Functions can take arguments:

```rust
fn sum_and_print(a: f32, b: f32) {
    println!("a + b = {}", a + b);
}
```

And they can return things:

```rust
fn just_sum(a: f32, b: f32) -> f32 {
    a + b
}
```

---

# Expression orientation

The previous functions returned an `f32` without using the `return` keyword.
Rust is an expression-oriented language:

```rust
let small = if a < b { a } else { b }; // "ternary operator"

```

---

# Variables

A `let` _binds_ a value to a name.

Mutable bindings can be created with `let mut ...`. It's rare!

**_Advanced_**: let can also be used to destructure data:

---


## Aside: Non-lexixal lifetimes

Sometimes, items are cleaned up **before** the end of their scope.
This happens when 


Conditional blankets:
impl<T: ?Sized + Hash> Hash for &T {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}
