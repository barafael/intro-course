---

title: "Bindings and Types"
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

# Bindings and Types

## Language Basics:<br>How to bind values to names

![bg blur:1px brightness contrast:0.3](images/waves_orange_lightblue.png)

---

<!-- header: ' '-->

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

## Aside: [`println!`](rust:std::println) and output formatting

To print a value using its "debug representation" (we'll get to it):

````rust tag:playground-button playground-wrap:main
let a = 4;
let b = std::env::current_dir().unwrap();
println!("{:?}", a);
println!("{:?}", b);
````

The `:?` means debug. `:#?` is debug with pretty-print. There are more options, we'll see them later.

---

## Aside: [`println!`](rust:std::println) and output formatting

A single identifier can be inlined in the format string, but expressions of any other sort do not work.

````rust tag:playground-button playground-wrap:main
let pi_half = std::f64::consts::PI / 2.0;
// `f64` has not only debug but also display formatter, so `{a}` works
println!("display: {pi_half}; 5 decimal places: {pi_half:.5}");
````

The pretty-printer can come in handy:

````rust tag:playground-button playground-wrap:main
println!("{:#?}", std::fs::read_to_string("nofile.txt"));
````

---

## Bindings and Scope

A binding is valid only in its own scope (block).
Scopes are enclosed by `{` and `}`.

````rust tag:playground-button playground-wrap:main
let a = 4;
{
    let a = 5;
    let b = false;
    println!("{a}");
}
println!("{a}");
// println!("{b}");
````

Deconstruction at end of scope, in opposite order of construction.

---

## Special Binding Names

````rust tag:playground-button playground-wrap:main
// means: discard whatever is being bound here.
let _ = i_dont_care();
````

Note that this throws the value away immediately (by dtor).

````rust tag:playground-button playground-wrap:main
// means: "`_name` is unused, pls no compiler warnings".
let _name = some_function();
````

Note that this does keep the value alive until the end of the scope.

---

## Binding and Patterns

The part after [`let`](keyword:let) is special - it's a **Pattern**.

Up to now we have only seen simple patterns such as `a`, `name`, or `_`.

Patterns can also destructure the value on the right-hand-side:

````rust tag:playground-button playground-wrap:main
let (before, after) = "012345678".split_at(4);
println!("{before}, then {after}");
````

There is more to this. Patterns will come in again and again later!

---

## Mutable Bindings

A binding can simply be made mutable by the [`mut`](keyword:mut)

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(a);}"$
let mut a = 5;
a += 4;
````

This works inside destructuring patterns, too:

````rust marker:mutable_binding_in_destructuring_pattern

````

---

## Mutable Bindings

A mutable binding means the bound value may be changed through this binding.

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

We will see later exactly how this works.

---

## Preview: Server Loop with only one [`mut`](keyword:mut)

````rust marker:server_cliche_loop hide_other_markers

````

---

## What about Types?

Every binding has a type. You can usually omit it. The [Hindley-Milner Type Inference](https://reasoning.page/2022/12/11/hindley-milner-type-inference-in-rust/) algorithm determines the type of all bindings. No need to worry about it, it gets out of the way.

To manually annotate the type of a bound value:

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

## Ambiguous Integer Types

Sometimes the inference cannot help:

````rust tag:playground-button playground-wrap:main
let abs = 3.abs();
````

---

## Ambiguous Integer Types

Sometimes the inference cannot help:

````rust tag:playground-button playground-wrap:main
let abs = 3.abs();
````

````
error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`
 --> src/main.rs:2:17
  |
2 |     let abs = 3.abs();
  |                 ^^^
  |
help: you must specify a concrete type for this numeric value, like `i32`
  |
2 |     let abs = 3_i32.abs();
  |               ~~~~~
````

---

## Ambiguous Integer Types

With type annotation, it isn't a problem:

````rust tag:playground-button playground-wrap:main
let abs: u8 = 3.abs();
````

Of course, you can also follow the compiler suggestion:

````rust tag:playground-button playground-wrap:main
let abs = 3_i32.abs();
````

---

## Providing Context for Type Inference

Type inference needs some context information to do its magic.

````rust tag:playground-button playground-before:$"fn main(){"$ playground-after:$"dbg!(result);}"$
let abs = 3.abs();
let result = abs + 1u8;
````

We will see later that function signatures require stating a full type.
This vastly simplifies type inference (and obviously readability!).

---

## Questions?

<iframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
