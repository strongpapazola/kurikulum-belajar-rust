# 01 · Basic

> Fondasi Rust — mulai dari sini sebelum lanjut ke modul berikutnya.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_hello_world.rs](./01_hello_world.rs) | Print & Output | `println!`, `print!`, `format!`, `eprintln!` |
| [02_variables.rs](./02_variables.rs) | Variables | `let`, `mut`, shadowing, `const` |
| [03_data_types.rs](./03_data_types.rs) | Tipe Data | integer, float, bool, char, tuple, array |
| [04_functions.rs](./04_functions.rs) | Functions | `fn`, closure, higher-order function |
| [05_control_flow.rs](./05_control_flow.rs) | Control Flow | `if`, `loop`, `while`, `for`, `match` |
| [06_structs.rs](./06_structs.rs) | Structs | `struct`, `impl`, method |

---

## Highlight Konsep

### Variables — Immutable by Default
Di Rust, variable **tidak bisa diubah** kecuali pakai `mut`. Ini beda dari PHP/JS/Python.

```rust
let x = 5;        // immutable — tidak bisa diubah
let mut y = 5;    // mutable   — bisa diubah
y = 10;           // ✅ OK

// Shadowing — bisa redeclare dengan tipe berbeda
let z = "42";
let z = z.parse::<i32>().unwrap(); // z sekarang integer
```

### Functions — Baris Terakhir = Return Value
```rust
fn tambah(a: i32, b: i32) -> i32 {
    a + b  // tidak ada titik koma = otomatis return
}
```

### Match — Switch Case yang Powerful
```rust
match status {
    200 => println!("OK"),
    404 => println!("Not Found"),
    500 => println!("Server Error"),
    _   => println!("Unknown"),  // wajib ada wildcard!
}
```

### Structs — Data + Behavior
```rust
struct Faktur {
    nomor: String,
    total: f64,
}

impl Faktur {
    fn ppn(&self) -> f64 {
        self.total * 0.11
    }
}
```

---

## Perbandingan dengan Bahasa Lain

```
PHP:    $x = 5;          echo "Hello $nama";
JS:     let x = 5;       console.log(`Hello ${nama}`)
Python: x = 5            print(f"Hello {nama}")
Rust:   let x = 5;       println!("Hello {nama}")
```

---

## Exercise

Selesaikan semua TODO di file exercise berikut:

| File | Soal | Topik |
|------|------|-------|
| [ex01_hello_world.rs](./ex01_hello_world.rs) | 7 soal | Print, format, padding |
| [ex02_variables.rs](./ex02_variables.rs) | 6 soal | let, mut, shadowing, const |
| [ex03_functions.rs](./ex03_functions.rs) | 6 soal | fn, closure, iterator |
| [ex04_control_flow.rs](./ex04_control_flow.rs) | 6 soal | if, match, loop, for |
| [ex05_structs.rs](./ex05_structs.rs) | 7 soal | struct, impl, PPh21 calculator |

```bash
# Cara mengerjakan exercise:
rustc ex01_hello_world.rs && ./ex01_hello_world

# Stuck? Lihat jawaban (hanya ex01):
rustc ex01_hello_world_jawaban.rs && ./ex01_hello_world_jawaban
```

---

## Setelah Selesai Modul Ini

Lanjut ke **[02_ownership](../02_ownership)** — konsep paling unik dan penting di Rust! 🦀
