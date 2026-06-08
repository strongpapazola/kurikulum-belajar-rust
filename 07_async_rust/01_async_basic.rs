// ============================================
// ASYNC / AWAIT di Rust
// ============================================
// Butuh dependency tokio di Cargo.toml:
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }
//
// Jalankan via Cargo:
//   cargo new async-latihan
//   cd async-latihan
//   (copy kode ini ke src/main.rs, tambah tokio ke Cargo.toml)
//   cargo run

use std::time::Duration;
use tokio::time::sleep;

// async function — mirip async/await di JS/Python
async fn ambil_data(id: u32) -> String {
    println!("Mulai ambil data {id}...");
    sleep(Duration::from_millis(100)).await; // simulasi network call
    format!("Data-{id}")
}

async fn kirim_ke_djp(faktur: &str) -> Result<String, String> {
    println!("Kirim {faktur} ke DJP...");
    sleep(Duration::from_millis(200)).await;

    // Simulasi kadang gagal
    if faktur.contains("999") {
        Err(format!("DJP reject: {faktur}"))
    } else {
        Ok(format!("NSFP-{faktur}"))
    }
}

#[tokio::main] // runtime macro dari tokio
async fn main() {
    println!("=== Async Basic ===\n");

    // ==================
    // AWAIT satu per satu (sequential)
    // ==================
    let d1 = ambil_data(1).await;
    let d2 = ambil_data(2).await;
    println!("Sequential: {d1}, {d2}");

    // ==================
    // CONCURRENT dengan join! (lebih cepat)
    // ==================
    let (d3, d4, d5) = tokio::join!(
        ambil_data(3),
        ambil_data(4),
        ambil_data(5),
    );
    println!("Concurrent: {d3}, {d4}, {d5}");

    // ==================
    // SPAWN — fire and forget task
    // ==================
    let handle = tokio::spawn(async {
        ambil_data(99).await
    });

    let hasil = handle.await.unwrap();
    println!("Spawn: {hasil}");

    // ==================
    // HANDLE ERROR async
    // ==================
    match kirim_ke_djp("FKT-001").await {
        Ok(nsfp)  => println!("\nBerhasil: {nsfp}"),
        Err(e)    => println!("\nGagal: {e}"),
    }

    match kirim_ke_djp("FKT-999").await {
        Ok(nsfp)  => println!("Berhasil: {nsfp}"),
        Err(e)    => println!("Gagal: {e}"),
    }

    // ==================
    // BULK ASYNC — relevan untuk kirim batch e-Faktur
    // ==================
    let fakturs = vec!["FKT-001", "FKT-002", "FKT-003", "FKT-004"];

    // Kirim semua concurrent
    let tasks: Vec<_> = fakturs.iter()
        .map(|f| kirim_ke_djp(f))
        .collect();

    let results = futures::future::join_all(tasks).await;
    // (butuh: futures = "0.3" di Cargo.toml)

    println!("\n=== Hasil Bulk ===");
    for (faktur, result) in fakturs.iter().zip(results) {
        match result {
            Ok(nsfp) => println!("✅ {faktur} → {nsfp}"),
            Err(e)   => println!("❌ {faktur} → {e}"),
        }
    }
}

// ============================================
// Perbandingan:
//
// JS:     async function fetch() { await something(); }
// Python: async def fetch(): await something()
// Rust:   async fn fetch() -> T { something().await }
//
// Key differences:
// - Rust async butuh runtime eksplisit (tokio, async-std)
// - .await di Rust ada di belakang expression (bukan depan)
// - Error handling tetap pakai Result, bukan throw/catch
// ============================================
