---

title: "Move Semantics"
description: "A sensible default"
marp: true
theme: rhea
color: ""
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

# Move Semantics & Ownership

## Sensible Defaults

![bg blur:1px brightness:1.1 contrast:1.3](images/colorkit%20(2).png)

---

## Overview

![bg left:33%](images/conveyor.jpeg)

Move Semantics and Ownership:
Ideas at the core of Rust

Implications for API design

Implications for Memory and Resource Management

---

<!-- header: ' '-->

## Ownership

**Owning** a value gives complete control - 
it comes with the responsibility to clean up the value.

Every value in Rust has exactly one owner.

Ownership can transfer using a **move**.

---

## Moving Ownership

Guess the output:

````rust tag:playground-button playground-before:$"fn main() { #[derive(Debug, Default)] struct Thing;"$ playground-after:$"}"$
let owner = Thing::default(); // `owner` holds a value of type `Thing`.
let new_owner = owner; // Ownership of the value moves to `new_owner`.
println!("{owner:?}");
````

<div data-marpit-fragment>

Compiler rejects:

````rust
error[E0382]: borrow of moved value: `owner`
4 |     let owner = Thing::default(); // `owner` holds a value of type `Thing`.
  |         ----- move occurs because `owner` has type `Thing`, which does not implement the `Copy` trait
5 |     let new_owner = owner; // Ownership of the value moves to `new_owner`.
  |                     ----- value moved here
6 |     println!("{owner:?}");
  |               ^^^^^^^^^ value borrowed here after move
````

</div>

---

## Move Semantics

````rust tag:playground-button playground-wrap:main
type Dough = Vec<String>;

#[derive(Debug)]
enum Baked {
    Cake,
    Bread,
}

fn bake(d: Dough) -> Baked {
    if d.iter().any(|e| e == "egg") {
        Baked::Cake
    } else {
        Baked::Bread
    }
}
````

<!-- here, bake takes a dough and consumes it, producing something new -->

<!-- pass by move -->

---

## Strict APIs with "Pass-by-Move"

This API is (maybe a little too) strict:

````rust tag:playground-button
fn uppercase(s: String) -> String {
    s.chars().map(|c| c.to_uppercase().to_string()).collect::<String>()
}
````

<div data-marpit-fragment>

However, the API is quite "honest": uppercasing may require (re-)allocation (ß -> SS).
This function always deallocates, then allocates.
It's an opinionated interface!

</div>

---

## Processing a `Qexpr` with Move Semantics

````rust tag:playground-button
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Qexpr(pub(crate) VecDeque<Value>);

impl Qexpr {
    pub fn into_sexpr(self) -> Sexpr {
        Sexpr(self.0)
    }

    pub fn head(mut self) -> Result<Value, anyhow::Error> {
        self.0.pop_front().context("'head' on empty qexpr")
    }
}
````

<!-- _footer: 'From: [pils](https://github.com/barafael/pils), demo at [https://barafael.github.io/pils/](https://barafael.github.io/pils/)' -->

---

## Mutex and Guard containing data

A simple mutex that owns its data:

````rust marker:simple_mutex

````

The data can only be mutated through the guard, which is limited by a lifetime.

---

## Move implies Ownership

Ownership of a resource (file, reference, mutex):
- Obligation to eventually **drop** (clean up) the resource
- Power to **move** (transfer) this resource to others
- Power to **borrow** (share) this resource with others

---

## What is this `Copy` thing?

This message from earlier:

````
  |         ----- move occurs because `owner` has type `Thing`, which does not implement the `Copy` trait
5 |     let new_owner = owner; // Ownership of the value moves to `new_owner`.
  |                     ----- value moved here
````

hints that `Copy` can bypass move semantics.

---

## Preview: The [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) Marker

The trait [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) marks a type which can trivially be copied bit-wise.

[Implementors](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors) of [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) include the basic scalar types, [`AsciiChar`](rust:std::ascii::Char), [`IpAddr`](rust:std::net::IpAddr), [`SocketAddr`](rust:std::net::SocketAddr), [`ExitCode`](rust:std::process::ExitCode), [`Instant`](rust:std::time::Instant), ...

<div data-marpit-fragment>

Copying a value does not move it:

````rust tag:playground-button playground-wrap:main
let now = std::time::Instant::now();
let now_too = now;
println!("{now:?}");
````

</div>

---

## References

An owner of a value may hand out references to it:

````rust tag:playground-button playground-before:$"fn main() { #[derive(Debug, Default)] struct Thing;"$ playground-after:$"}"$
let thing = Thing::default();
let reference = &thing;
let another_reference = reference;
````

Note that references are [`Copy`](rust:std::marker::Copy).

---

## References

References are literally pointers, but a few extra rules apply:

- cannot be null
- cannot be misaligned
- cannot dangle (point to deallocated resources)
- cannot be used to write

<!-- _footer: '[barafael/errare-humanum-est](https://github.com/barafael/errare-humanum-est)' -->

---

## Sharing Data by Reference

Shared references may be shared without limits:

````rust marker:sharing_data_with_threads

````

Here, the shared reference is copied, then moved to the new thread.

<!-- _footer: 'Lifetimes play a role here, though.' -->

---

## Blanket impls which enable sharing

Shared references to any unconstrained `T` are marked [`Copy`](rust:std::marker::Copy):

````rust
/// Shared references can be copied, but mutable references *cannot*!
impl<T: ?Sized> Copy for &T {}
````

<!-- _footer: '[impl block in std](https://doc.rust-lang.org/src/core/marker.rs.html#499-501)' -->

---

## Blanket impls which enable sharing

Shared references to types which may be accessed from more than one thread may be moved over a thread boundary:

````rust
unsafe impl<T: Sync + ?Sized> Send for &T {}
````

<!-- _footer: '[impl block in std](https://doc.rust-lang.org/src/core/marker.rs.html#91-95)' -->

---

## Questions?

<jframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
