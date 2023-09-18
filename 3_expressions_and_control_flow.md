---

title: "Expressions and Control Flow"
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

# Expressions and Control Flow

![bg blur:1px brightness:1.1 contrast:1.3](images/triangles_blue_to_orange.png)

---

## Overview

![bg blur:6px left:33%](images/gears.png)

Ultimately, coding is **expression**.

It shouldn't be surprising that control flow is also just an **expression**.

---

<!-- header: ' '-->

## Expressions are declarative

Expression: specifies a calculation to be performed.

Expression:
````rust tag:playground-button playground-before:$"fn main() { let vals = [1,2,3,4,5,6,7];"$ playground-after:$";}"$
vals[3] + u8::min(5, 6)
````

An expression can be suppressed via `;`.
This can be used to bind a value to a name ([`let`](keyword:let)):

````rust
let a = vals[3] + u8::min(5, 6);
````

---

## Statements are imperative

Statement: specifies an action to be performed.

Statement(-like expression):

````rust
*GLOBAL_HANDLE.lock().unwrap() = Some(handle);
````

Assignments (via [`let`](keyword:let) or like above) really are re-bindings of new values to existing bindings.

---

## Expressions in blocks

A block can be used to group expressions.

````rust tag:playground-button playground-wrap:main
let a = { 5 };
````

A block with a sequence of "statements" evaluates to the value of the last expression in it:

````rust tag:playground-button playground-before:$"fn main() {"$ playground-after:$"dbg!(a);}"$
let a = {
    let a = 1 + 2;
    let b = a + a;
    b
};
````

---

## "Special Blocks": `if`

If-expressions can be viewed as special block expressions.

````rust tag:playground-button playground-before:$"fn main() {let a = 4;"$ playground-after:$"dbg!(value);}"$
let value = if a < 7 { "less" } else { "greater-equals" };
````

This is just the "ternary operator", generalized.

---

## "Special Blocks": [`loop`](keyword:loop)

In a function returning `Ok(())` or `Err(...)`
````rust marker:break_loop_with_value

````

---

## Questions?

<jframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
