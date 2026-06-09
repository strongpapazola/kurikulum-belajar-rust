# 07 · Async Rust

> Async/await di Rust — syntax mirip JavaScript, tapi lebih efisien dan tanpa runtime overhead.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_async_basic.rs](./01_async_basic.rs) | Async dasar | `async fn`, `.await`, `tokio`, `join!`, `spawn` |

---

## Setup — Butuh Cargo

Async Rust butuh runtime eksternal. Yang paling populer: **Tokio**.

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```bash
cargo new belajar-async
cd belajar-async
# tambah tokio ke Cargo.toml
cargo run
```

---

## Konsep Utama

### async fn + .await
```rust
// JS:   async function ambilData() { await fetch(...) }
// Rust:
async fn ambil_data(id: u32) -> String {
    // simulasi network call
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Data-{id}")
}

#[tokio::main]
async fn main() {
    let data = ambil_data(1).await;
    println!("{data}");
}
```

### Sequential vs Concurrent
```rust
// Sequential — lambat (total 300ms)
let d1 = ambil_data(1).await;   // tunggu 100ms
let d2 = ambil_data(2).await;   // tunggu 100ms lagi
let d3 = ambil_data(3).await;   // tunggu 100ms lagi

// Concurrent dengan join! — cepat (total ~100ms)
let (d1, d2, d3) = tokio::join!(
    ambil_data(1),
    ambil_data(2),
    ambil_data(3),
);
```

### Bulk Async — Relevan untuk Kirim Batch e-Faktur
```rust
let fakturs = vec!["FKT-001", "FKT-002", "FKT-003"];

// Kirim semua concurrent
let tasks: Vec<_> = fakturs.iter()
    .map(|f| kirim_ke_djp(f))
    .collect();

let results = futures::future::join_all(tasks).await;
```

---

## Perbandingan dengan JS

```javascript
// JavaScript
async function kirim(faktur) {
    try {
        const res = await fetch(url, { body: faktur });
        return await res.json();
    } catch (e) {
        console.error(e);
    }
}
```

```rust
// Rust
async fn kirim(faktur: &str) -> Result<String, reqwest::Error> {
    let res = client.post(url).body(faktur).send().await?;
    let json = res.text().await?;
    Ok(json)
}
```

Perbedaan utama:
- Rust butuh runtime eksplisit (`tokio`)
- `.await` ada di belakang expression (bukan depan)
- Error handling tetap pakai `Result`, bukan `try/catch`

---

## Setelah Selesai Modul Ini

Lanjut ke **[08_mini_projects](../08_mini_projects)** — terapkan semua ilmu! 🦀
