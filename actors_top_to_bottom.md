---

title: "Actors Top To Bottom"
description: ""
marp: true
theme: rhea
color: "dark-gray"
size: 16:9
style: |
  .columns {
    display: grid;
    gap: 1rem;
  }
  .columns-left {
  }
  .columns-right {
  }

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

<!-- 
_paginate: false
 -->

## `$whoami`

![bg right](images/colorkit%20(7).png)

- Lerne Rust seit 2016
- Embedded (C, C++, Rust)
- Automotive (Rust)
- Distributed Systems (Rust)
- Seit 2020: Rust Meetup Nuremberg
- Opensource auf GitHub: [github.com/barafael](https://github.com/barafael)

---

## Motivation I

Nebenläufige und parallele Systeme sind überall:

- Web Backends jeglicher Art
- Message Broker
- Datenbanken
- LSP-Server
- ECU Services (DoIP, UDS, etc.)
- MCU Firmware

Wie kann man solche Systeme sinnvoll programmieren?

---

## Motivation II

Ursprünglicher Zweck von Ownership, Borrowing, Lifetimes:

- Safe Systems
- Painless Concurrency

Aus Versehen: **Thread Safety** (keine Data Races)

Tertiärer Effekt (Behauptung von mir): Klare Systemgrenzen

<!-- _footer: '["Fearless Concurrency with Rust" (Yehuda Katz, 2015)](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)' -->

---

## Motivation III

Ownership = Verantwortung

System Architektur verteilt Verantwortung im System

Korrekte Ownership = Gute Architektur?

---

## Fahrplan

- Der Future Trait
- Nebenläufigkeit mit Future-Kombinatoren
- Parallelität mit Futures und Tokio
- Channels und ihr Einfluss auf die System-Architektur
- Exemplarische Anwendungen aus der Vogelperspektive
  - Broker-Anbindung an große Legacy Software
  - USB-Gerät Server
  - USB-Gerät Firmware
- Was ist also ein Aktor? Und ist Rust Objektorientiert?

---

## Vorraussetzungen

- "Rust schon mal gesehen"
- Ownership, Borrowing, Lifetimes

---

## Trait [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)

Passt auf einen Slide:

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

`Poll::Pending` ähnlich zu unix `EWOULDBLOCK`.
`Poll::Ready(T)` markiert das Ende eines Vorgangs.

---

## Struct [`Context`](https://doc.rust-lang.org/std/task/struct.Context.html#)

Enthält einen [`Waker`](https://doc.rust-lang.org/std/task/struct.Waker.html):

- Runtime gibt beim pollen einen `Waker` über den `Context` mit
- Der `Waker` wird im [Event Loop](https://man7.org/linux/man-pages/man7/epoll.7.html) registriert (wie ein Callback)
- Später, bei bestimmtem Event, wird über den `Waker` die lauffähige Future markiert

Kurz: Futures werden nicht busy-polled, sondern wenn ein für sie relevantes Event eintritt.

<!-- _footer: '["Learning Async Rust With Entirely Too Many Web Servers" von Ibraheem Ahmed](https://ibraheem.ca/posts/too-many-web-servers/)' -->

---

## Unterschiede zu Promises, Coroutinen

Eine `Future` ist lediglich ein Typ, welcher den `Future` Trait implementiert.

Von alleine tut eine `Future` gar nichts!

Eine `Future` repräsentiert deklarativ einen Vorgang.

<!-- _footer: '[Future Docs](https://doc.rust-lang.org/std/future/trait.Future.html#)' -->

---

## Futures erstellen

Eine Future zum Parsen von JSON-Daten auf einem Socket:

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

## Nebenbei: `HostToSensor`?

Die Typ-Information zum Parsen in der vorigen Future wird über `serde::De/Serialize` deriviert:

````rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HostToSensor {
    Trigger,
    SetConfig(config),
}
````

<!-- _footer: '[serde.rs](https://serde.rs)' -->

---

## Nebenläufigkeit mit `Future`s (Kombinatoren)

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
- Tasks sind stackless! Nur die Größe der Future zählt, und deren Maximum ist berechenbar.

> Tasks in Tokio are very lightweight. Under the hood, they require only a single allocation and 64 bytes of memory. Applications should feel free to spawn thousands, if not millions of tasks.
<cite>[The Tokio Tutorial](https://tokio.rs/tokio/tutorial/spawning)</cite>

---

## Kooperatives Multitasking (done right)

`.await` markiert yield point (`.await` = Zustand in der Future State Machine).

Das bedeutet auch: Blocking I/O oder lange Berechnungen oder loops **ohne** `.await` haben in einem Task nichts zu suchen.

Rust kann nicht davor schützen, dass ein Task zu lange läuft. Wenn das passiert, dann sinkt die Reaktivität des Systems (ein Worker Thread weniger im Pool).

---

### `spawn` Beispiel

Eine TCP Client Connection in einem Task:

````rust tag:playground-button playground-before:$"use tokio::io::AsyncWriteExt; use tokio::net::TcpStream; #[tokio::main] async fn main() -> anyhow::Result<()> { let mut buffer = [0u8; 128]; let message = b"wie man in den wald hinein ruft"; const ECHO_SERVER: &str = "tcpbin.com:4242"; "$ playground-after:$"}"$
let mut stream = TcpStream::connect(ECHO_SERVER).await?;
let handle = tokio::spawn(async move {
    loop {
        stream.write_all(message).await?;
        let len = stream.read(&mut buffer).await?;
        assert_eq!(message[..], buffer[..len]);
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
});
````

Aus dem Playground ist der `ECHO_SERVER` nicht erreichbar.

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

## Nebenbei: Wie viele Bugs?

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
> <cite>[Effective Go](https://go.dev/doc/effective_go), möglicherweise Rob Pike</cite>

Und:

> Lock data, not code
> <cite>[Aaron Turon](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)</cite>


<!--
---
## Sharing XOR Mutation

Data Race: Mutation von Daten die aliased sind (Sharing AND Mutation)

* Haskell, funktionale: !Mutation
* Erlang: !Sharing
* Rust: Sharing XOR Mutation

Heute: !Sharing, limited Mutation - konzeptionell ähnlich zu Erlang.

-->

<!--

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

Ein Channel erlaubt es, eine Datenstruktur im Programm zu verschieben

- Ohne Zeitliche Kopplung
- Mit Verschiebung der Ownership
- Detaillierte Semantik:
  - Many to One, One to Many, One to Many with History, One to One, One to One ONCE, etc.

Channels sind der Grundbaustein für Aktor-Basierte Architekturen!

---

## Collector Channel

- [`mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html): Sammelt Nachrichten Zentral (Many to One)
  - Das Sender-Handle ([`mpsc::Sender`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html)) ist `std::clone::Clone`
  - Das Empfänger-Handle ist **nicht** `std::clone::Clone`!
  - Der Channel schließt, wenn alle Nachrichten bearbeitet sind und es keine [`mpsc::Sender`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html) mehr gibt
  - Senden auf einem bounded channel ist **asynchron**! Wenn der Channel voll ist, wird der Sender geblockt.

---

### Deadlock durch Backpressure

Nur bei Bounded Variante möglich.

Auch ein `unbounded` Channel ist eventually bounded. Man sollte niemals annehmen, über unendlich Ressourcen zu verfügen.

Was fällt hier auf?

![bg right width:100%](images/actor_cycle.drawio.svg)

---

## Oneshot Channel

- [`oneshot`](https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html): Übergabe **genau eines** Wertes von A nach B (One to One ONCE).
- Senden konsumiert den Sender
- Senden ist synchron
- Empfangen konsumiert den Empfänger

In Kombination mit `mpsc`: Request-Reply Muster!

---

## Request Reply mit MPSC/Oneshot

Beim Stellen der Anfrage steckt der Requestor den Reply Channel mit in die Nachricht:

````rust
enum Message {
    Trigger,
    GetValue(oneshot::Sender<Value>),
}
````

Nach dem Stellen der Anfrage wartet der Sender auf eine Antwort auf dem [`oneshot::Sender`](https://docs.rs/tokio/latest/tokio/sync/oneshot/struct.Receiver.html).

Deadlock Potenzial!

---

## Broadcast Channel

- [`broadcast`](https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html): Verteilt Nachrichten von mehreren Sendern an **jeden** Empfänger (One to Many, Many to Many).
- Historie speichert letzte Einträge (bounded). Wenn ein Empfänger hinterher hängt (zu langsam abholt) werden für ihn die ältesten Nachrichten verworfen.
- Channel schließt, wenn alle Sender verschwunden und Nachrichten abgehandelt sind.

---

## Weitere Channels

- [`watch`](https://docs.rs/tokio/latest/tokio/sync/watch/index.html): `broadcast` ohne history, letzter Wert zählt
- [`mpmc`](https://docs.rs/async-channel/latest/async_channel/): Verteilt Nachrichten von vielen Sendern an je genau einen Empfänger (Many to Any)
- [`PriorityChannel`](https://docs.embassy.dev/embassy-sync/git/default/priority_channel/struct.PriorityChannel.html#): Mpmc mit Prioritäten für Messages

Schlussendlich: Channels geben maßgeblich Architektur vor!

---

## Eine Aktor-Basierte Architektur

![center width:1100px](images/cascaded-cancellation.drawio.svg)

---

## Eigenschaften der Architektur I

Klare Auftrennung der Aufgaben!
Verwaltung: Connector. Business Logik: Session. Effekt: Callback.

- Information fließt nur von links nach rechts
- Directed Acyclic Graph!
- Anzahl der Sessions praktisch unlimitiert
- Typ der Session ist erweiterbar (Polymorphie)

---

## Eigenschaften der Architektur II

- Alles hängt am mpsc Sender oben links
  - Shutdown: dieses eine Handle droppen, der Rest zieht nach sobald er kann
- Zykelfrei
  - Kein Deadlock möglich
  - Kein Livelock möglich (Nachrichten die im Kreislauf feststecken)
- Panic in einer Session hat keinen negativen Effekt
- Panic im Connector über mpsc Sender detektierbar

---

## Management eines Sensors über USB

<div class="columns">
<div class="columns-left">

- Klischee Applikation: Verwaltet Sensor, sammelt Nachrichten daran zentral, broadcasted Nachrichten vom Gerät an alle Clients
- Eher IPC als TCP
- Kommunikation mit Clients via JSON, mit device via [postcard/COBS](https://docs.rs/postcard/latest/postcard/)

</div>
<div class="columns-right">

![width:40%](images/mt21-server-architecture.drawio.svg)

</div>
</div>

---

## USB Device Aktor (Intern)

`service_device`:

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

---

### USB Device Aktor (Retry Loop)

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

---

## Aktor für Client

Beispiel von oben: Requests an Device Aktor weiterleiten (mpsc), Ereignisse an den Nutzer senden.

````rust
loop {
    select! {
        Ok(Some(msg)) = reader.read::<HostToSensor>() => collect_request.send(msg).await,
        Ok(msg) = broadcast_event.recv() => writer.write(&msg).await?,
        else => break;
    }
}
````

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
                    writer.write(&msg).await?;
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

---

## Aktor für TCP Listener

Klischee Server Loop:

````rust
loop {
    let (stream, client_addr) = listener.accept().await.context("Failed to accept")?;

    let rx = sensor_event.resubscribe();
    let tx = sensor_request.clone();
    tokio::spawn(async move { // Client Aktor spawned hier!
        if let Err(e) = handle_client(stream, client_addr, rx, tx).await {
            error!("Error while servicing client {client_addr}: {e}");
        }
    });
}
````

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

![center width:100%](images/channels_and_cores.drawio.svg)

---

## Alan Kay

oop in rust ja nein? Alan Kay quotes/perspektiven
