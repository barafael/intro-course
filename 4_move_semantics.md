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



---

## Questions?

<iframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
