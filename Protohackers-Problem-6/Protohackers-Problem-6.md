---
marp: true
theme: rhea
---

<!--
paginate: true
 -->

<!-- 
_footer: ''
_paginate: false
 -->

<!-- _class: lead -->

# Protohackers<br>Solution for Problem 6<br>(Speed Daemon)

---

<!-- header: ' ' -->

# Problem: State has no money

### Also Problem: People are driving too fast

1. Set speed limits
2. Install cameras
4. ?!
5. Profit!

---

# Problem: State has no money

### Also Problem: People are driving too fast

1. Set speed limits
2. Install cameras
4. __Write some software__
5. Profit!

---

# [Problem Description](https://protohackers.com/problem/6)

* Vehicles have license plates, which are ASCII Strings.
* Roads have identifiers (`u16`).
* Roads are equipped with cameras.
* Each road has a fixed speed limit (`u16`).
* Cameras are positioned at integer mile marks (`u16`).

---

# [Problem Description](https://protohackers.com/problem/6)

* When a vehicle passes a camera, the camera reports:
  The license plate, the time, the road, and the speed limit.
* When a speed limit is breached by a car, a ticket is dispatched to exactly one dispatcher registered for this road.
* If there is no dispatcher for this road yet, the ticket must be queued for later.
* There can be at most one ticket per day per car.
* Multi-day tickets are possible though.

---

# Some Problem Analysis

Our task is to implement a server which the cameras and ticket dispatchers connect to.
Our server shall process camera reports and dispatch tickets.
Each client, before or after it identifies as Camera or Dispatcher, may activate a heartbeat sender with customizable rate (the heartbeat messages go from server to client).

---

# Some Problem Analysis

* Roads are independent. Crossroads don't matter/don't exist.
* Clients, Cameras, and Dispatchers can all activate Heartbeat exchange, but each client can do this only once.
* Dispatchers must register, but Ticket dispatch must be independent of the number of registered Dispatchers.
* Details: Only once a day ticketing is challenging.
* We'll need plenty of channels, probably.

---

# Static Architecture

* Listener
* Collector
* Clients
  - Camera
  - Dispatcher
* Heartbeats

![bg right height:90%](images/speedd.drawio.svg)

<!-- _footer: "[speedd.drawio.png](https://github.com/barafael/protohackers/blob/main/speedd/speedd.drawio.png)" -->

---

# A live snapshot

![center height:500px](images/live-topology.drawio.svg)

<!-- _footer: "[live-topology.drawio.png](https://github.com/barafael/protohackers/blob/main/speedd/live-topology.drawio.png)" -->

---
