---

title: "Actors Top To Bottom"
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

# Actors Top to Bottom

## Ist Rust jetzt Objektorientiert oder nicht?!

![bg](images/intro.png)

---

<!-- header: ' '-->

## `$whoami`

![bg right](images/colorkit%20(7).png)

- Lerne Rust seit 2016
- Embedded (C, C++, Rust)
- Automotive (Rust)
- Distributed Systems (Rust)
- Seit 2020: Rust Meetup Nuremberg
- Opensource auf GitHub: [github.com/barafael](https://github.com/barafael)

---

## Ownership ist einfach

- Jeder Wert hat genau einen **Owner**
- Besitz kann übertragen (`move`d) werden
- Besitzer dürfen Werte ausleihen ("borrow")
- Besitz verpflichtet zum Aufräumen (`drop`)

Besitz bedeutet Verantwortung!

---

## Borrowing ist auch einfach

- Ein Wert darf einmalig veränderlich verliehen sein
- Ein Wert darf mehrmals unveränderlich verliehen sein
- Ein Wert darf nicht gleichzeitig veränderlich und unveränderlich ausgeliehen werden

Wenn ein Verleih endet, kann eine andere Art des Verleihens beginnen.
Es ist nicht möglich, eine verliehene Ressource aufzuräumen.

---

## Lifetimes "eigentlich auch" einfach

Einzige wichtige Lifetime für heute: `'static`

- Ein `'static` **borrow** markiert die globale scope
- Ein `'static` **trait bound** markiert einen Wert, der für sich steht
  - Also keine non-`'static` referenzen enthält
  - Ein Wert der nicht `'static` erfüllt kann nicht in einen anderen Thread gegeben werden
  - (`Send` auch erforderlich wegen z.b. Thread Locals)

---

## Warum das alles?

Ursprünglicher Zweck von Ownership, Borrowing, Lifetimes: Thread Safety

- Sekundärer Effekt: Sicheres statisches Ressourcenmanagement
- Tertiärer Effekt: Klare Systemgrenzen

<!-- _footer: '[Fearless Concurrency with Rust (Yehuda Katz, 2015)](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)' -->

---

## Beispiel: Signatur von [`std::vec::Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)

`self` wird als "mutable reference" übergeben,
`value` wird in den Vektor ge`move`d.

````rust
impl<T, A: Allocator> Vec<T, A> {
    pub fn push(&mut self, value: T) {
````

<!-- _footer: '[`push` in std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)' -->

---

## Beispiel: [`std::vec::Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)

Das Argument `"--help"` als `String` auf dem Heap wird in den Vektor ge`move`d:

````rust tag:playground-button playground-wrap:main
let mut numbers: Vec<String> = std::env::args().collect();
let arg = String::from("--help");
numbers.push(arg);
println!("{numbers:?}");
println!("{arg}");
````

Kompiliert dieser Schnipsel?

---

## Trait [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)

Fast nur ein Slide:

````rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
````

---

## Enum [`Poll`](https://doc.rust-lang.org/std/task/enum.Poll.html)

Isomorph zu `Option<T>`:

````rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
````

Pollen einer `Future` führt irgendwann zu einem `Poll::Ready(T)`.

---

## Struct [`Context`](https://doc.rust-lang.org/std/task/struct.Context.html#)

Enthält einen [`Waker`](https://doc.rust-lang.org/std/task/struct.Waker.html):

- Runtime gibt beim pollen einen `Waker` über den `Context` mit
- Der `Waker` wird im Event Loop registriert (als Callback)
- Später, bei interessantem Event, wird über den `Waker` die lauffähige Future markiert

Kurz: Futures werden nicht busy-polled, sondern wenn ein für sie interessantes Event eintritt.

<!-- _footer: '[z.B. epoll](https://man7.org/linux/man-pages/man7/epoll.7.html)' -->

---

## Futures erstellen

Viele Futures schaufeln Daten von hier nach dort:

````rust
pub async fn handle_client(
    mut reader: AsyncJsonLinesReader<BufReader<OwnedReadHalf>>,
    collect_request: mpsc::Sender<HostToSensor>,
) -> anyhow::Result<()> {
    while let Ok(Some(msg)) = reader.read::<HostToSensor>().await {
        collect_request.send(msg).await?;
    }
    Ok(())
}
````

---

## Aside: `HostToSensor`?

Die Future oben liest Json-encodierte Objekte aus dem Stream.
Die Typ-Information dazu wird über `serde::De/Serialize` deriviert:

````rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HostToSensor {
    Trigger,
    SetConfig(config),
}
````

<!-- _footer: '[serde.rs](https://serde.rs)' -->

---

## Nebenläufige Futures (Kombinatoren)

`join!` und `select!` aus `tokio` kombinieren Futures:

````rust
loop {
    select!(
        Some(msg) = collector.recv()   => handle_message(msg),
        _tick     = interval.tick()    => send_ping().await,
        _expiry   = wdg.expired()      => return,
        else                           => break,
    )
}
````

````pre
Refutable Pattern = Progress Futures   => Winner
````

---

## `select!` Eigenschaften

- Concurrency ohne thread oder task
- Mehrere konkurrierende Futures haben sicheren Zugriff auf alle lokalen Variablen
- Fairness: Zufällige Auswahl des Branches
- Variante mit dynamischer Anzahl an Futures: `FuturesUnordered`
- Mehr: `biased`, `else`-branch, `<preconditions>`, ...

<!-- _footer: '[`tokio::select!` docs](https://docs.rs/tokio/latest/tokio/macro.select.html)' -->

---

## Das `loop`-`select!` Muster

Gerne lebt das `select!` in einem `loop`: [GitHub Code Search](https://github.com/search?type=code&auto_enroll=true&q=select%21+language%3ARust+owner%3Abarafael+)

Damit werden die Progress Futures sequentiell abgearbeitet.

---

## `select!` hat Tücken

Mächtiger aber schwieriger Kombinator:

- Panic wenn alle branches disabled sind und es keinen `else` branch gibt
- Cancellation safety: Progress Futures müssen damit klar kommen, dass sie an jedem `.await` nicht weiter gepollt werden (Seiteneffekte erst, wenn sie `Poll::Ready(t)` sind).
- IDE assistance und Formatting versagen (innerhalb)

---

## Parallelismus mit [`tokio::spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)

- `tokio::spawn` lädt eine Future in den work-stealing thread pool von tokio.
- Kooperatives Multitasking: `.await` Punkt markiert yield
- (`.await` = Zustand in der Future State Machine)
- Tasks sind stackless! Nur die Größe der Future zählt, und deren Maximum ist berechenbar.

---

## `spawn` Beispiel

````rust tag:playground-button playground-before:$"use tokio::io::AsyncWriteExt; use tokio::net::TcpStream; #[tokio::main] async fn main() -> anyhow::Result<()> { let mut buffer = [0u8; 128]; let message = b"wie man in den wald hinein ruft"; const ECHO_SERVER: &str = "tcpbin.com:4242"; "$ playground-after:$"}"$
let mut stream = TcpStream::connect(ECHO_SERVER).await?;
let handle = tokio::spawn(async move {
    loop {
        stream.write_all(message).await?;
        let len = stream.read(&mut buffer).await?;
        assert_eq!(message[..], buffer[..len]);
    }
});
````

---

<!--
## `'static` bound für `spawn`

````rust tag:playground-button playground-before:$"use std::time::Duration; #[tokio::main] async fn main() {"$ playground-after:$"}"$
let mut counter = 0;
async fn increment(reference: &mut i32) {
    loop {
        *reference += 1;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
tokio::spawn(increment(&mut counter));
tokio::spawn(increment(&mut counter));
````

`tokio::spawn` erfordert `'static`.

-->
<!--
---

## Aside: Wie viele Bugs?

* `counter` out of scope während es noch Referenzen auf ihn gibt
* `counter` mutably borrowed an zwei Stellen

Soft bug:
* Handles zu gespawnten Tasks werden direkt ge`drop`ped
  - Tasks laufen weiter, bis `main` terminiert
  - (`tokio`) Tasks laufen unabhängig von ihren Handles
    (siehe [`tokio::join!`](https://docs.rs/tokio/latest/tokio/macro.join.html))

---

-->

---

> Don't communicate by sharing memory,
> share memory by communicating
> <cite>Wahrscheinlich Rob Pike</cite>

Und:

> Lock data, not code
> <cite>[Aaron Turon](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)</cite>

---

<!--
## Sharing XOR Mutation

Data Race: Mutation von Daten die aliased sind (Sharing AND Mutation)

* Haskell, funktionale: !Mutation
* Erlang: !Sharing
* Rust: Sharing XOR Mutation

Heute: !Sharing, limited Mutation - konzeptionell ähnlich zu Erlang.

-->

<!--
---

## Beispiel: TCP-Server für USB-Gerät

Anforderungen:

* Öffne USB-Gerät
* Öffne TCP Listening Socket
* Für jede eingehende TCP-Verbindung:
  - Spawne einen Client Task
  - Jeder Task kann Kommandos an das USB-Gerät senden
  - Jede Antwort des USB-Gerätes wird an jeden verbundenen Client gesendet
  - Der Server pingt das USB-Gerät periodisch

-->
<!-- 
---

Eine sehr einfache Topologie.

Collect Commands,
Broadcast Events.

![bg right:66% height:400px](images/mt21-server-architecture.drawio.svg)

---
-->
<!-- 
## Aktor für TCP Listener

Klischee Server Loop:

````rust
loop {
    let (stream, client_addr) = listener.accept().await.context("Failed to accept")?;

    let rx = sensor_event.resubscribe();
    let tx = sensor_request.clone();
    tokio::spawn(async move { // Client Aktor spawned hier!
        if let Err(e) = handle_client(stream, client_addr, rx, tx).await {
            error!("Error while servicing client: {e}");
        }
    });
}
````
-->

<!--

---

## Aktor für Client

Auch Klischee: Requests ans Gerät weiterleiten (mpsc), Ereignisse an den Nutzer senden. 
````rust
loop {
    select! {
        Ok(Some(msg)) = reader.read::<HostToSensor>() => collect_request.send(msg).await,
        Ok(msg) = broadcast_event.recv() => writer.write(&msg).await.unwrap(),
        else => break;
    }
}
````
-->

<!-- 
---

## Aktor für Client (wirklich)

````rust
loop {
    select! {
        msg = reader.read::<HostToSensor>() => match msg {
            Ok(None) => {
                info!("Goodbye {addr}");
                break;
            },
            Ok(Some(msg)) => {
                if let Err(error) = collect_request.send(msg).await {
                    error!(%error, "broadcast failed");
                }
            },
            Err(error) => { // TODO just ignore invalid JSON by further matching.
                error!(%error, "Error while reading SensorCommand");
                break;
            }
        }
        msg = broadcast_event.recv() => {
            match msg {
                Ok(msg) => {
                    writer.write(&msg).await.unwrap();
                },
                Err(broadcast::error::RecvError::Lagged(n)) => warn!(count = %n, "Missed messages"),
                Err(error) => {
                    error!(%error, "transport error receiving SensorEvent");
                    break;
                }
            }
        }
    }
}
````
-->
<!-- 
---

## USB Device Aktor

````rust
loop {
    let (sink, stream) = loop {
        match open_device() {
            Err(error) => {
                tokio::time::sleep(RECONNECTION_DELAY).await;
                continue;
            }
            Ok(t) => break t,
        }
    };
    collector = service_device(collector, sensor_event_broadcast.clone(), stream, sink).await?;
    sensor_event_broadcast.send(SensorToHost::Disconnection)?
}
````

`service_device` gibt nach regulärem beenden den `mpsc::Receiver` zurück.

-->
<!-- 
---

## USB Device Aktor (Intern)

````rust
loop {
    select! {
        _ = interval.tick() => ping_device().await,
        _ = &mut wdg_expiration => break,
        Some(msg) = request_collection.recv() => request_from_sensor(msg).await,
        Some(Ok(item)) = reader.next() => {
            wdg_reset.send(Signal::Reset).await;
            event_broadcast.send(item);
        }
        else => break,
    }
}
````
-->

---

## Grundbaustein: Channel

- Collector ([`mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/fn.channel.html)): Sammelt Nachrichten Zentral
  - Das Sender-Handle ([`mpsc::Sender`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html)) ist `std::clone::Clone`
  - Der Channel schließt, wenn alle Nachrichten bearbeitet sind und es keine [`mpsc::Sender`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html) mehr gibt

---

## Eine Aktor-Basierte Architektur

![center width:1100px](images/cascaded-cancellation.drawio.svg)

---

## Cancel Culture (für Aktoren)

Am besten terminiert ein Aktor von alleine, wenn seine Zeit gekommen ist.

Beispiel:

````rust tag:playground-button playground-before:$"use tokio_stream::StreamExt; #[tokio::main] async fn main() { let mut stream = tokio_stream::iter(&[1, 2, 3]); "$ playground-after:$"}"$
while let Some(v) = stream.next().await {
    println!("GOT = {:?}", v);
}
````

<!-- _footer:'[Streams chapter of tokio tutorial](https://tokio.rs/tokio/tutorial/streams)' -->

---

## Cancellation Token Antipattern

Cancellation Token sollte nicht verwendet werden, um Aktoren zu beenden, die von alleine ein valides Shutdown-Verhalten haben.

Ein Aktor hängt ja (über channel handles) von anderen Aktoren ab. Shutdown ist damit (außer bei Zyklen) strukturiert.

---

## Cancellation Token Antipattern

````rust
loop {
    select!(
        _ = cancellation_token.cancelled() => println!("Cancelled, Good bye"),
        Some(msg) = rx.recv() => {
            handle_message(msg).await;
        }
        else => break;
    )
}
````

Problem: Unbearbeitete Nachrichten verbleiben im Channel und werden gedropped.

---

## Automatisches Aufräumen

Wenn kein Sender mehr da ist und keine Nachrichten mehr im Channel sind, gibt die `recv()` future `None` zurück.

````rust
match msg {
    Some(Signal::Reset) => {
        // on reset: set active, restart sleep.
    }
    // on channel end: exit watchdog.
    None => break,
}
````

Keine garbage collection, aber irgendwo doch...

<!-- _footer: '[Watchdog actor auf GitHub](https://github.com/barafael/watchdog/blob/dddbc4debd759ca195fea4ffe945334e425515c7/src/lib.rs#L67C1-L78C1)' -->

---

## [Protohackers Problem 6](https://protohackers.com/problem/6)

![bg right height:90%](images/speedd.drawio.svg)

<!-- _footer: "[speedd.drawio.png](https://github.com/barafael/protohackers/blob/main/speedd/speedd.drawio.png)" -->

---

![center height:600px](images/live-topology.drawio.svg)

---

## Bausteine für das Aktor Pattern

- Futures als Zero-Cost [Abstraktion](https://doc.rust-lang.org/std/future/trait.Future.html) über Event-getriebene Berechnung
- Zero Cost Kombinatoren: `join!` und `select!` für Concurrency
- Stackless Coroutines für Parallelismus (tokio tasks)
- Channels mit Semantik für verschiedene Topologien
  - mpsc (collector), oneshot, broadcast, watch

---

## Aspekte des Aktor Patterns

- Aktoren besitzen eine I/O Ressource (Socket, File Handle, etc.)
  - Nebeneffekte werden innerhalb eines Aktors abgehandelt
- Aktoren sind vollständig isolierte Entitäten
  - Kein eigener Heap wie in Erlang: `'static` bound verhindert sharing
- Ein Aktor schläft wenn es nichts zu tun gibt
- Ein Aktor ist unsichtbar, er ist lediglich durch Channel Handles erreichbar
  - Die Message-Tabelle ist eine Art V-Table! Der Typ des Aktors ist unsichtbar.

---

## Aktoren auf Mikrokontroller

mt21 device architektur

---

## Alan Kay

oop in rust ja nein? Alan Kay quotes/perspektiven
