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

## Das Ownership-System

* Rust Ownership:
  - Jeder Wert hat genau einen **Besitzer**
  - Besitz kann übertragen werden
  - Besitz verpflichtet zum Aufräumen
* Besitz == Verantwortung

Ursprünglicher Sinn: Thread Safety
Aus Versehen: Sicheres statisches Management von Ressourcen

---

## Ownership/Move-Semantik Beispiel

Das Argument `"--help"` als `String` auf dem Heap wird in den Vektor ge`move`d:

````rust tag:playground-button playground-before:$"fn main() {"$ playground-after:$"}"$
let mut numbers: Vec<String> = std::env::args().collect();
let arg = String::from("--help");
numbers.push(arg);
println!("{numbers:?}");
// println!("{arg}"); // Compile error
````

---

## Ownership + Borrowing + Multitasking

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

---

## Aside: Wie viele Bugs?

* `counter` out of scope before reference to it
* `counter` borrowed mutably twice
* Handles to spawned tasks are dropped after spawning
  - Tasks keep running until main terminates
  - (Tasks DO NOT terminate when handle is dropped!)

---

## Aber wie kooperieren ohne sharing?

Man kann natürlich Mutexe etc. nutzen. Rob Pike jedoch empfiehlt:

> Don't communicate by sharing memory,
> share memory by communicating
> <cite>Rob Pike<cite>

Wie gut eignen sich Tasks + Channels zur Umsetzung von Systemen?

---

## Architektur eines einfachen Servers

---

## Beispiel: einfacher TCP-Server

Anforderungen:

* Öffne USB-Gerät
* Öffne TCP Listening Socket
* Für jede eingehende TCP-Verbindung:
  - Spawne einen Client Task
  - Jeder Task kann Kommandos an das USB-Gerät senden
  - Jede Antwort des USB-Gerätes wird an jeden verbundenen Client gesendet

---

## Beispiel: einfacher TCP-Server

Eine sehr einfache Topologie.

Collect Commands,
Broadcast Events.

![bg right:66% height:400px](images/mt21-server-architecture.drawio.svg)

---

das beispiel wird natürlich zu einem aktor
mt21 server architektur

---

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

# [Problem Description](https://protohackers.com/problem/6)


![bg right height:90%](images/speedd.drawio.svg)

<!-- _footer: "[speedd.drawio.png](https://github.com/barafael/protohackers/blob/main/speedd/speedd.drawio.png)" -->

---

![center height:500px](images/live-topology.drawio.svg)

---

## Bausteine für das Aktor Pattern

* Futures als Zero-Cost [Abstraktion](https://doc.rust-lang.org/std/future/trait.Future.html) über Event-getriebene Berechnung
* Zero Cost Kombinatoren: `join!` und `select!` für Concurrency
* Stackless Coroutines für Parallelismus (tokio tasks)
* Channels mit Semantik für verschiedene Topologien
  - mpsc (collector), oneshot, broadcast, watch

---

## Aspekte des Aktor Patterns

* Aktoren besitzen eine I/O Ressource (Socket, File Handle, etc.)
  - Nebeneffekte werden innerhalb eines Aktors abgehandelt
* Aktoren sind vollständig isolierte Entitäten
  - Kein eigener Heap wie in Erlang: `'static` bound verhindert sharing
* Ein Aktor schläft wenn es nichts zu tun gibt
* Ein Aktor ist unsichtbar, er ist lediglich durch Channel Handles erreichbar
  - Die Message-Tabelle ist eine Art V-Table! Der Typ des Aktors ist unsichtbar.

---

## Wie allgemeintauglich ist es?

mt21 device architektur

---

# Schlussfolgerung:

oop in rust ja nein? Alan Kay quotes/perspektiven
