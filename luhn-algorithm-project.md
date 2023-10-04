---

title: "Luhn Algorithm"
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

# Luhn Algorithm

![bg](images/intro.png)

---

<!-- header: ' ' -->

## Luhn Algorithm

This is a credit card number validation algorithm.
It takes a string as input and returns a boolean.

[Inspired by "Comprehensive Rust"](https://comprehensive-rust.pages.dev/exercises/day-2/luhn#luhn-algorithm), but we'll do it with a twist.

---

## Luhn Algorithm

* Clone [https://github.com/barafael/luhns-algorithm-exercise](https://github.com/barafael/luhns-algorithm-exercise)

* If you speak german: follow the instructions in the README.

* Else: we'll collab to understand the problem.
  * Or: go [here](https://comprehensive-rust.pages.dev/exercises/day-2/luhn#luhn-algorithm) for a description of the algorithm in english.

* During development, run:

````bash
cargo test
cargo t # shorthand
# or, if you like
cargo watch -x test
````

---

## WASM Surprise

To interact with your program via REPL, run:

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
