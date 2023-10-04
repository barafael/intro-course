---
title: Cargo and Crates.io
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

# Cargo and Crates.io

![bg blur:1px brightness:1.1 contrast:1.3](images/intro.png)

---

## Overview

![bg blur:6px left:33%](images/gears.png)

Get to know Cargo and the ecosystem

---

<!-- header: ' ' -->

## Rustc and Cargo

`rustc` has a CLI, which almost nobody uses. Cargo is the way!

`cargo new --bin crate_name` or `cargo new --lib lib_name`

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

![width:200px](images/9_cargo_and_crates_io/binary-crate.png)

</td>
<td>

![width:600px](images/9_cargo_and_crates_io/library-crate.png)

</td>
</tr>
</table>

---

## Cargo.toml

```toml
[package]
name = "achat"
version = "0.1.0"
edition = "2021"
description = "Super simple tokio chat server for educational purposes."
repository = "https://github.com/barafael/achat"

[dependencies]
anyhow = "1.0.68"
tokio = { version = "1.24.1", features = [
    "io-util",
    "macros",
] }
```

---

## Cargo.toml

Dependencies via path or git repo:

```toml
[dependencies]
klask = { path = "../klask", features = ["json"], default-features = true }
transmogrifier = { git = "https://github.com/barafael/pils" }
```

---

## Cargo.toml

```toml
[[bin]]
name = "chat"
path = "bin/chat.rs"

[dev-dependencies]
tokio-test = "0.4.2"

[build-dependencies]
phf = { version = "0.11.1", default-features = false }
phf_codegen = "0.11.1"
```

---

## Cargo.toml

```toml
[features]

default = [
  "std",
  "backend-winit",
  "backend-qt",
  "compat-1-0",
]

std = ["i-slint-core/std"]
libm = ["i-slint-core/libm"]
log = ["dep:log"]
```

<!-- _footer: "[Slint Project Cargo TOML](https://github.com/slint-ui/slint/blob/c11415dbbbbd93350ba1899114fe8517c8a16641/api/rs/slint/Cargo.toml#L20)" -->

---

## Semantic Versioning

The individual fields of MAJOR.MINOR.PATCH have strictly defined meanings:

<style scoped>
p {
  font-size: 25px;
}
</style>

> Given a version number MAJOR.MINOR.PATCH, increment the:
> MAJOR version when you make incompatible API changes
> MINOR version when you add functionality in a backward compatible manner
> PATCH version when you make backward compatible bug fixes

This applies primarily to lib crates.

<!-- _footer: "Aus [semver.org/lang/de/](https://semver.org/lang/de/)" -->

---

## Semantic Versioning

Additionally, some crates append pre-release and build info:

```toml
embedded-hal = { version = "1.0.0-alpha.10" }
```

---

## Cargo.toml and Cargo.lock

```toml
[dev-dependencies]
tokio-test = "0.4.2"

[dependencies]
anyhow = "1.0.68"
tokio = { version = "1.24.1", features = [
    "io-util",
    "macros",
] }
```

---

## Cargo.toml and Cargo.lock

```toml
[dev-dependencies]
tokio-test = "*"

[dependencies]
anyhow = "1"
tokio = { version = "1", features = [
    "io-util",
    "macros",
] }
```

---

```toml
[[package]]
name = "anyhow"
version = "1.0.68"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2cb2f989d18dd141ab8ae82f64d1a8cdd37e0840f73a406896cf5e99502fab61"

[[package]]
name = "andrew"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8c4afb09dd642feec8408e33f92f3ffc4052946f6b20f32fb99c1f58cd4fa7cf"
dependencies = [
 "bitflags",
 ...
]
```

---

## Why not just be explicit about versions?

* A library can declare a dependency on a range of versions of another library. This leaves freedom for the crate version resolver.
* Duplicate dependencies can be reduced if they stem from a compatible range.
* Compatible types can be used from slightly differing but compatible crate versions.

Cargo.lock is usually not edited manually.
`cargo update` updates the specific chosen versions.

---

## Transitive Dependencies

The same dependency in different subtrees at differing revision is not a problem.

But, a crate can publicly export types in its public interface.
Of course, then there may not be any inconsistencies.

---

## Transitive Dependencies

Transitive deps go fat quickly:

![width:1000px height:250px center](images/9_cargo_and_crates_io/transitive-deps.png)

<!-- _footer: "[cargo depgraph](https://github.com/jplatte/cargo-depgraph) auf [github.com/barafael/achat](https://github.com/barafael/achat)" -->

---

## Cargo.toml and Cargo.lock

From "Cargo.toml vs. Cargo.lock" in the Cargo Book:

“If you’re building a non-end product, such as a rust library that other rust packages will depend on, put Cargo.lock in your .gitignore.
If you’re building an end product, which are executable like command-line tool or an application, [...] check Cargo.lock into git.”

<!-- _footer: '[Chapter in the Cargo Book](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)' -->
---

## Change in advisory, though

https://github.com/rust-lang/cargo/issues/8728

---

## Crates.io

Central repo for all Rust crates. Authenticated maintainers can:
* Publish new crates
* Publish new versions of their crates
* "Yank" old versions of crates (doesn't remove a version, but prevents it from being added newly)

---

## Crates.io

![width:600px center](images/9_cargo_and_crates_io/crates.io.png)

---

## Crates.io - Namespace Pollution

![width:1000px center](images/9_cargo_and_crates_io/namespace-pollution-crates.io.png)

---

## Crates.io - Namespace Occupation

![width:1000px center](images/9_cargo_and_crates_io/namespace-occupation-crates.io.png)

---

## Crates.io - Name Squatting

Ah the irony!

This crate called "Crater" was occupied (has no content since 2 years).
The only downloads happen through the crater project...

![bg right height:700px](images/9_cargo_and_crates_io/name-hogging.png)

<!-- _footer: "[https://crates.io/crates/crater](https://crates.io/crates/crater)" -->

---

## Crates.io: Reverse Name Squatting

Clever move: to ensure no typosquatting, this crate was hogged:

![width:1000px center](images/9_cargo_and_crates_io/reverse-hogging.png)

---

## Vulnerability Index

RustSec collects CVEs. Example:

__RUSTSEC-2020-0071__
Potential segfault in the time crate:
https://rustsec.org/advisories/RUSTSEC-2020-0071.html

__RUSTSEC-2022-0081__
json is unmaintained:
https://rustsec.org/advisories/RUSTSEC-2022-0081.html

---

## RustSec and [`cargo audit`](https://crates.io/crates/cargo-audit)

Crates.io doesn't make any assertions about CVEs, but:

![w:650px center](images/9_cargo_and_crates_io/rustsec.png)

---

## RustSec and [`cargo audit`](https://crates.io/crates/cargo-audit)

Crates.io doesn't make any assertions about CVEs, but:

![w:650px center](images/9_cargo_and_crates_io/tokio-rustsec.png)

---

## RustSec and [`cargo audit`](https://crates.io/crates/cargo-audit)

Lists known CVEs in all your dependencies and recommends mitigations.

![w:850px center](images/9_cargo_and_crates_io/cargo-audit.png)

---

### Cargo Audit (RustSec Advisories in Cargo.lock)

```
> cargo audit
   Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
     Loaded 486 security advisories (from /home/rafaelbachmann/.cargo/advisory-db)
   Updating crates.io index
   Scanning Cargo.lock for vulnerabilities (59 crate dependencies)
Crate:    alloc-cortex-m
Version:  0.4.4
Warning:  unmaintained
Title:    crate has been renamed to `embedded-alloc`
Date:     2022-12-21
ID:       RUSTSEC-2022-0073
URL:      https://rustsec.org/advisories/RUSTSEC-2022-0073
Dependency tree:
alloc-cortex-m 0.4.4
└── wasmi-m4 0.1.0
```

<!-- _footer: "[rustsec.org](https://rustsec.org)" -->

---

## Review

- Normal and Special features of cargo
- SemVer
- Why Cargo.lock?
- Crates.io (good and bad)
- RustSec

![bg right](images/crate.webp)