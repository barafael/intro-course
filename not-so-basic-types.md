---

title: "Not so basic types"
description: ""
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

# Not so basic types

---

## Goals

---

## Basic Types

<iframe style="margin-top:5%" width="100%" height="80%" src="https://doc.rust-lang.org/std/#primitives"></iframe>
<!--
_footer: '[https://doc.rust-lang.org/std/#primitives](doc.rust-lang.org/std/#primitives)'
 -->

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

## Notable Basic Types: Unit

The unit type `()` is the only type that has only one value. Therefore, the value and the type name are the same.

````rust tag:playground-button playground-wrap:main
let (): () = println!("Println doesn't return anything");
````

The unit type is zero-sized - it exists only at compile time. It is a first-class citizen.

This means you can use it as a value in a datastructure normally.

---

## Notable Basic Types: Never

The never type ([`!`](rust:std::convert::Infallible) or [`Infallible`](rust:std::convert::Infallible)) is somewhat experimental still.

It denotes the return value of a diverging function, such as [`loop {}`](keyword:loop ), [`panic!()`](keyword:panic), [`return`](keyword:return) or [`break`](keyword:break).

The never type is a subtype of every type, which means it can be converted to every type out there.

````rust tag:playground-button playground-before:$"#![feature(never_type)]fn count() -> u8 {"$ playground-after:$"}"$ playground-channel:nightly
let never: ! = return 123;
````

---

## Type Annotations as Degrees of Freedom

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(sum);}"$
let sum = [1, 2, 3, 4].iter().sum/*::<u8>*/();
````

````
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let sum = [1, 2, 3, 4].iter().sum();
  |         ^^^
  |
help: consider giving `sum` an explicit type
  |
2 |     let sum: /* Type */ = [1, 2, 3, 4].iter().sum();
  |            ++++++++++++
````

<!-- 
_footer: 'This thing is called the [turbofish](http://turbo.fish)'
 -->

---

## Type Annotations as Degrees of Freedom

While parsing, you need to tell what type you are expecting:

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(parsed);}"$
let parsed = String::from("1337").parse::<usize>().unwrap();
````

Simpler, with normal type specification:

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(parsed);}"$
let parsed: usize = String::from("1337").parse().unwrap();
````

This works for all types implementing [rust:`std::str::FromStr`] (see signature of [rust:`str::parse`]).

---

## Type Annotations as Degrees of Freedom

While parsing, you need to tell what type you are expecting:

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(config);}"$
#[derive(Debug, serde::Deserialize)]
struct Config {
    value: u8,
}

let config: Config = serde_json::from_str(r#"{"value": 3}"#).unwrap();
````

---

turbofish
types in function signatures