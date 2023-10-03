---
title: Unit Testing
marp: true
theme: rhea
color: "dark-gray"
size: 16:9

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

# Unit Testing

<br>

### There isn't much to it

![bg](images/intro.png)

---

![bg right:40%](images/microscope.webp)

- Writing unit tests
- Separating code and tests
- Integration tests
- Property-Based Testing
- Unit tests of `async` code
- Pausing Time

---

<!-- header: ' ' -->

## "Business Logic"

In `lib.rs`:

````rust marker:identity_fn

````

It's also known as [`std::convert::identity`](rust:std::convert::identity).

---

## A test module

Use conditional compilation to separate code and tests:

````rust marker:test_module

````

The super module is made visible here.

---

## A simple test

````rust marker:test_should_map_integers

````

---

## `#[should_panic]`

Some tests need to panic:

````rust marker:test_should_panic

````

---

## `#[should_panic]`

Some tests need to panic:

````rust
#[test]
#[should_panic(expected = "Empty Selection")]
fn test_new_empty() {
    let _ = Selection::new(smallvec![], 0);
}
````

In this case, the test should be called `refuses_empty_selection`.

<!-- _footer: '[`test_new_empty` in Helix](https://github.com/helix-editor/helix/blob/75c0a5ceb32d8a503915a93ccc1b64c8ad1cba8b/helix-core/src/selection.rs#L810-L814)' -->

---

## Doc Tests

````rust marker:simple_doctest

````

---

## Panicking in doctest

````rust
/// ```should_panic
/// use std::process::Command;
///
/// let mut child = Command::new("/bin/cat")
///                         .arg("file.txt")
///                         .spawn()
///                         .expect("failed to execute child");
````

[Generated docs](https://doc.rust-lang.org/std/process/struct.Command.html#examples-12)

[Doc Test in std sources](https://doc.rust-lang.org/src/std/process.rs.html#153-165)

---

## `#[ignore]`

To silence a test whether it fails or not:

````rust
#[test]
#[ignore]
fn flaky_flake() {
    ...
}
````

---

## Serial tests

Tests run in parallel by default, in randomized order.

This can be problematic if you need them to access some global state (like android properties).

````rust
#[test]
#[serial]
fn should_write_then_read_state() {
    ...
}
````

This uses the crate [`serial_test`](docsrs:https://docs.rs/serial_test/2.0.0/serial_test/).

---

## Different rules for tests

- Tests should not start with `test_`
- Tests should start with `should_` or `is_`
- Test names should be as long as needed (but not longer)
  - `should_trigger_a_deletion_after_receiving_a_timeout_msg`
- `unwrap` is allowed in tests!

---

## Integration Tests

Integration tests are simply tests in the `tests/` directory.
They are full crates themselves, and as such only have access to the public API of our crate under test.

````
the_crate
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
````

---

## Proptest

Given properties about a system ("invariants"), property-based testing exercises the system to find a case in which the property is violated. It then minimizes the input to still find the case.

````rust marker:dev_dependencies_proptest

````

---

## Proptest

Property: For any `i32` from 0 to 1000, the identity function returns that same `i32`.

````rust marker:simple_proptest

````

---

## Proptest Strategies: any

Above example had a limited range.

````rust marker:any_strategy_proptest

````

---

## Proptest Strategies: vec

A vector can also be generated instead, greatly increasing the range of input values:

````rust marker:vec_strategy_proptest

````

---

## Further strategies

For the use case of testing Codecs, proptest a custom [`Strategy`](docsrs:https://docs.rs/proptest/latest/proptest/strategy/index.html) can be composed for many types.

It can also be derived in some cases, using [`proptest-derive`](https://proptest-rs.github.io/proptest/proptest-derive/index.html).

Frequent usecase: Given a random protocol data unit from a protocol implementation, encode it to bytes, decode it again, assert equality.

---

## Testing [`async`](keyword:async) Code

Async code requires a runtime to be started.

````rust tag:playground-button playground-before:$"use std::time::Duration; use tokio::{sync::mpsc, time::timeout};"$ playground-after:$""$
const TIMEOUT: Duration = Duration::from_millis(500);

#[tokio::test]
async fn should_time_out() {
    let (tx, mut rx) = mpsc::channel::<String>(16);
    assert!(timeout(TIMEOUT, rx.recv()).await.is_err())
}
````

---

## Testing [`async`](keyword:async) Code

Don't operate on Sockets or TcpStreams directly - require the [`tokio::io::AsyncRead`](docsrs:https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html) and/or [`tokio::io::AsyncWrite`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncWrite.html):

````rust
pub async fn handle_connection<Reader, Writer>(
    addr: SocketAddr,
    reader: Reader,
    mut writer: Writer,
    tx: broadcast::Sender<(String, SocketAddr)>,
    mut rx: broadcast::Receiver<(String, SocketAddr)>,
) -> anyhow::Result<()>
where
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
````

---

## Testing [`async`](keyword:async) Code

This allows to use the [`tokio_test::io::Builder`](docsrs:https://docs.rs/tokio-test/0.4.3/tokio_test/io/struct.Mock.html) as your Mock.

````rust
use tokio_test::io::Builder as Mock;
let writer = Mock::new()
    .write(b"Announcement: hello\n")
    .write(b"Announcement: i'm\n")
    .write(b"Announcement: a\n")
    .write(b"Announcement: teapot\n")
    .build();
let reader = Mock::new().wait(Duration::from_secs(1)).build();
````

---

## Testing [`async`](keyword:async) Code

Now, you can create the normal connection with the mock.

````rust
let handle = tokio::spawn(handle_connection(
    "127.0.0.3:8081".parse().unwrap(),
    reader,
    writer,
    tx.clone(),
    tx.subscribe(),
    topic_rx,
));
````

<!-- _footer: [barafael/achat](https://github.com/barafael/achat)' -->

---

## Traits and Mocks

Traits and honest low-level interfaces are the de-facto way of mocking in Rust.

Iterators, Streams, Read/Write, AsyncRead/AsyncWrite are such low-level interfaces.

Plenty of crates, though: [https://crates.io/search?q=mock](https://crates.io/search?q=mock), but nothing seriously advanced.

---

## Stop Time

Some test cases would take a long time in realtime. But `tokio` can travel faster than light!

The tokio timer wheel can be paused and resumed.

Whenever the runtime is only waiting on timers, it fast-forwards to the next one.

---

## Stop Time

It's not magic. This test should run within milliseconds.

````rust
#[tokio::test(start_paused = true)]
async fn should_expire_after_one_second() {
    let watchdog = Watchdog::with_timeout(Duration::from_secs(1));
    let (reset_tx, expired_rx) = watchdog.run();
    let now = Instant::now();
    expired_rx.await.unwrap();
    assert_elapsed!(now, Duration::from_secs(1));
    assert!(reset_tx.send(Signal::Reset).await.is_err());
}
````

<!-- _footer: '[barafael/watchdog](https://github.com/barafael/watchdog/blob/main/src/lib.rs)' -->

---

## Review

- Simple tests
- Special attributes for tests
- Doc tests
- Integration tests
- Proptest
- Async tests with mocks and time travel

![bg right](images/magnifying_glass.webp)

---

## Questions?

<jframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>
