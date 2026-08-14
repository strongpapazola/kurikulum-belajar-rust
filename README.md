# 🦀 Kurikulum Belajar Rust

> Kurikulum belajar Rust yang dirancang untuk developer dengan background **PHP / Node.js / Python**.
> Setiap materi disertai perbandingan dengan bahasa yang sudah kamu kenal, dan exercise bertema dunia nyata.

![Rust](https://img.shields.io/badge/Rust-1.96+-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Level](https://img.shields.io/badge/level-Beginner%20→%20Intermediate-green?style=flat-square)

---

## Kenapa Rust?

| | Rust | Go | Java | Node.js |
|---|---|---|---|---|
| **Performa** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Memory** | ~2–5 MB | ~10–20 MB | ~100–200 MB | ~30–50 MB |
| **Safety** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Learning Curve** | Tinggi | Rendah | Sedang | Rendah |

Rust dipakai oleh: **Discord, Cloudflare, AWS, Microsoft, Meta, Mozilla**

---

## Struktur Kurikulum

```
belajar-rust/
├── 01_basic/           → Fondasi Rust (mulai di sini!)
├── 02_ownership/       → Konsep paling unik Rust
├── 03_enums/           → Enum, Option, Pattern Matching
├── 04_collections/     → Vec, String, HashMap
├── 05_error_handling/  → Result, Error Handling
├── 06_traits/          → Traits & Generics
├── 07_async_rust/      → Async/Await & Tokio
└── 08_mini_projects/   → Proyek nyata untuk praktek
```

---

## Roadmap Belajar

```
Week 1    ████████░░░░░░░░  01_basic
Week 2    ████████░░░░░░░░  02_ownership  ← paling penting!
Week 3    ████████░░░░░░░░  03_enums + 04_collections
Week 4    ████████░░░░░░░░  05_error_handling + 06_traits
Week 5+   ████████░░░░░░░░  07_async_rust + 08_mini_projects
```

---

## Cara Mulai

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version  # rustc 1.96.0
```

### 2. Jalankan file materi
```bash
cd 01_basic
rustc 01_hello_world.rs && ./01_hello_world
```

### 3. Kerjakan exercise
```bash
# Buka file exercise, selesaikan semua TODO
rustc ex01_hello_world.rs && ./ex01_hello_world
```

---

## Modul

| # | Folder | Topik | File Materi | Exercise |
|---|--------|-------|-------------|----------|
| 1 | [01_basic](./01_basic) | Hello World, Variables, Functions, Structs | 6 file | 5 exercise |
| 2 | [02_ownership](./02_ownership) | Ownership, Borrowing, Slices | 2 file | 1 exercise |
| 3 | [03_enums](./03_enums) | Enum, Option\<T\>, Pattern Matching | 2 file | 1 exercise |
| 4 | [04_collections](./04_collections) | Vec, String, HashMap | 2 file | 1 exercise |
| 5 | [05_error_handling](./05_error_handling) | Result\<T,E\>, Custom Error, ? | 1 file | 1 exercise |
| 6 | [06_traits](./06_traits) | Traits, Generics, Lifetimes | coming soon | - |
| 7 | [07_async_rust](./07_async_rust) | Async/Await, Tokio | 1 file | - |
| 8 | [08_mini_projects](./08_mini_projects) | CSV Processor, CLI Kalkulator, REST API, Monitor Gempa BMKG | 4 project | - |

---

## Tips

- **Baca error compiler pelan-pelan** — Rust compiler sangat helpful dan informatif
- **Jangan skip ownership** — ini fondasi segalanya di Rust
- **Referensi utama:** [The Rust Book](https://doc.rust-lang.org/book/) (gratis, resmi)
- **Latihan interaktif:** [Rustlings](https://github.com/rust-lang/rustlings)

---

*Dibuat untuk developer Indonesia yang ingin belajar Rust dari nol* 🇮🇩
