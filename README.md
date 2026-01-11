# code2md

> Projektcode übersichtlich in einer Markdown-Datei zusammenfassen

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

---

## Was macht code2md?

**code2md** ist ein schnelles Kommandozeilen-Werkzeug, das den gesamten Quellcode eines Projekts in eine einzige, übersichtliche Markdown-Datei exportiert. Das Ergebnis ist ein strukturiertes Dokument mit Ordnerübersicht und allen Code-Dateien – perfekt lesbar und mit Syntax-Highlighting.

### Wofür ist das nützlich?

- **Dokumentation** – Erstellen Sie eine Übersicht Ihres Projekts für Kollegen oder Kunden
- **Code-Reviews** – Teilen Sie Ihren Code ohne Zugang zum Repository
- **KI-Assistenten** – Übergeben Sie Ihr gesamtes Projekt an ChatGPT, Claude & Co.
- **Archivierung** – Speichern Sie einen lesbaren Snapshot Ihres Codes
- **Lernen** – Studieren Sie fremde Projekte in einem übersichtlichen Format

### Warum Rust?

- **Schnell** – 10-50x schneller als vergleichbare Python-Tools
- **Standalone** – Eine einzige Binary, keine Abhängigkeiten
- **Cross-Platform** – Läuft auf Linux, macOS und Windows

---

## Installation

### Voraussetzungen

- Rust 1.70 oder neuer (nur zum Kompilieren)

### Aus dem Quellcode

```bash
# Repository klonen
git clone https://github.com/m0h1nd4/rusty-code2md.git
cd code2md

# Release-Build erstellen
cargo build --release

# Die Binary liegt nun in ./target/release/rusty-code2md
```

### Systemweit installieren

```bash
# Via cargo install
cargo install --path .

# Oder manuell
sudo cp target/release/code2md /usr/local/bin/
```

### Vorkompilierte Binaries

Vorkompilierte Binaries für gängige Plattformen finden Sie unter [Releases](https://github.com/m0h1nd4/rusty-code2md/releases).

---

## Schnellstart

### Einfaches Beispiel

```bash
# Python-Projekt exportieren
code2md ./mein-projekt --type python
```

Das erstellt eine Datei `mein-projekt_code.md` mit folgendem Aufbau:

```markdown
# mein-projekt

> Generiert am 2024-01-15 14:30:00
> Anzahl Dateien: 12

## Ordnerstruktur

mein-projekt/
├── src/
│   ├── main.py
│   └── utils/
│       └── helpers.py
└── config.json

## Dateien

### `src/main.py`

def main():
    print("Hello, World!")

### `src/utils/helpers.py`

def greet(name):
    return f"Hallo, {name}!"
```

---

## Verwendung

### Grundsyntax

```bash
code2md [VERZEICHNIS] --type [PROJEKTTYP] [OPTIONEN]
```

### Projekttypen

code2md kennt viele gängige Projekttypen und weiß automatisch, welche Dateien dazugehören:

| Typ | Beschreibung | Dateiendungen |
|-----|--------------|---------------|
| `python` | Python-Projekte | `.py`, `.pyi`, `.pyw` |
| `arduino` | Arduino & C++ | `.ino`, `.cpp`, `.c`, `.h`, `.hpp` |
| `vue` | Vue.js | `.vue`, `.js`, `.ts`, `.css`, `.scss` |
| `react` | React.js | `.jsx`, `.tsx`, `.js`, `.ts`, `.css` |
| `web` | HTML/CSS/JS | `.html`, `.css`, `.js`, `.ts` |
| `php` | PHP | `.php`, `.phtml` |
| `node` | Node.js | `.js`, `.ts`, `.mjs`, `.json` |
| `flutter` | Flutter/Dart | `.dart`, `.yaml`, `.json` |
| `rust` | Rust | `.rs`, `.toml` |
| `go` | Go | `.go`, `.mod`, `.sum` |
| `java` | Java | `.java`, `.xml`, `.gradle` |
| `csharp` | C# | `.cs`, `.csproj`, `.sln` |
| `config` | Konfiguration | `.json`, `.yaml`, `.toml`, `.env` |
| `docs` | Dokumentation | `.md`, `.rst`, `.txt` |

Alle Typen anzeigen:

```bash
code2md list-types
```

---

## Praxisbeispiele

### Mehrere Projekttypen kombinieren

Für ein Full-Stack-Projekt mit Vue.js-Frontend und Python-Backend:

```bash
code2md ./meine-app --type vue,python,config
```

### Eigene Dateiendungen hinzufügen

Zusätzlich `.env` und `.graphql` Dateien einschließen:

```bash
code2md ./projekt --type react --ext .env .graphql .prisma
```

### Ordner und Dateien ausschließen

Tests und Mock-Dateien ignorieren:

```bash
code2md ./app --type node --exclude tests/ mocks/ "*.test.js" "*.spec.ts"
```

### Eigenen Dateinamen und Projektnamen wählen

```bash
code2md ./src --type python --output dokumentation.md --name "Mein Projekt v2"
```

### Ausführliche Ausgabe

Zeigt alle gefundenen Dateien während der Verarbeitung:

```bash
code2md ./projekt --type python -v
```

---

## Alle Optionen

| Option | Kurzform | Beschreibung |
|--------|----------|--------------|
| `--type` | `-t` | Projekttyp(en), kommasepariert |
| `--ext` | `-e` | Zusätzliche Dateiendungen |
| `--exclude` | `-x` | Ordner/Dateien/Muster ausschließen |
| `--output` | `-o` | Name der Ausgabedatei |
| `--name` | `-n` | Projektname im Dokument |
| `--verbose` | `-v` | Ausführliche Ausgabe |
| `--no-tree` | | Ordnerstruktur nicht anzeigen |
| `--no-default-excludes` | | Standard-Ausschlüsse deaktivieren |

### Subcommands

| Command | Beschreibung |
|---------|--------------|
| `list-types` | Zeigt alle verfügbaren Projekttypen |

---

## Automatische Ausschlüsse

Folgende Ordner und Dateien werden standardmäßig ignoriert:

- **Abhängigkeiten:** `node_modules`, `vendor`, `venv`, `.venv`
- **Build-Ordner:** `dist`, `build`, `target`, `bin`, `obj`
- **Cache:** `__pycache__`, `.pytest_cache`, `.mypy_cache`
- **IDE-Dateien:** `.idea`, `.vscode`, `.vs`
- **Versionskontrolle:** `.git`, `.svn`
- **System:** `.DS_Store`, `Thumbs.db`
- **Logs:** `*.log`, `logs/`, `tmp/`

Mit `--no-default-excludes` können Sie diese Ausschlüsse deaktivieren.

---

## Projektstruktur

```
code2md/
├── Cargo.toml          # Projektdefinition & Dependencies
├── LICENSE             # MIT-Lizenz
├── README.md           # Diese Datei
└── src/
    ├── main.rs         # Entry Point
    ├── cli.rs          # CLI-Definitionen (clap)
    ├── types.rs        # Projekttypen & Syntax-Mapping
    ├── collector.rs    # Dateisammlung & Filterung
    ├── tree.rs         # Baumdarstellung
    └── markdown.rs     # Markdown-Generierung
```

---

## Entwicklung

### Build

```bash
# Debug-Build
cargo build

# Release-Build (optimiert)
cargo build --release

# Tests ausführen
cargo test

# Clippy Linting
cargo clippy
```

### Cross-Compilation

Mit [cross](https://github.com/cross-rs/cross) für andere Plattformen kompilieren:

```bash
# Für Windows
cross build --release --target x86_64-pc-windows-gnu

# Für Linux (musl, statisch gelinkt)
cross build --release --target x86_64-unknown-linux-musl

# Für macOS ARM
cross build --release --target aarch64-apple-darwin
```

---

## Häufige Fragen

### Kann ich nur bestimmte Dateien exportieren?

Ja, nutzen Sie `--ext` mit nur den gewünschten Endungen:

```bash
code2md ./projekt --ext .py .json
```

### Warum fehlen manche Dateien?

Prüfen Sie mit `-v` (verbose), welche Dateien gefunden werden. Möglicherweise liegt die Datei in einem automatisch ausgeschlossenen Ordner oder hat eine nicht erkannte Endung.

### Wie schnell ist code2md?

Bei einem Projekt mit 1.000 Dateien typischerweise unter 100ms. Die Rust-Implementierung ist etwa 10-50x schneller als vergleichbare Python-Tools.

---

## Mitwirken

Beiträge sind willkommen! Öffnen Sie gerne ein Issue oder Pull Request auf GitHub.

1. Repository forken
2. Feature-Branch erstellen (`git checkout -b feature/neue-funktion`)
3. Änderungen committen (`git commit -m 'Neue Funktion hinzugefügt'`)
4. Branch pushen (`git push origin feature/neue-funktion`)
5. Pull Request öffnen

---

## Lizenz

Dieses Projekt steht unter der **MIT-Lizenz**. Sie dürfen den Code frei verwenden, verändern und weitergeben – auch in kommerziellen Projekten.

Siehe [LICENSE](LICENSE) für Details.

---

## Autor

Entwickelt von [m0h1nd4](https://github.com/m0h1nd4)

---

<p align="center">
  <sub>⭐ Wenn Ihnen dieses Tool gefällt, freue ich mich über einen Stern auf GitHub!</sub>
</p>
