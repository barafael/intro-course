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

---

## Overview

"All of these things": `struct`


"Any of these things": `enum`

And that's it!

---

## Product Types (`struct`s)

A `struct` is just a "Plain Old Data" type. Not a class. It is comprised of **members**.

````rust tag:playground-button
struct Message {
    from: Option<String>,
    to: String,
    content: String,
}
````

Why product type? The number of different values is the product of the number of different values of each member.

## Questions?

<iframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
