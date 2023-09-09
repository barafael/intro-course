---

title: "The Rust Bare Minimum Package"
description: Is 10% Rust enough?
marp: true
theme: rhea
color: "dark-gray"
size: 16:9

---

<!--
paginate: true
 -->
<!-- 
_footer: ''
_paginate: false
 -->
<!-- _class: lead -->

# The Rust Bare Minimum Package

## Is 10% Rust enough?

---

## Goals

* Familiarize with the basic language - variables, functions, datastructures, control flow.

* Side goal: Get to know the presentation format (playground buttons, inline links, local snippets)

* Finally: write our first little useful program - an implementation of the Luhn Algorithm (with a surprise!)

---

## There Are No Variables

There are **Bindings** instead! It's a conceptual difference.

````rust tag:playground-button playground-wrap:main
let age = 4;
let name = "Eric";
````

**Values** are bound to **Names**.

These bindings above are **immutable**!

---

## Immutable?

Try this:

````rust tag:playground-button playground-wrap:main
let age = 4;
age += 1;
````

---

## Immutable?

Try this:

````rust tag:playground-button playground-wrap:main
let age = 4;
age += 1;
````

````
error[E0384]: cannot assign twice to immutable variable `age`
2 |     let age = 4;
  |         ---
  |         first assignment to `age`
  |         help: consider making this binding mutable: `mut age`
3 |     age += 1;
  |     ^^^^^^^^ cannot assign twice to immutable variable
````

---

## Immutable!

An immutable binding to a value means:

the value cannot be changed through that binding.

The Mutable/Immutable distinction is important.
It plays a role in many areas of Rust.

Example: Immutability is the trivial way to achieve safe sharing of data in more than one thread.

---

## Mutable Bindings

A binding can simply be made mutable by the [keyword:mut](keyword:mut)

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(acc);}"$
let mut acc = 0;
for i in [1, 2, 3, 4] {
    acc += i;
}
````

Somewhat surprisingly, you can often get away without mutability.

---

## Preview: Iterator Combinators

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(sum);}"$
let sum = [1, 2, 3, 4].iter().fold(0, |acc, n| acc + n);
````

Fold is like `reduce` in other languages.

More concisely:

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(sum);}"$
let sum = [1, 2, 3, 4].iter().sum::<u8>();
````

We will see later how this works.

---

## What about Types?

Every binding has a type. You can usually omit it. The [Hindley-Milner Type Inference](https://reasoning.page/2022/12/11/hindley-milner-type-inference-in-rust/) algorithm determines the type of all bindings. No need to worry about it, it gets out of the way.

To manually specify the type of a binding:

````rust tag:playground-button playground-wrap:main
let x: u8 = 5;
let name: String = "Bohica".to_string();
````

---

## Inline Type Annotations

Some literals may be annotated with type information:

````rust tag:playground-button playground-wrap:main
let some_byte/*: u8*/ = 0u8;
let byte_str/*: &[u8]*/ = b"bytes! Just bytes! No Unicode!";
````

The default type of integer literals is assumed to be `i32`:

````rust tag:playground-button playground-before:$"fn print_type_of<T>(_: T) { println!("{}", std::any::type_name::<T>()) }fn main() {"$ playground-after:$"}"$
print_type_of(3);
````

---

## How to find the type of a value

<style scoped>
img {
  display: block;
  margin: 0 auto;
}
</style>

Either use an IDE with inlay hints via Rust-Analyzer:

![width:800px](images/inlay-hint-string-format.png)

This is especially useful with iterator chains:

![width:800px](images/inlay-hint-iterator-chain.png)

---

## How to find the type of a value

If IDE isn't available, just briefly annotate the wrong type:

````rust tag:playground-button playground-wrap:main
let path: bool = std::env::temp_dir();
````

---

## How to find the type of a value

If IDE isn't available, just briefly annotate the wrong type:

````rust tag:playground-button playground-wrap:main
let path: bool = std::env::temp_dir();
````

````
error[E0308]: mismatched types
 --> src/main.rs:2:19
  |
2 |     let path: bool = std::env::temp_dir();
  |               ----   ^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `PathBuf`
  |               |
  |               expected due to this
````

---

## Basic Types

<iframe style="margin-top:5%" width="100%" height="80%" src="https://doc.rust-lang.org/std/#primitives"></iframe>
<!--
_footer: '[https://doc.rust-lang.org/std/#primitives](doc.rust-lang.org/std/#primitives)'
 -->

---

In some cases, you do have to specify the type:

* Function signatures

---

## Immutability and References

Even when the value is a reference, an immutable binding still means the value cannot be changed.

Do these compile? If not, why?

````rust tag:playground-button playground-wrap:main
let a = 1;
let b = 5;
let ref_a = &a;
ref_a = &b;
*ref_a = b;
````

---

## Immutable?

Try this:

````rust tag:playground-button playground-before:$"fn main() {\n"$ playground-after:$"}"$ playground-indent
let age = 4;
age++;
````

Why is `++` not a thing in Rust?

---

## First Project! Luhn Algorithm

* Clone [https://github.com/barafael/luhns-algorithm-exercise](github.com/barafael/luhns-algorithm-exercise)

* If you speak german: follow the instructions in the README.

* Else: we'll collab to understand the problem.

* During development, run:

````bash
cargo test # or
cargo t # or, if you like
cargo watch -x test
````

---

## WASM Surprise

To test your program, run:

````bash
wasm-pack build --target web
````

then in the same directory, host a local server:

````bash
python -m http.server
````

And open the hosted page!

---

## Example Solution

[https://github.com/barafael/luhney](barafael/luhney) solves this problem one way.
In `bin/`: REPL and a CLI interface examples.

<iframe style="margin-top:5%" width="100%" height="60%" src="https://barafael.github.io/luhney/">
</iframe>

---

re-bindings and why they are idiomatic
expressions vs. statements

chapter on setup