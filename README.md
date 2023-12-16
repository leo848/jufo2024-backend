# jufo2024-backend

Dieses Repository enthält den Code für den Backend-Webserver des Jugend forscht-Projekts "sorting the colors: Dimensionsbezogene Generalisierung vergleichsbasierter Sortierung". Der Server ist in [Rust](https://www.rust-lang.org) geschrieben.

## Installation

```
git clone https://github.com/leo848/jufo2024-backend
cd jufo2024-backend
cargo run
```

Auf dem Port 3141 wird ein Webserver gestartet, der einkommende Requests über das Websocket-Protokoll beantwortet. Eine dazugehörige Frontend-Anwendung, [jufo2024-frontend](https://github.com/leo848/jufo2024-frontend), nutzt diesen Server zur Sortierung verschiedener Arten von Objekten.

## Funktionen

- Zahlen sortieren
    - Algorithmen: Bubble Sort, Selection Sort, Insertion Sort
    - Bei jedem Zwischenschritt wird das Ergebnis mitgeteilt
    - Zusätzliche Ausgabe der Aktionen des Algorithmus
- Pfad erstellen
    - Algorithmen: Zufällig, Reihenfolge, Brute Force, Nächster Nachbar, Greedy, geplant: Christofides, Concorde
    - Aus beliebiger Liste von Vektoren so einen Pfad erstellen
- Pfad verbessern
    - Einen bestehenden Pfad verbessern
    - Algorithmen: Rotieren, 2-opt, 3-opt, geplant: Simulated Annealing, Ameisenkolonie
