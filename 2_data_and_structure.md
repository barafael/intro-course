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

A `struct` is just a "Plain Old Data" type. Not a class.
It is comprised of **members**.

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

In reality, this is rare on it's own (but we'll see why it exists).

---

## Unit Struct (no members)

Just an empty struct.

````rust tag:playground-button
struct StatelessCodec;
````

Sometimes a unique type is required but it has no meaningful members.

[docsrs:https://docs.rs/tokio/latest/tokio/time/error/struct.Elapsed.html] struct has no ([keyword:pub]) members, used as a marker

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

## Size of a `struct`

For a product type, the size in bytes is the sum of the size of the members.

````rust tag:playground-button playground-before:$"struct Bunch((), bool, u8); fn main() {"$ playground-after:$"}"$
println!("{}", std::mem::size_of::<Bunch>());
````

This is true for member-less [keyword:struct]s too:

````rust tag:playground-button playground-before:$"struct StatelessCodec; fn main() {"$ playground-after:$"}"$
println!("{}", std::mem::size_of::<StatelessCodec>());
````

---

## Memory Layout, Alignment and Padding

Rust **does not** have a stable ABI. The compiler is free to determine the memory layout of [keyword:struct] members as it pleases. For example, it can sort the members by size, so that least padding is required.

To manually determine the binary layout of a struct:

````rust marker:simple_misaligned_reprc_struct

````

<table style="width:100%; table-layout: fixed; text-align: center">
    <tr>
        <td><code>u8</code>     </td>
        <td><code>u8</code>     </td>
        <td><code>d/c</code>    </td>
        <td><code>d/c</code>    </td>
        <td><code>u32[0]</code> </td>
        <td><code>u32[1]</code> </td>
        <td><code>u32[2]</code> </td>
        <td><code>u32[3]</code> </td>
    </tr>
</table>

---

## Memory Layout, Alignment and Padding

````rust marker:print_meta

````

---

## Memory Layout, Alignment and Padding

````rust marker:print_meta

````

This prints:

````
type name: to_byte_slice::tests::misaligned_struct::A,
len: 8,
bytes: 1, 2, 0, 0, 4, 0, 0, 0
````

---

## Memory Layout, Alignment and Padding (Packed)

You can **pack** struct members, of course.

````rust marker:simple_misaligned_reprpacked_struct

````

<table style="width:100%; table-layout: fixed; text-align: center">
    <tr>
        <td><code>u8</code>     </td>
        <td><code>u8</code>     </td>
        <td><code>u32[0]</code> </td>
        <td><code>u32[1]</code> </td>
        <td><code>u32[2]</code> </td>
        <td><code>u32[3]</code> </td>
    </tr>
</table>

---

## Memory Layout, Alignment and Padding (Packed)

````rust marker:print_meta_packed

````

---

## Memory Layout and Padding (Packed)

````rust marker:print_meta_packed

````

This prints:

````
type name: to_byte_slice::tests::misaligned_struct_packed::A,
len: 6,
bytes: 1, 2, 4, 0, 0, 0
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

In the binary, this looks like a "discriminated union" in C (we'll see).

---

## What does "Sum Type" mean?

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

## Example: `Maybe<T>` type

Simple type borrowed from Haskell land:

````rust marker:enum_type_maybe

````

This type signifies a value that may or may not be present.
Don't mind the generic `T`.

---

## Memberless Enum?

In the same way where a member-less [keyword:struct] has exactly one possible value isomorphic to `()`, a variant-less [keyword:enum] has **no** possible value.

````rust tag:playground-button playground-wrap:main
enum Impossible {}
//let x = Impossible::?;
assert_eq!(0, std::mem::size_of::<Impossible>());
````

This fact is exploited by [rust:std::convert::Infallible]: it is an un-instantiable type.

<!-- _footer: '1: neutral element of multiplication, 0: neutral element of summation' -->

---

## How does a Sum Type look in C?

Let's examine this enum:

````rust marker:disc_union_enum

````

<!-- _footer: 'Inspired by [`serde_json::Value`](https://docs.rs/serde_json/1.0.106/serde_json/enum.Value.html)'-->

---

## C Tag Enum

Let's [mozilla/cbindgen](https://github.com/mozilla/cbindgen) to look at the C representation.
The generated "Tag Enum":

````c marker:disc_union_tag_enum

````

Note that C enums are simply global integers with compiler-assigned values.

---

## C Struct `Value`

The wrapping `struct` with the "tag" and "union":

````c
typedef struct Value {
  Value_Tag tag;
  union {
    struct {
      bool bool_;
    };
    // ...
  };
} Value;
````

---

## Size of an [keyword:enum]

An [keyword:enum] needs space to store the "discriminant": the value that marks the valid variant. The size of the discriminant depends on the number of variants of the [keyword:enum].

**Add**itionally, the [keyword:enum] must have space to store the (aligned) data for the largest variant.

---

## Questions?

<iframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
