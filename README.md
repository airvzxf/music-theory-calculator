# Music theory: The calculator | `tonic-music`.

[![CI](https://github.com/airvzxf/music-theory-calculator/actions/workflows/ci.yml/badge.svg)](https://github.com/airvzxf/music-theory-calculator/actions/workflows/ci.yml)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

> A command-line music theory calculator for generating scales, chords, and full scale harmonization. Built with Rust.

`tonic-music` is a simple, fast CLI tool for musicians and developers who need to quickly calculate music theory constructs from their terminal.

## 🚀 Installation

### From Source
Ensure you have Rust and Cargo installed.

1.  Clone the repository:
    ```bash
    git clone [https://github.com/airvzxf/music-theory-calculator.git](https://github.com/airvzxf/music-theory-calculator.git)
    ```
2.  Navigate to the project directory:
    ```bash
    cd music-theory-calculator/src/tonic-music
    ```
3.  Build and install the binary:
    ```bash
    cargo install --path .
    ```
4.  You can now run `tonic-music` from anywhere.

## ⚙️ Usage

`tonic-music` provides three main commands: `scale`, `chord`, and `harmonize`.

### `scale`
Generates the notes of a given scale. Now supports `major`, `minor`, `harmonic` (minor), and pentatonic (`penta-major`, `penta-minor`) scales.

**Command:**
```bash
tonic-music scale --root A --scale-type penta-minor
```

**Output:**

```text
--- A penta-minor Scale ---
[A, C, D, E, G]
```

### `chord`

Generates the notes of a given chord. Now supports triads, a wide variety of **seventh chords** (e.g., `maj7`, `m7`, `7`), and can display **inversions**.

**Command:**
```bash
tonic-music chord -r G -c 7 --inversions
```

**Output:**

```text
--- G 7 Chord ---
Root:   [G, B, D, F]
1st Inv:    [B, D, F, G]
2nd Inv:    [D, F, G, B]
3rd Inv:    [F, G, B, D]
```

### `harmonize`

Generates the full set of diatonic chords for a given scale. Can now generate **seventh chords** instead of triads.

**Command:**
```bash
tonic-music harmonize -r C -s major --sevenths
```

**Output:**

```text
--- C major Harmonization ---
I (C):       C maj7  -> [C, E, G, B]
II (D):      D m7    -> [D, F, A, C]
III (E):     E m7    -> [E, G, B, D]
IV (F):      F maj7  -> [F, A, C, E]
V (G):       G 7     -> [G, B, D, F]
VI (A):      A m7    -> [A, C, E, G]
VII (B):     B m7b5  -> [B, D, F, A]
```

## 🤝 Contributing

Contributions are welcome\! Whether it's reporting a bug, suggesting a feature, or writing code, all help is appreciated.

Please read our [CONTRIBUTING.md](https://www.google.com/search?q=CONTRIBUTING.md) for guidelines.

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPLv3)**. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.
