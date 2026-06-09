# 03 · Enums & Pattern Matching

> Enum di Rust jauh lebih powerful dari enum di Java/PHP — bisa bawa data berbeda tiap variant.
> Plus `Option<T>` sebagai pengganti `null` yang aman.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_enums.rs](./01_enums.rs) | Enum | enum dengan data, `match`, `if let` |
| [02_option.rs](./02_option.rs) | Option\<T\> | `Some`, `None`, `unwrap`, `map`, `?` |

---

## Enum — Bukan Sekadar Konstanta

Di PHP/Java, enum hanya bisa menyimpan nilai sederhana.
Di Rust, tiap **variant** bisa bawa data berbeda!

```rust
// PHP 8.1: enum Status { Draft; Sent; }  → tidak bisa bawa data

// Rust: tiap variant bisa bawa data berbeda
enum Pesan {
    Teks(String),
    Angka(i32),
    Koordinat { lat: f64, lng: f64 },
    Kosong,
}
```

### Contoh Domain Pajak
```rust
enum StatusSPT {
    BelumLapor,
    Lapor { tanggal: String },
    Terlambat { tanggal: String, hari: u32 },
    Pembetulan { ke: u32 },
}
```

---

## Option\<T\> — Pengganti Null

Di Rust **tidak ada `null`**. Diganti dengan `Option<T>` yang harus di-handle secara eksplisit.

```rust
// PHP:    $result = null;  → bisa lupa cek, NullPointerException!
// Rust:   compiler MEMAKSA kamu handle kemungkinan "tidak ada nilai"

fn cari_faktur(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("FKT-001"))
    } else {
        None
    }
}

// Wajib handle keduanya:
match cari_faktur(1) {
    Some(f) => println!("Ditemukan: {f}"),
    None    => println!("Tidak ada"),
}
```

### Cara Handle Option
```rust
// 1. match — paling explicit
match nilai { Some(v) => ..., None => ... }

// 2. if let — ringkas
if let Some(v) = nilai { ... }

// 3. unwrap_or — dengan default
nilai.unwrap_or(String::from("default"))

// 4. map — transform isi Some
nilai.map(|v| v.to_uppercase())

// 5. ? — early return None (di dalam function)
let v = nilai?;
```

---

## Pattern Matching

`match` di Rust **wajib exhaustive** — semua kemungkinan harus di-cover.

```rust
match status_code {
    200        => "OK",
    201        => "Created",
    400..=499  => "Client Error",   // range
    500 | 503  => "Server Error",   // multiple value
    _          => "Unknown",        // wildcard — wajib!
}
```

---

## Exercise

| File | Soal | Topik |
|------|------|-------|
| [ex01_enums.rs](./ex01_enums.rs) | 8 soal | Enum data, Option, match, if let |

```bash
rustc ex01_enums.rs && ./ex01_enums
```

---

## Setelah Selesai Modul Ini

Lanjut ke **[04_collections](../04_collections)** — Vec, String, HashMap! 🦀
