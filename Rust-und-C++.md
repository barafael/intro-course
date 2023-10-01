---
title: Collections in Rust and C++
marp: true
theme: rhea
color: "dark-gray"

---

<!-- 
footer: "Collections in Rust und C++"
 -->

<!--
paginate: true
 -->

<!-- 
_footer: ''
_paginate: false
 -->

<!-- _class: lead -->

# Rust und C++
## Äquivalenzen und Unterschiede

---

<!-- header -->

# Disclaimer (Triggerwarnung?)

Wenn hier C++ und Rust verglichen werden, soll das ein fairer Vergleich von Technologien und ihren Werten sein.

Wenn Rust dabei ziemlich gut abschneidet, liegt das daran, dass ich mich damit besser auskenne und natürlich Probleme von der Rust Perspektive aus beurteile. Schließlich ist es auch eine Rust Schulung.

Zweifelsohne entwickelt sich C++ auf erfreuliche Art weiter und ist auch weiterhin eine sehr beliebte Technologie.

<!-- _footer: Empfehlung zum Thema "Werte" von Technologien: [Software as a Reflection of Values ](https://corecursive.com/024-software-as-a-reflection-of-values-with-bryan-cantrill/#) -->
---

# Ziele von C++

Laut [P2137R0: Goals and priorities for C++](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2020/p2137r0.html):

* Performance-critical software
* Both software and language evolution
* Code that is simple and easy to read, understand, and write
* Practical safety guarantees and testing mechanisms
* Fast and scalable development
* Current hardware architectures, OS platforms, and environments as they evolve

---

# Ziele von Rust (prä-2018)

[prev.rust-lang.org](https://www.prev.rust-lang.org/):

<style scoped>
img[alt~="center"] {
  display: block;
  margin: 0 auto;
  border: 5px solid;
  border-radius: 10px;
}
</style>

![w:800px center](assets/images/old-goals-of-rust.png)

---

# Ziele von Rust Aktuell

[rust-lang.org](https://www.rust-lang.org/):

<style scoped>
img[alt~="center"] {
  display: block;
  margin: 0 auto;
  border: 5px solid;
  border-radius: 10px;
}
</style>

![w:800px center](assets/images/goals-of-rust.png)

---

# C++ und Rust wollen dasselbe!

Rust hat nur den Vorteil, aus Jahrzehnten C++ und anderen Sprachen gelernt zu haben.

Und den Nachteil, eben keine 40 Jahre Reifung und Evolution und Ökosystem zu haben.

[The Coded Message: "Rust: A New Attempt at C++'s Main Goal"](https://www.thecodedmessage.com/posts/rust-new-cpp/)

<!-- _footer: "[The Coded Message](https://www.thecodedmessage.com/) beschreibt u.A., wie ein Entwickler von C++ auf Rust umsteigt" -->

---

# Ungefähre Äquivalenzen

* Collections und Datenstrukturen
  - `vector<T>` ≈ `Vec<T>`
  - `queue<T>` ≈ `VecDeque<T>`
  - `list<T>` ≈ `LinkedList<T>`
  - `set<T>` ≈ `BTreeSet<T>`
  - `unordered_set<T>` ≈ `HashSet<T>`
  - `map<Key, T>` ≈ `BTreeMap<K, V>`
  - `unordered_map<Key, T>` ≈ `HashMap<K, V>`

---

# Detail: Implementierung von Sets

Die Implementierungen der Mengen-Datentypen sind geschickt erledigt:

```rust
pub struct HashSet<T, A: Allocator> {
    pub(crate) map: HashMap<T, (), A>,
}
```

Das ist eine Zero-Cost Abstraction!

```rust
assert_eq!(std::mem::size_of::<()>(), 0);
```

<!-- _footer: "[See docs](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html)" -->

---

# BTree und Hash?

Alle Methoden der HashMap leben in einem `impl` Block mit Bounds:

```rust
impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
```

Bei der BTreeMap gibt es eher lokale Bounds an den Methoden:

```rust
pub fn insert(&mut self, key: K, value: V) -> Option<V>
where
    K: Ord,
```

---

# Ungefähre Äquivalenzen

* Eingebaute Datentypen und Exoten
  - `struct` ≈ `struct`
  - tagged union pattern ≈ `enum`
  - `pair<T1, T2>` ≈ `(T, U)`, `(T₁, ..., Tₙ)`
  - `array<T, N>` ≈ `[T; N]`
  - `make_heap` ≈ `BinaryHeap`

---

# Ungefähre Äquivalenzen

* Smart Pointer
  - `unique_ptr` ≈ `Box<T>`
  - `shared_ptr` ≈ `Arc<T>` und `Rc<T>`

* Referenzen
  - `T&` `const T&` ≈≈≈ `&T` und `&mut T`

* Pointer
  - `int (const) * (const)` ≈ `*const T` und `*mut T`

---

# C++ Referenzen und Rust Borrows

Gemeinsamkeiten Referenz/Borrow:
 * Niemals Null
 * Automatisch dereferenziert (`(*b).member`)

Gemeinsamkeiten Pointer/Borrow:
 * Memory Layout
 * Neue Zuweisung erlaubt
 * Mutation erfordert Dereferenzierung (`*`)

---

# C++ Referenzen und Rust Borrows

Alleinstellungsmerkmale von Borrows
* Alle Ownership und Borrowing Regeln
* "Fat Pointer", doppelte Größe:
  - Slices: Pointer + Länge
  - Trait Objects: Pointer + Pointer auf VTable

# ["it's a bit of both, and then some"](https://users.rust-lang.org/t/is-ref-in-rust-more-similar-to-c-pointer-rather-than-c-referene/14167/2?u=medium-endian)

<!-- _footer: "See [Rust ♡ Existing C++: 7. Pointers, references, values](https://google.github.io/autocxx/index.html)" -->
