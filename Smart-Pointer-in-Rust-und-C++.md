---
title: Smart Pointer in Rust und C++
marp: true
theme: rhea
color: "dark-gray"

---

<!-- 
footer: "Smart Pointer in Rust und C++"
 -->

<!--
paginate: true
 -->

<!-- 
_footer: ''
_paginate: false
 -->

<!-- _class: lead -->

# Smart Pointer in Rust und C++

---

### Was bedeutet "Smart Pointer" in Rust?

### Warum überhaupt Smart Pointer, wenn es doch `&T` und `&mut T` gibt?

### Welche Smart Pointer gibt es in Rust, und welche C++-Analoge kann man dazu finden?

### (Warum) genügen `unique_ptr` und `shared_ptr` nicht?

---

<!-- header: ' ' -->

# Smart Pointer in Rust

Ein Smart Pointer ist ein Typ, welcher einen normalen Pointer mit zusätzlichen Eigenschaften und Invarianten ausstattet. Ein Smart Pointer übernimmt _Verantwortung_ (=Ownership) für eine Ressource.

Normale Referenzen (`&` und `&mut`) besitzen keinen Speicher, sie borgen ihn nur von einem `T`!

Allgemein sind Smart Pointer seltener als normale Borrows, aber dennoch beim Multithreading etc. gerne verwendet.

---

# Häufige Eigenschaften (Traits)

Alle Smart Pointer haben spezialisiertes Verhalten für:

* `std::ops::Deref`: Anpassung Dereferenz-Operators (`*p`).

* `std::ops::Drop`: Anpassung des Deallokationsverhaltens.

Oft kommt hinzu:

* `std::clone::Clone`: Anpassung des Kopiervorgangs. Auch Abwesenheit davon ist eine Aussage.

<!-- _footer: "Für noch mehr Verwirrung mit Traits, siehe [Rust Concept Clarification: Deref vs AsRef vs Borrow vs Cow](https://dev.to/zhanghandong/rust-concept-clarification-deref-vs-asref-vs-borrow-vs-cow-13g6)" -->

---

# Einfache Smart Pointer

* `Box<T>`: Heap-allozierter einfacher Wert.
    - `Deref`: Borrow vom inneren Wert
    - `Drop`: Deallokation
* `Rc<T>`, `Arc<T>`: Simuliert "Shared Ownership" für Graph-Strukturen
    - `Deref`: Borrow vom inneren Wert
    - `Clone`: Inkrementierung des Zählers
    - `Drop`: Falls Zähler 0, Deallokation

---

# Warum `Deref`?

Über `Deref` wird definiert, was bei einer Dereferenzierung geschieht. Das erlaubt, viele verschiedene Typen einfach als Borrow zu behandeln.

```rust
impl Deref for String {
    type Target = str;
    fn deref(&self) -> &str { ... }
}
```

"Deref Coercion" wird solange angewendet, bis der Typ passt - daher gibt es kein `->`.

<!-- _footer: "\"The Rust Programming Language\": [Implicit Deref Coercions with Functions and Methods](https://doc.rust-lang.org/book/ch15-02-deref.html)" -->

---

# Einschub: Slices über Deref

In Signaturen stehen oft nur Referenzen oder Slices - `Deref` regelt:

```rust
let vec = vec![1, 2, 3];
takes_slice(&vec); // Treat `&Vec<T>` as `&[T]`

fn takes_slice<T: std::fmt::Debug>(slice: &[T]) {
    for elem in slice {
        dbg!(elem);
    }
}
```

Korrespondierende [Implementierung in `std`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-Deref-for-Vec%3CT%2C%20A%3E)

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%0A++++let+vec+%3D+vec%21%5B1%2C+2%2C+3%5D%3B%0A++++%0A++++takes_slice%28%26vec%29%3B%0A%7D%0A%0Afn+takes_slice%3CT%3A+std%3A%3Afmt%3A%3ADebug%3E%28slice%3A+%26%5BT%5D%29+%7B%0A++++for+elem+in+slice+%7B%0A++++++++dbg%21%28elem%29%3B%0A++++%7D%0A%7D%0A)"-->
---

# Warum `std::ops::Drop`?

Über den `Drop` Trait wird Deallokationsverhalten angepasst:

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

Damit erhalten Smart Pointer, Collections, Sockets, Files, Locks und so weiter die Möglichkeit, sauber aufzuräumen.

Die Liste der Implementoren ist [nicht kurz](https://doc.rust-lang.org/std/ops/trait.Drop.html#implementors), aber oft ist eine Implementierung unnötig, da sowieso implizit aufgeräumt wird.


---

# Einschub: Was ist [`core::mem::drop`](https://doc.rust-lang.org/stable/core/mem/fn.drop.html)?

Vielleicht die schönste Funktion in [`std`](https://doc.rust-lang.org/std/index.html):

```rust
#[inline]
pub fn drop<T>(_x: T) {}
```

* Nimmt ein `T`, gibt nichts zurück

* `T` ist beliebig (nicht mal [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html), oder [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) erforderlich)

* Was kann die Funktion anhand ihrer Signatur tun?

---

<!-- _class: lead -->

# Direkter Vergleich: <br> Die meisten Smart Pointer ähneln denen von C++ (oberflächlich)

---

# `unique_ptr<T>` vs. `Box<T>`

Beide Typen sind Smart Pointer, die ihren Inhalt besitzen und am Ende ihrer Lebenszeit aufräumen.

Unterschiede:
* `unique_ptr` kann einen `nullptr` enthalten
* `unique_ptr` erlaubt Pointer auf Inhalt, welche Inhalt überleben (dangling pointer)

---

# `unique_ptr<T>` vs. `Box<T>`

Ein paar Spitzfindigkeiten:

* Borrowing und Ownership gelten weiterhin -> man kann einen `Box<T>` safe zu einem `&T` oder `&mut T` degradieren
* Ein `unique_ptr<T>` kann nicht in einem Register übergeben werden (es ist [kompliziert](https://stackoverflow.com/questions/58339165/why-can-a-t-be-passed-in-register-but-a-unique-ptrt-cannot)). Ein `Box<T>` ist immer so groß wie ein Pointer und darf, da Rust keine fixe ABI hat, auch in einem Register übergeben werden.

Verwendung von `Box<T>`: Selten! Referenzen genügen meist.

---

# Move Semantik `unique_ptr`/`Box`


```c++
#include <iostream>
#include <memory>

int main() {
    std::unique_ptr<int> fifteen(new int(15));
    std::unique_ptr<int> moved(std::move(fifteen));

    std::cout << "moved = " << *moved << '\n';
    std::cout << "fifteen = " << *fifteen << '\n';
}
```

```
shell: Job 1, './a.out' terminated by signal SIGSEGV (Address boundary error)
```

---

# Move Semantik `unique_ptr`/`Box`

Direkte Übersetzung in Rust:

```rust
fn main() {
    let fifteen = Box::new(15);
    let moved = fifteen;

    println!("{moved}");
    println!("{fifteen}");
}
```


<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%0A++++let+fifteen+%3D+Box%3A%3Anew%2815%29%3B%0A++++let+moved+%3D+fifteen%3B%0A%0A++++println%21%28%22%7Bmoved%7D%22%29%3B%0A++++println%21%28%22%7Bfifteen%7D%22%29%3B%0A%7D%0A)" -->

---

# Der Compiler hat eine Meinung dazu

```
error[E0382]: borrow of moved value: `fifteen`
 --> src/main.rs:6:16
  |
2 |     let fifteen = Box::new(15);
  |         ------- move occurs because `fifteen` has type `Box<i32>`, which does not implement the `Copy` trait
3 |     let moved = fifteen;
  |                 ------- value moved here
...
6 |     println!("{fifteen}");
  |                ^^^^^^^ value borrowed here after move
  |
[...]
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let moved = fifteen.clone();
  |                        ++++++++
```

Cloning ist hier natürlich ein pragmatischer Workaround.

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%0A++++let+fifteen+%3D+Box%3A%3Anew%2815%29%3B%0A++++let+moved+%3D+fifteen%3B%0A%0A++++println%21%28%22%7Bmoved%7D%22%29%3B%0A++++println%21%28%22%7Bfifteen%7D%22%29%3B%0A%7D%0A)" -->

---

# Beispiel: Dangling `Box`?

Die _Lifetime_ einer Referenz in eine Box ist kleiner als die der Box.

```rust
fn get_dangling<'a>() -> &'a u32 {
    let on_heap = Box::new(5);
    &on_heap
//  ^^^^^^^^ returns a reference to data owned by the current function
}
```
```
error[E0515]: cannot return reference to local variable `on_heap`
```

* Führen Sie im Terminal aus: `rustc --explain E0515`

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+get_dangling%3C%27a%3E%28%29+-%3E+%26%27a+u32+%7B%0A++++let+on_heap+%3D+Box%3A%3Anew%285%29%3B%0A++++%26on_heap%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+five+%3D+get_dangling%28%29%3B%0A++++dbg%21%28five%29%3B%0A%7D)" -->
---

# Beispiel: A leaky `Box`

Memory leaks sind nach der Definition von Rust nicht `unsafe`:

```rust
fn get_dangling<'a>() -> &'a u32 {
    let on_heap = Box::new(5);
    let leaked = Box::leak(on_heap);
    leaked
}
```

* Führen Sie das Programm im verlinkten Playground aus.
* Führen Sie das Programm mithilfe des Miri interpreters im Playground aus.

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+get_dangling%3C%27a%3E%28%29+-%3E+%26%27a+u32+%7B%0A++++let+on_heap+%3D+Box%3A%3Anew%285%29%3B%0A++++let+leaked%3A+%26mut+u32+%3D+Box%3A%3Aleak%28on_heap%29%3B%0A++++leaked%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+five+%3D+get_dangling%28%29%3B%0A++++dbg%21%28five%29%3B%0A%7D)" -->
---

# Einschub: Detektion von UB mit Miri

Miri **interpretiert** Rust Code auf einer virtuellen Maschine und prüft dabei auf undefiniertes Verhalten.

```rust
     Running `/playground/...`
[src/main.rs:9] five = 5
The following memory was leaked: alloc1536 (Rust heap, size: 4, align: 4) {
    05 00 00 00                                     │ ....
}

error: the evaluated program leaked memory
```

<!-- _footer: "[Miri auf GitHub](https://github.com/rust-lang/miri)" -->

---

# Beispiel: Size of `Option<Box<T>>`

```rust
fn maybe_box() -> Option<Box<u32>> {
    Some(Box::new(3))
}

fn main() {
    let three = maybe_box();
    println!("{}", std::mem::size_of_val(&three));
}
```

Welchen Output vermuten Sie?
Wie erklären Sie sich den Output des Programms?

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+maybe_box%28%29+-%3E+Option%3CBox%3Cu32%3E%3E+%7B%0A++++Some%28Box%3A%3Anew%283%29%29%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+three+%3D+maybe_box%28%29%3B%0A++++println%21%28%22%7B%7D%22%2C+std%3A%3Amem%3A%3Asize_of_val%28%26three%29%29%3B%0A%7D)" -->

---

# Einschub: Turbofish Syntax

Einfacheres Programm für die identische Beobachtung:

```rust
assert_eq!(mem::size_of::<Option<Box<u32>>>(), mem::size_of::<Box<u32>>());
```

Hier wird der Turbofish-Syntax genutzt, um statisch den Typ des generischen Argumentes von `std::mem::size_of` festzulegen.

```rust
pub const fn size_of<T>() -> usize {
    ...
}
```

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Amem%3B%0A%0Afn+main%28%29+%7B%0A++++assert_eq%21%28mem%3A%3Asize_of%3A%3A%3COption%3CBox%3Cu32%3E%3E%3E%28%29%2C+mem%3A%3Asize_of%3A%3A%3CBox%3Cu32%3E%3E%28%29%29%3B%0A%7D)" -->

---

# (Anti-)Beispiel: Singly Linked List

<style scoped>
table, tr, td, th {
  all: unset;
  border: 0 !important;
  background: transparent !important;
}
table { display: table; }
tr { display: table-row; }
td, th { display: table-cell; }

table {
  width: 100%;
}
td {
  vertical-align: middle;
  width: 10%;
  padding: 0 15px;
}
</style>
<table>
<td>

```rust
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
```
</td>
<td>
Beispiel aus:

<a href="https://rust-unofficial.github.io/too-many-lists/index.html">Learning Rust With Entirely Too Many Linked Lists</a>

Indirektion notwending, da sonst `Link` Typ unbestimmbar groß.
</td>
</tr>
</table>

<!-- _footer: "[Learning Rust With Entirely Too Many Linked Lists -> Basic Data Layout](https://rust-unofficial.github.io/too-many-lists/first-layout.html#basic-data-layout)" -->

---

# `shared_ptr`: Shared Responsibility

Reference Counted Pointers: `Rc<T>` und `Arc<T>`.

Ähnlich zum `shared_ptr` inkrementieren sie beim `.clone()` einen Reference Count und dekrementieren ihn beim `.drop()` (RAII/SBRM).

`Rc<T>` ist nicht Thread Safe, daher billiger!

Daumenregel: "`Rc<T>` als Default, falls Compiler jammert, `Arc<T>`, falls immer noch, `Arc<Mutex<T>>`", aber das ist kein Freifahrtsschein!

<!-- SBRM: Scope-Bound Resource Management -->

---

# Multithreading mit `Arc<T>`

Ein `Arc<T>` implementiert genau dann `Send` und `Sync` (≈ "Thread Safe"), wenn seine enthaltenen Daten `Send` und `Sync` sind.

--> Sichere Shared Ownership aus verschiedenen Threads!

```
`Arc<T>` will implement [`Send`] and [`Sync`] as long as the `T` implements
[`Send`] and [`Sync`]. Why can't you put a non-thread-safe type `T` in an
`Arc<T>` to make it thread-safe? [...]
The key is this: Arc<T> makes it thread safe to have multiple ownership of the same data,
but it doesn’t add thread safety to its data.
```

<!-- _footer: "[std::sync::Arc docs über Thread Safety](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety)" -->

---

# Shared Ownership mit `Arc<T>`

```rust
let remote = Arc::new("there is only one TV remote");

let mut handles = Vec::new();
for _ in 0..10 {
    let remote = Arc::clone(&remote);

    let handle = thread::spawn(move || {
        sleep(Duration::from_millis(thread_rng().gen_range(0..7000)));
        println!("{:?}", remote);
    });
    handles.push(handle);
}
handles.into_iter().for_each(|h| h.join().unwrap());
```

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+rand%3A%3A%7Bthread_rng%2C+Rng%7D%3B%0Ause+std%3A%3A%7B%0A++++sync%3A%3AArc%2C%0A++++thread%3A%3A%7Bself%2C+sleep%7D%2C%0A++++time%3A%3ADuration%2C%0A%7D%3B%0A%0Afn+main%28%29+%7B%0A++++let+remote+%3D+Arc%3A%3Anew%28%22there+is+only+one+TV+remote%22%29%3B%0A%0A++++let+mut+handles+%3D+Vec%3A%3Anew%28%29%3B%0A++++for+_+in+0..10+%7B%0A++++++++let+remote+%3D+Arc%3A%3Aclone%28%26remote%29%3B%0A%0A++++++++let+handle+%3D+thread%3A%3Aspawn%28move+%7C%7C+%7B%0A++++++++++++sleep%28Duration%3A%3Afrom_millis%28thread_rng%28%29.gen_range%280..7000%29%29%29%3B%0A++++++++++++println%21%28%22%7B%3A%3F%7D%22%2C+remote%29%3B%0A++++++++%7D%29%3B%0A++++++++handles.push%28handle%29%3B%0A++++%7D%0A++++handles.into_iter%28%29.for_each%28%7Ch%7C+h.join%28%29.unwrap%28%29%29%3B%0A%7D%0A) und [ähnliches Beispiel aus 'Rust By Example'](https://doc.rust-lang.org/rust-by-example/std/arc.html)"-->

---



# Beispiel: Shared Mutability

```rust
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

type Registry<K, V> = Arc<Mutex<HashMap<K, V>>>;
// type Registry<K, V> = Arc<RwLock<HashMap<K, V>>>;

fn main() {
    let registry = Registry::default();
    registry.lock().unwrap().entry("A").or_insert(6);
    let _ = dbg!(registry.lock().unwrap());
}
```

<!-- _footer: "[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashMap%3B%0Ause+std%3A%3Async%3A%3A%7BMutex%2C+Arc%7D%3B%0A%0Atype+Registry%3CK%2C+V%3E+%3D+Arc%3CMutex%3CHashMap%3CK%2C+V%3E%3E%3E%3B%0A%0Afn+main%28%29+%7B%0A++++let+registry+%3D+Registry%3A%3Adefault%28%29%3B%0A++++registry.lock%28%29.unwrap%28%29.entry%28%22A%22%29.or_insert%286%29%3B%0A++++let+_+%3D+dbg%21%28registry.lock%28%29.unwrap%28%29%29%3B%0A%7D)" -->

---

# `Arc<Mutex<T>>` Arbeitsteilung

Der `Arc<...>` verwaltet die geteilte Verantwortung zwischen Threads für den enthaltenen Wert, und der letzte Thread hat dann die Aufräumarbeiten.

Der `Mutex<T>` verwaltet das geteilte Recht auf Zugriff auf die Ressource.
Ein `RwLock<T>` ist ähnlich, aber noch granularer: es setzt "Sharing XOR Mutation" zur Laufzeit um, d.h. es dürfen viele Lesen.

Rechte (Zugriff) und Pflichten (Aufräumen)!

---

# `shared_ptr` und `Rc`/`Arc` im Vergleich

Der C++ `shared_ptr` hat immer einen Thread Safe Zähler (atomic),
aber die enthaltenen Daten müssen nicht Thread Safe sein
(siehe [“Curiously Recurring C++ Bugs at Facebook”](https://www.youtube.com/watch?v=lkgszkPnV8g&t=1267s)).

D.h.: `shared_ptr` genügt nicht für Thread safety, aber jede Verwendung des `shared_ptr` (auch single Threaded) kostet.

In Rust forciert der Compiler die Verwendung von `Arc`, falls nötig.
Bei Verwendung von `Arc` wo `Rc` genügt jammert aber niemand :)

---

# `Arc<Mutex<T>>` im Kontext von `async`

Häufiges Pattern: asynchrone Tasks, Futures, oder Threads teilen Zugriff auf einen `Arc<Mutex<T>>` o.Ä.

Das ist in Ordnung, aber:

### <p style="text-align:center;">"Don't communicate by sharing memory, share memory by communicating"</p>
<p style="text-align:right;">- Rob Pike</p>

---

# Einschub: Channels/Message Passing

In "[Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/)" beschreibt Alice Ryhl eine Alternative zu `Arc<Mutex<...>>`: das Actor-Pattern.

```rust
enum ActorMessage {
    GetUniqueId(oneshot::Sender<u32>)
}

async fn run(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}
```

---

# Beispiel: `String` als Smart Pointer

Ein String ist ein `Vec<u8>`, dessen Inhalt immer UTF-8 encodiert ist. Er **besitzt** seine Bytes, auch wenn man aus einem `String` eine `&str` Slice gewinnen kann.

Ein String bietet spezialisierte Operationen auf seinem Inhalt an, welche seine Invarianten erhalten:
* `insert`, `into_bytes`, `push_str`, ...
* Iteration über Unicode Character
* u.v.m.

---

# "Collections sind Smart Pointer"

* Sie verwalten ihren eigenen Speicher
* Sie implementieren `Deref`, `Drop`, und oft `Clone`
* Sie bieten spezialisierte Operationen auf ihrem Inhalt an

[Collections are smart pointers](https://rust-unofficial.github.io/patterns/idioms/deref.html) aus dem Buch "Rust Design Patterns"

---

# Rückschau

* Smart Pointer
* Drop und Deref
* C++ und Rust Vergleich
* Niche Optimization
* Actor Pattern
