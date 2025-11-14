# Music theory: The calculator | `tonic-music`.

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
Generates the notes of a given scale.

**Command:**
```bash
tonic-music scale --root F# --scale-type major
```

**Output:**

```text
--- F# major Scale ---
[FSharp, GSharp, ASharp, B, CSharp, DSharp, F]
```

### `chord`

Generates the notes of a given chord.
(Supports short-form flags `-r` for root and `-c` for chord-type).

**Command:**

```bash
tonic-music chord -r Bb -c min
```

**Output:**

```text
--- Bb min Chord ---
[ASharp, CSharp, F]
```

### `harmonize`

Generates the full set of diatonic triads for a given scale.
(Supports short-form flags `-r` for root and `-s` for scale-type).

**Command:**

```bash
tonic-music harmonize -r C -s harmonic
```

**Output:**

```text
--- C harmonic Harmonization ---
I (C):       C m     -> [C, DSharp, G]
II (D):      D °     -> [D, F, GSharp]
III (DSharp):DSharp + -> [DSharp, G, B]
IV (F):      F m     -> [F, GSharp, C]
V (G):       G       -> [G, B, D]
VI (GSharp): GSharp  -> [GSharp, C, DSharp]
VII (B):     B °     -> [B, D, F]
```

## 🤝 Contributing

Contributions are welcome\! Whether it's reporting a bug, suggesting a feature, or writing code, all help is appreciated.

Please read our [CONTRIBUTING.md](https://www.google.com/search?q=CONTRIBUTING.md) for guidelines.

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPLv3)**. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.
