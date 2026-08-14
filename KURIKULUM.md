# Kurikulum Belajar Rust 🦀
> Dirancang untuk developer dengan background PHP / Node.js / Python

## Roadmap

```
basic/          → Fondasi (mulai di sini)
ownership/      → Konsep paling unik di Rust
enums/          → Enum, Option, Pattern Matching
collections/    → Vec, String, HashMap
error_handling/ → Result, ? operator, custom error
traits/         → Trait, Generics, Lifetime
async_rust/     → Async/Await, Tokio
mini_projects/  → Proyek kecil untuk praktek
```

---

## 1. Basic `/basic`
| File | Topik |
|------|-------|
| `01_hello_world.rs` | println!, print!, format! |
| `02_variables.rs` | let, mut, shadowing, const |
| `03_data_types.rs` | integer, float, bool, char, tuple, array |
| `04_functions.rs` | fn, parameter, return value |
| `05_control_flow.rs` | if/else, loop, while, for, range |
| `06_comments.rs` | //, ///, /* */ |

## 2. Ownership `/ownership`
| File | Topik |
|------|-------|
| `01_ownership.rs` | Move semantics, scope, drop |
| `02_borrowing.rs` | &reference, &mut reference |
| `03_slices.rs` | &str, slice array |

## 3. Enum & Pattern Matching `/enums`
| File | Topik |
|------|-------|
| `01_enums.rs` | Enum dasar, enum dengan data |
| `02_option.rs` | Option<T>, Some, None |
| `03_match.rs` | match, if let, while let |

## 4. Collections `/collections`
| File | Topik |
|------|-------|
| `01_vectors.rs` | Vec<T>, push, iter |
| `02_strings.rs` | String vs &str, manipulasi |
| `03_hashmaps.rs` | HashMap, entry API |

## 5. Error Handling `/error_handling`
| File | Topik |
|------|-------|
| `01_result.rs` | Result<T,E>, Ok, Err |
| `02_question_mark.rs` | ? operator, propagating error |
| `03_custom_error.rs` | Custom error type, thiserror |

## 6. Traits & Generics `/traits`
| File | Topik |
|------|-------|
| `01_traits.rs` | trait, impl trait, default method |
| `02_generics.rs` | Generic function, generic struct |
| `03_lifetimes.rs` | Lifetime annotation, 'a |

## 7. Async Rust `/async_rust`
| File | Topik |
|------|-------|
| `01_async_basic.rs` | async fn, .await, Future |
| `02_tokio.rs` | Tokio runtime, spawn, join! |
| `03_async_web.rs` | Axum basic HTTP server |

## 8. Mini Projects `/mini_projects`
| Folder | Deskripsi |
|--------|-----------|
| `cli_kalkulator/` | Kalkulator CLI — praktek basic + error handling |
| `csv_processor/` | Baca & proses CSV — relevan untuk e-Faktur! |
| `rest_api/` | REST API sederhana pakai Axum |
| `gempa_bmkg/` | Ambil data gempa real-time dari API publik BMKG |

---

## Tips Belajar
- Jalankan tiap file: `rustc namafile.rs && ./namafile`
- Atau pakai Cargo: `cargo new latihan && cd latihan`
- Kalau error di borrow checker — baca pesannya pelan-pelan, Rust compiler sangat helpful
- Referensi: https://doc.rust-lang.org/book/
