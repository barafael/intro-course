---

title: "Data and Structure"
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

# Data and Structure

## Language Basics: How to structure data

![bg blur:1px brightness:1.1 contrast:1.3](images/triangles_blue_to_orange.png)

---

## Overview

![bg blur:6px left:33%](images/gears.png)

"All of these things": `struct`

"Any of these things": `enum`

And that's it!

---

## `struct`s ("Product Types")

A `struct` is just a "Plain Old Data" type. Not a class. It is comprised of **members**.

````rust tag:playground-button
struct Message {
    from: Option<String>,
    to: String,
    content: String,
}
````

---

## Con`struct`ion

There is only one way to initialize a `struct`. Looks a little like a C++ initializer list.

````rust tag:playground-button playground-before:$"struct Message { from: Option<String>, to: String, content: String, }fn main() {"$ playground-after:$"}"$
let msg = Message {
    from: None,
    to: "Bertha".to_string(),
    content: "Hey, a struct".to_string(),
};
````

---

## But how can we print it?

Well, just use `println!`?

````rust tag:playground-button playground-before:$"struct Message { from: Option<String>, to: String, content: String, }fn main() {"$ playground-after:$"}"$
let msg = Message {
    from: None,
    to: "Bertha".to_string(),
    content: "Hey, a struct".to_string(),
};
println!("{msg}");
````

---

## Pls can we print it?

````rust tag:playground-button playground-before:$"struct Message { from: Option<String>, to: String, content: String, }fn main() {"$ playground-after:$"}"$
let msg = Message {
    from: None,
    to: "Bertha".to_string(),
    content: "Hey, a struct".to_string(),
};
println!("{msg}");
````

````
error[E0277]: `Message` doesn't implement `std::fmt::Display`
  --> src/main.rs:12:15
   |
12 |     println!("{msg}");
   |               ^^^^^ `Message` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Message`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
````

---

## I just want to print it!

````rust tag:playground-button playground-before:$"struct Message { from: Option<String>, to: String, content: String, }fn main() {"$ playground-after:$"}"$
let msg = Message {
    from: None,
    to: "Bertha".to_string(),
    content: "Hey, a struct".to_string(),
};
println!("{msg:#?}");
````

---

## Arghh why is this so hard

````
error[E0277]: `Message` doesn't implement `Debug`
  --> src/main.rs:12:15
   |
12 |     println!("{msg:#?}");
   |               ^^^^^^^^ `Message` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Message`
   = note: add `#[derive(Debug)]` to `Message` or manually `impl Debug for Message`
   = note: ...
help: consider annotating `Message` with `#[derive(Debug)]`
   |
1  + #[derive(Debug)]
2  | struct Message {
   |
````

---

## WTF is this

````rust
#[derive(Debug)]
struct Message {
    from: Option<String>,
    to: String,
    content: String,
}
````

Shh... A `derive` may be used to extend a type with a derivable trait - more on this later.

---

## WTF is this

````rust tag:playground-button playground-before:$"#[derive(Debug)] struct Message { from: Option<String>, to: String, content: String, }fn main() {"$ playground-after:$"}"$
let msg = Message {
    from: None,
    to: "Bertha".to_string(),
    content: "Hey, a struct".to_string(),
};
println!("{msg:#?}");
````

---

## Finally :)

````rust tag:playground-button playground-before:$"#[derive(Debug)] struct Message { from: Option<String>, to: String, content: String, }fn main() {"$ playground-after:$"}"$
let msg = Message {
    from: None,
    to: "Bertha".to_string(),
    content: "Hey, a struct".to_string(),
};
println!("{msg:#?}");
````

````
Message {
    from: None,
    to: "Bertha",
    content: "Hey, a struct",
}
````

---

## Preview: Deriving Traits

`derive`-ing just means generating Code for a type.
This code comes in the form of a [keyword:trait] like [rust:std::fmt::Debug].

````rust tag:playground-button
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Message {
    from: Option<String>,
    to: String,
    content: String,
}
````

The Rust API guidelines recommend [eagerly deriving common traits](https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits). Above, traits like `Default` are possible but may violate semantics.

---

## Preview: Deriving Traits

Don't overdo it though :)

````rust
#[derive(
	clap::ValueEnum, PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Serialize, Deserialize, Default,
)]
pub enum Level {
	Trace = 0,
	Debug,
	#[default]
	Info,
	Warn,
	Error,
	Critical,
	Off,
}
````

<!--
_footer: 'From: [Level](https://github.com/microsoft/vscode/blob/6b9583d2dc4140e0db51d8037643e5ce8763cb0c/cli/src/log.rs#L29C1-L41) in the 0.9% of VSCode that are written in Rust' -->

---

## Tuple Struct (unnamed members)

Sometimes, it is not necessary/possible to give the members good names.

````rust tag:playground-button
struct Version(u32, u32, u32);
````

Realistic example: [rust:std::sync::mpsc::SendError]

In reality, this is rare on it's own.

---

## Unit Struct (no members)

Just an empty struct.

````rust tag:playground-button
struct StatelessCodec;
````

Sometimes a unique type is required but it has no meaningful members.

Guess the size in bytes?

````rust tag:playground-button playground-before:$"struct StatelessCodec; fn main() {"$ playground-after:$"}"$
println!("{}", std::mem::size_of::<StatelessCodec>());
````

---

## Now what is a Product Type?

Just a term from type theory.
The number of different values is the product of the number of different values of each member.

````rust
(): 1 possible value
bool: 2 possible different values
u8: 256 possible different values

struct Bunch((), bool, u8); // 1 * 2 * 256 values
````

---

## Is there also a Sum Type then?

An [keyword:enum] is a type with a number of variants.
Each variant carries data. The enum is exactly one variant at any time.

````rust tag:playground-button
enum Event {
    Reload,
    Shoot {
        x: u32,
        y: u32,
    },
    Mana(f32),
}
````

In the binary, this looks like a "discriminated union" in C.

---

## Why "Sum Type"?

The syntax in the variants is exactly like the syntax for `struct`s - that's no coincidence.

````rust
(): 1 possible value
bool: 2 possible different values
u8: 256 possible different values

// 1 + 2 + 256 possible different values
enum Any {
    Unit,
    Boolean(bool),
    Byte(u8),
}
````

---

## Questions?

<iframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
