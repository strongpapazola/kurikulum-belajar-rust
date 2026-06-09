# 05 · Error Handling

> Tidak ada `try/catch` di Rust. Error adalah nilai biasa yang harus di-handle secara eksplisit.
> Ini bikin kode lebih predictable dan tidak ada "kejutan" runtime error.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_result.rs](./01_result.rs) | Result\<T,E\> | `Ok`, `Err`, `?`, `map`, `unwrap` |

---

## Result\<T, E\> — Pengganti Try/Catch

```rust
// PHP/JS/Python:
// try {
//     $result = riskyOperation();
// } catch (Exception $e) {
//     echo $e->getMessage();
// }

// Rust: error adalah nilai return biasa
fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Tidak bisa bagi dengan nol"))
    } else {
        Ok(a / b)
    }
}

match bagi(10.0, 0.0) {
    Ok(hasil) => println!("Hasil: {hasil}"),
    Err(e)    => println!("Error: {e}"),
}
```

---

## Operator ? — Propagate Error dengan Ringkas

Tanpa `?`:
```rust
fn proses() -> Result<String, String> {
    let a = langkah_1();
    let a = match a {
        Ok(v)  => v,
        Err(e) => return Err(e),
    };
    let b = langkah_2(a);
    let b = match b {
        Ok(v)  => v,
        Err(e) => return Err(e),
    };
    Ok(b)
}
```

Dengan `?`:
```rust
fn proses() -> Result<String, String> {
    let a = langkah_1()?;   // kalau Err → langsung return Err
    let b = langkah_2(a)?;  // sama
    Ok(b)
}
```

---

## Custom Error Type

```rust
#[derive(Debug)]
enum FakturError {
    NpwpTidakValid(String),
    TotalNegatif(f64),
    DjpGagal { kode: u32, pesan: String },
}

impl std::fmt::Display for FakturError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FakturError::NpwpTidakValid(s) => write!(f, "NPWP tidak valid: {s}"),
            FakturError::TotalNegatif(v)   => write!(f, "Total negatif: {v}"),
            FakturError::DjpGagal { kode, pesan } =>
                write!(f, "DJP error {kode}: {pesan}"),
        }
    }
}
```

---

## Cara Handle Result

```rust
// 1. match — paling explicit
match hasil { Ok(v) => ..., Err(e) => ... }

// 2. unwrap() — PANIC kalau Err (hindari di production!)
let v = hasil.unwrap();

// 3. unwrap_or() — default kalau Err
let v = hasil.unwrap_or(0.0);

// 4. unwrap_or_else() — closure kalau Err
let v = hasil.unwrap_or_else(|e| { log(e); 0.0 });

// 5. map() — transform Ok value
let v = hasil.map(|n| n * 2);

// 6. ? — propagate error ke caller (paling idiomatis)
let v = hasil?;
```

---

## Perbandingan

```
PHP:    throw new Exception("msg")   → catch bisa di mana saja, mudah lupa
JS:     throw new Error("msg")       → sama, uncaught crash
Python: raise Exception("msg")       → sama

Rust:   return Err(FakturError::X)   → caller WAJIB handle
        → compiler error kalau tidak di-handle
        → tidak ada runtime surprise ✅
```

---

## Exercise

| File | Soal | Topik |
|------|------|-------|
| [ex01_error_handling.rs](./ex01_error_handling.rs) | 8 soal | Result, custom error, ?, collect |

```bash
rustc ex01_error_handling.rs && ./ex01_error_handling
```

---

## Setelah Selesai Modul Ini

Lanjut ke **[06_traits](../06_traits)** — Traits & Generics! 🦀
