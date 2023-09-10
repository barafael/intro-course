---

title: "Luhns Algorithm"
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

# Luhns Algorithm

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