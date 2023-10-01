---
title: Collections and Iteration
marp: true
theme: rhea
color: "dark-gray"

---

<!-- 
footer: " "
 -->

<!--
paginate: true
 -->

<!-- 
_footer: ''
_paginate: false
 -->

<!-- _class: lead -->

# Collections and Iteration

<br>

### It's Iterators all the way down!

![bg](images/intro.png)

---

## How are Algorithms and Datastructures organized in [`std`](rust:std)?

## What are the differences between the Rust collections and the containers in the C++ STL?

## Overview of the idiomatic APIs in [`std`](rust:std)?

---

<!-- _class: lead -->

## Iterators and `#include<algorithm>`

### How to operate generically on Collections?

---

<!-- header: ' ' -->

## Basics: Rust [`Iterator`](rust:std::iter::Iterator)

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

---

## Basics: Rust [`Iterator`](rust:std::iter::Iterator)

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Very many concepts in this one snippet!

* Associated type [`Item`](rust:std::iter::Iterator::Item): allows the trait implementor to specify the element type
* Method [`next`](rust:std::iter::Iterator::next): returns `Some(Item)` until the iterator is empty, then `None`.

---

## Iterators are inherently lazy

An iterator must be actively used to yield elements.

````rust tag:playground-button playground-wrap:main
let values = vec![1, 2, 3];
let iter = values.iter();
iter.map(|val| val + 1);
````

---

## Iterators are inherently lazy

An iterator must be actively used to yield elements.

````rust tag:playground-button playground-wrap:main
let values = vec![1, 2, 3];
let iter = values.iter();
iter.map(|val| val + 1);
````

````
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     iter.map(|val| val + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
````

---

## Implementing the Iterator Trait

Let's implement the famous "fizzbuzz" via an iterator.

We must store the iterator state somewhere:

````rust marker:fizzbuzz_struct

````

---

## Implementing the Iterator Trait

Our [`Item`](rust:std::iter::Iterator::Item) type is a [`String`](rust:String) to print.

````rust marker:fizzbuzz_iterator_trait_impl_1

````

---

## Implementing the Iterator Trait

It's a match!

````rust marker:fizzbuzz_iterator_trait_impl_2

````

---

## Implementing the Iterator Trait

Here's how to use it (we'll see more on this):

````rust marker:first_20_elems

````

---

## Constructing Iterators from Collections

Most collections offer three iterator functions:

* `iter()`: iterate over shared references (`&T`) to the items
* `iter_mut()`: iterate over unique, mutable references (`&mut T`) to the items
* `into_iter()`: iterate "by value", consuming the datastructure in the process

---

## Iterators and For-loops

The type of the expression in a for-loop determines the type of iteration (shared, mutable, owned):

````rust tag:playground-button playground-before:$"#![feature(type_name_of_val)] fn main() {"$ playground-after:$"}"$ playground-channel:nightly
let mut some_vec = vec![1, 2, 4, 8];
for value in &mut some_vec {
    println!("{}", std::any::type_name_of_val(&value));
    break;
}
````

Try changing the expression being iterated upon!

<!-- _footer: 'This uses [`type_name_of_val`](https://doc.rust-lang.org/std/any/fn.type_name_of_val.html)' -->

---

## Custom Iterator Constructors

Some collections offer interfaces to iterate over aspects of their contents:
````rust tag:playground-button playground-before:$"use std::collections::HashMap;"$
fn sum_values<K>(map: &HashMap<K, u32>) -> u32 {
    let mut sum = 0;
    for value in map.values() {
        sum += value;
    }
    sum
}
````

---

## Iterator combinators: [`Iterator::sum`](rust:std::iter::Iterator::sum)

The trait [`Iterator`](rust:std::iter::Iterator) only requires implementing [`Iterator::next`](rust:std::iter::Iterator::next).
The trait defines common generic combinators:

````rust tag:playground-button playground-before:$"use std::collections::HashMap;"$
fn sum_values<K>(map: &HashMap<K, u32>) -> u32 {
    map.values().sum()
}
````

---

## Iterator combinators: [`Iterator::all`](rust:std::iter::Iterator::all)

The [`Iterator::all`](rust:std::iter::Iterator::all) combinator evaluates a predicate for all items:

````rust tag:playground-button playground-before:$"fn main() { let name = "Teddie";"$ playground-after:$"}"$
assert!(name.chars().all(|c| c.is_ascii_alphanumeric()));
````

<!-- _footer: "[Protohackers Budget Chat](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/budget_chat/src/main.rs#L56)" -->

---

## Iterator combinators: [`Iterator::any`](rust:std::iter::Iterator::any)

The dual of [`Iterator::all`](rust:std::iter::Iterator::all) is [`Iterator::any`](rust:std::iter::Iterator::any).

In this example, tickets may only be dispatched if on that particular day none have been dispatched yet:
```rust
if Self::days(ticket.timestamp1, ticket.timestamp2)
    .any(|day| already_ticketed_days.contains(&day))
{
    let day = Self::day(ticket.timestamp1);
    tracing::info!("Ignoring ticket starting on {day}: {ticket:?}");
}
```

<!-- _footer: "[Protohackers Speedd](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/speedd/src/collector.rs#L75)" -->

---

## Iterator combinators: [`Iterator::map`](rust:std::iter::Iterator::map)

[`Iterator::map`](rust:std::iter::Iterator::map) implements a bijective mapping of the elements in one iterator to the elements in a new one.
The function passed determines the type of the new iterator.

````rust tag:playground-button playground-before:$"fn main() { let input = "1 a b 4 5 66 7 a";"$ playground-after:$"}"$
input
    .split_whitespace()
    .map(parse_hex_digit)
    .for_each(|item| {
        println!("{item}");
    });
````

<!-- _footer: "[Protohackers Netcrab](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/netcrab/src/main.rs#L32-L36)" -->

---

## Aside: `Result<Vec<T>, E>` or `Vec<Result<T, E>>`?

Note: `parse_hex_digit` returns a `Result<T, E>`, so the iterator item type becomes `Iterator<Item = Result<u8, Error>>`

What we want is a `Result<Vec<T>, E>`.

[`Iterator::collect`](rust:std::iter::Iterator::collect) can do just that, and more!

---

## Kombinatoren für Iteratoren: `filter`

Filter nimmt ein Prädikat und behält nur die Elemente des Iterators, für welche das Prädikat wahr ist.

```rust
let mut iter: Filter<Iter<'_, i32>, _> = a.iter().filter(|x| x.is_positive());
```

<div data-marpit-fragment>

einfacher:
```rust
let mut iter: Filter<_, _> = a.iter().filter(|x| x.is_positive());
```

```rust
let mut iter = a.iter().filter(|x| x.is_positive());
```

</div>

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+core%3A%3Aiter%3A%3AFilter%3B%0Ause+std%3A%3Aslice%3A%3AIter%3B%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B0i32%2C+-1%2C+2%2C+-3%2C+4%5D%3B%0A++++let+mut+iter%3A+Filter%3CIter%3C%27_%2C+i32%3E%2C+_%3E+%3D+a.iter%28%29.filter%28%7Cx%7C+x.is_positive%28%29%29%3B%0A++++%0A++++while+let+Some%28n%29+%3D+iter.next%28%29+%7B%0A++++++++dbg%21%28n%29%3B%0A++++%7D%0A%7D%0A)" -->

---

## Kombinatoren für Iteratoren: `collect`

`collect` erstellt eine Collection aus einem Iterator.
Hier ist häufig ein Turbofish oder eine Annotation nötig.

```rust
let values = vec!["birdie", "cat", "doggo"];
let result: HashSet<_> = values
    .iter()
    .map(|a| a.to_uppercase())
    .collect();
```

Die Typinferenz setzt hier den Freiheitsgrad von [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) fest.
Oft kann man so mit einem [`let`](https://doc.rust-lang.org/std/keyword.let.html) binding den Turbofish vermeiden.

---

## Kombinatoren für Iteratoren: `collect`

Manchmal braucht es einen Turbofish!

```rust
let line = input                    // String
    .split_whitespace()             // SplitWhitespace
    .map(parse_hex_digit)           // impl Iterator<Item = Result<u8, Error>>
    .collect::<Result<Vec<u8>>>()?; // Vec<u8>
```

Hier setzt der Turbofish den Freiheitsgrad fest.

Das Ergebnis ist ein `Result<Vec<u8>, Error>`, weil man auch ein `Result<T, E>` als Collection sehen kann.

<!-- _footer: "[Protohackers Netcrab](https://github.com/barafael/protohackers/blob/f1fe6cf0d6864661efd7d0014edc327ed523114d/netcrab/src/main.rs#L32)" -->

---

## Aside: Point-free style

````rust tag:playground-button playground-before:$"fn main() { let seq = "\n\t\r\\";"$ playground-after:$"}"$
seq.chars().map(char::escape_default).for_each(|elem| {
    print!("{elem}");
});
````

---

## Collecting und [`Result<Collection, E>`](https://doc.rust-lang.org/std/result/)

Gegeben ein [`Result<T, E>`](https://doc.rust-lang.org/std/result/), wie kann man [`?`](https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-) zur Fehlerbehandlung einsetzen?

<p style = "text-align: center;">
<code>Iterator<Item = Result<T, E>> -> Result<Vec<T>, E></code>
</p>

```rust
let handles = env
    .interfaces
    .iter()
    .map(|(interface, socket_addr)| {
        socket::open(interface, socket_addr).map(|socket| (socket, interface))
    })
    .collect::<Result<Vec<_>, _>>()?;
```

---

## Collecting und [`Result<Collection, E>`](https://doc.rust-lang.org/std/result/)

Der [`FromIterator<A>`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) Trait wird genutzt um [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) zu implementieren.

```rust
impl<A, E, V: FromIterator<A>> FromIterator<Result<A, E>> for Result<V, E>
```

Ganz schön kompliziert. Daumenregel: man kann [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) auf Iteratoren aufrufen, auch auf Iteratoren von `Result`.
Und man sollte den gewünschten Ausgabetyp festlegen.

---

## Collecting und [`try_join_all`](https://docs.rs/futures/latest/futures/future/fn.try_join_all.html)

Realistisches Beispiel mit Futures und Future-Kombinatoren:

```rust
// let handles: Vec<JoinHandle> = ...;
futures::future::try_join_all(handles)
    .await
    .context("Unable to join client tasks")?
    .into_iter() // IntoIter<Result<(), Error>>
    .collect::<Result<(), _>>() // Result<(), Error>
    .context("Error in client task")
```

Das ist ein sehr beliebtes Pattern.

<!-- _footer: "
[Achat: chat with cancellation](https://github.com/barafael/achat/blob/c8fa30d90b703b41993e04f53fe474070b0ab199/bin/chat_with_cancel.rs#L51)
" -->

---

## Weitere Kombinatoren

`map`, `filter`, `collect` sind nur der Anfang!

[Provided Methods vom Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#provided-methods)

[Provided Methods von String Slices (`&str`)](https://doc.rust-lang.org/std/primitive.str.html)

Besondere/beliebte Kombinatoren: `chain`, `zip`, `cycle`, `take`, `windows`, `fold`, ...

Viele weitere in [itertools](https://github.com/rust-itertools/itertools): `windows`, `interleave`, `collect_vec`, `join`, `partition`, `peek_nth`

---

## Einschub: Parallele Iteratoren

Die Iteratoren sind eh schon Threadsafe! Also: work stealing.

```rust
let mut pixels = img.enumerate_pixels_mut().collect::<Vec<_>>();

pixels.par_iter_mut().for_each(|(x, y, pixel)| {
    let steps = convergence_steps(
        ...
    );
    **pixel = colorgrad(steps, &colorgrad::turbo());
});
```

<!-- _footer: "[Julia Set Renderer mit Rayon](https://github.com/cocomundo/julia-set-renderer/blob/b88241ba482c0af1269a990ad3184d47179e7144/src/lib.rs#L42)" -->

---

## Iteratoren jenseits von Collections

Auch viele andere Funktionen geben einen Iterator zurück, selbst wenn es keine Collection dahinter gibt:

* `std::env::args()`: Argumente des Programmes
* `std::str::matches()`: Matches eines Patterns in einem String

Man kann die Schnittstelle auch sonst vielfältig nutzen, zum Beispiel um [Fibonacci-Sequenzen](https://doc.rust-lang.org/rust-by-example/trait/iter.html) zu erzeugen.

---

## Die HashMap Entry API

Die Entry API ist der idiomatische und ergonomische Weg, HashMap-Einträge zu bearbeiten.
Dabei werden die Einträge der Map in-place editiert!
Ein Entry:
```rust
pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
```

Mehr Details auf "[A Rust Gem - The Rust Map API](https://www.thecodedmessage.com/posts/rust-map-entry/)"

---

## Beispiel: `or_insert`

Oft will man auf existierenden Elementen operieren, oder falls es keine gibt, einen Anfangswert einfügen:

```rust
let mut counts: HashMap<&str, usize> = HashMap::new();
for name in ["a", "b", "c", "a", "a"] {
    *counts.entry(name).or_insert(0) += 1;
}
```


<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn+main%28%29+%7B%0A++++let+mut+counts%3A+HashMap%3C%26str%2C+usize%3E+%3D+HashMap%3A%3Anew%28%29%3B%0A++++for+name+in+%5B%22a%22%2C+%22b%22%2C+%22c%22%2C+%22a%22%2C+%22a%22%5D+%7B%0A++++++++*counts.entry%28name%29.or_insert%280%29+%2B%3D+1%3B%0A++++%7D%0A++++dbg%21%28counts%29%3B%0A%7D%0A)" -->

---

## Beispiel: `or_default`

```rust
let mut counts: HashMap<&str, usize> = HashMap::new();
for name in ["a", "b", "c", "a", "a"] {
    *counts.entry(name).or_default() += 1;
}
```

`or_default` gibt ein `&mut V`, also einen mutable borrow auf den Wert in der HashMap.

Wo kommt der default-Wert her? Natürlich vom `Default` Trait!

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn+main%28%29+%7B%0A++++let+mut+counts%3A+HashMap%3C%26str%2C+usize%3E+%3D+HashMap%3A%3Anew%28%29%3B%0A++++for+name+in+%5B%22a%22%2C+%22b%22%2C+%22c%22%2C+%22a%22%2C+%22a%22%5D+%7B%0A++++++++*counts.entry%28name%29.or_default%28%29+%2B%3D+1%3B%0A++++%7D%0A++++dbg%21%28counts%29%3B%0A%7D%0A)" -->

---

## Beispiel: `and_modify`

Viele der [Usages](https://github.com/search?q=%22and_modify%22+language%3ARust&type=code&ref=advsearch) lassen sich auch als Kombination von `or_insert(.)` oder `or_default`.

```rust
edges.entry(dv.init).and_modify(|e| *e += 1).or_insert(1);
```

Äquivalent:

```rust
*edges.entry(dv.init).or_default() += 1;
```

<!-- _footer: "[facebook/hhvm](https://github.com/facebook/hhvm/blob/c01bc30d5883ffdf08329111fa709ed9da815ad5/hphp/hack/src/hackc/ir/conversions/ir_to_bc/emitter.rs#L171)" -->

---

## Review

 - Collections
 - Iterators
 - Iterator Combinators
 - The HashMap Entry API
