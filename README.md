# jufo2024-backend

Dieses Repository enthält den Code für den Backend-Webserver des Jugend forscht-Projekts "sorting the colors: Dimensionsbezogene Generalisierung vergleichsbasierter Sortierung". Der Server ist in [Rust](https://www.rust-lang.org) geschrieben.

## Installation

```
git clone https://github.com/leo848/jufo2024-backend
cd jufo2024-backend
cargo run
```

Auf dem Port 3141 wird ein Webserver gestartet, der einkommende Requests über das Websocket-Protokoll beantwortet. Eine dazugehörige Frontend-Anwendung, [jufo2024-frontend](https://github.com/leo848/jufo2024-frontend), nutzt diesen Server zur Sortierung verschiedener Arten von Objekten.

Für die Nutzung von Word2Vec muss zudem eine binäre Modelldatei heruntergeladen und im neu zu erstellenden Verzeichnis `jufo2024-backend/nlp` als `model.bin` gespeichert werden, ansonten wird bei jeder NLP-Anfrage nur `Unsupported` zurückgegeben. Beispielhafte solche Dateien finden sich unter [vectors.nlpl.eu](https://vectors.nlpl.eu/repository). Die Datei wird dabei vollständig in den Arbeitsspeicher gelesen.

## Funktionen

- Zahlen sortieren
    - Algorithmen
        - Bubble Sort
        - Selection Sort
        - Insertion Sort
    - Bei jedem Zwischenschritt wird das Ergebnis mitgeteilt
    - Zusätzliche Ausgabe der Aktionen des Algorithmus
- Pfad erstellen
    - Aus beliebiger Liste von Vektoren einen Hamilton-Pfad erstellen
    - Algorithmen:
        - Zufällig
        - Reihenfolge
        - Brute Force
        - Nächster Nachbar
        - Greedy
        - Christofides (geplant)
        - Concorde (geplant)
- Pfad verbessern
    - Einen bestehenden Pfad verbessern
    - Algorithmen:
        - Rotieren
        - Swap
        - 2-opt
        - 3-opt
        - Simulated Annealing
        - Ameisenkolonie (geplant)
