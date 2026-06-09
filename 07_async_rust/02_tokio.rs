// ============================================
// 02 - Tokio: Runtime Async Rust
// ============================================
// Butuh Cargo! Jalankan via:
//
//   cargo new belajar-tokio
//   cd belajar-tokio
//
// Tambah ke Cargo.toml:
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
//
// Copy kode ini ke src/main.rs, lalu:
//   cargo run

use std::time::{Duration, Instant};
use tokio::time::sleep;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

// ==================
// TASK & SPAWN
// ==================
async fn worker(id: u32, durasi_ms: u64) -> String {
    println!("Worker {id} mulai...");
    sleep(Duration::from_millis(durasi_ms)).await;
    println!("Worker {id} selesai ({durasi_ms}ms)");
    format!("Hasil-Worker-{id}")
}

// ==================
// CHANNEL — komunikasi antar task (seperti message queue mini)
// ==================
async fn demo_channel() {
    println!("\n=== Channel ===");
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Producer: spawn task yang kirim data
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for i in 1..=5 {
            let pesan = format!("Faktur-{:03}", i);
            tx_clone.send(pesan).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
        // tx_clone di-drop di sini, channel otomatis tertutup
    });
    drop(tx); // drop original tx supaya channel bisa tutup

    // Consumer: terima semua pesan
    while let Some(pesan) = rx.recv().await {
        println!("Terima: {pesan}");
    }
    println!("Channel selesai");
}

// ==================
// SHARED STATE — Mutex untuk shared data antar task
// ==================
async fn demo_mutex() {
    println!("\n=== Shared State dengan Mutex ===");

    // Arc = reference counting (bisa di-share antar thread)
    // Mutex = mutual exclusion (hanya 1 task bisa akses sekaligus)
    let counter = Arc::new(Mutex::new(0u32));

    let mut handles = vec![];

    for i in 1..=5 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut val = counter_clone.lock().await;
            *val += 1;
            println!("Task {i}: counter = {}", *val);
        });
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }

    println!("Final counter: {}", *counter.lock().await);
}

// ==================
// TIMEOUT — batasi waktu eksekusi
// ==================
async fn operasi_lambat() -> Result<String, &'static str> {
    sleep(Duration::from_millis(2000)).await; // simulasi 2 detik
    Ok("Selesai".to_string())
}

async fn demo_timeout() {
    println!("\n=== Timeout ===");

    match tokio::time::timeout(
        Duration::from_millis(500), // timeout 500ms
        operasi_lambat(),
    ).await {
        Ok(Ok(hasil)) => println!("Berhasil: {hasil}"),
        Ok(Err(e))    => println!("Error: {e}"),
        Err(_)        => println!("❌ Timeout! Operasi terlalu lambat"),
    }
}

// ==================
// SIMULASI: Kirim batch e-Faktur ke DJP
// ==================
async fn kirim_faktur(nomor: &str) -> Result<String, String> {
    let delay = if nomor.contains("003") { 300 } else { 100 };
    sleep(Duration::from_millis(delay)).await;

    if nomor.contains("005") {
        Err(format!("DJP reject {nomor}: format salah"))
    } else {
        Ok(format!("NSFP-{nomor}-2024"))
    }
}

async fn kirim_batch(fakturs: Vec<String>) {
    println!("\n=== Kirim Batch e-Faktur ===");
    let mulai = Instant::now();

    // Kirim semua concurrent
    let tasks: Vec<_> = fakturs.iter()
        .map(|f| kirim_faktur(f))
        .collect();

    let results = futures::future::join_all(tasks).await;

    let mut sukses = 0;
    let mut gagal  = 0;
    for (faktur, result) in fakturs.iter().zip(results) {
        match result {
            Ok(nsfp) => { println!("✅ {faktur} → {nsfp}"); sukses += 1; }
            Err(e)   => { println!("❌ {e}"); gagal += 1; }
        }
    }

    println!(
        "\nSelesai dalam {:.0}ms | Sukses: {} | Gagal: {}",
        mulai.elapsed().as_millis(), sukses, gagal
    );
}

#[tokio::main]
async fn main() {
    println!("=== Tokio Async Runtime ===\n");

    // 1. Spawn concurrent tasks
    println!("=== Concurrent Tasks ===");
    let mulai = Instant::now();

    let (r1, r2, r3) = tokio::join!(
        worker(1, 200),
        worker(2, 150),
        worker(3, 300),
    );
    println!("Hasil: {r1}, {r2}, {r3}");
    println!("Total waktu: {:.0}ms (bukan 650ms!)", mulai.elapsed().as_millis());

    // 2. Channel
    demo_channel().await;

    // 3. Shared state
    demo_mutex().await;

    // 4. Timeout
    demo_timeout().await;

    // 5. Batch e-Faktur
    let fakturs = (1..=6)
        .map(|i| format!("FKT-{:03}", i))
        .collect();
    kirim_batch(fakturs).await;
}

// ============================================
// Tokio features:
//
// tokio::spawn()            → buat task concurrent
// tokio::join!()            → tunggu beberapa task
// tokio::select!()          → tunggu yang pertama selesai
// tokio::time::sleep()      → async sleep
// tokio::time::timeout()    → batasi waktu
// tokio::sync::mpsc         → channel (multi-producer single-consumer)
// tokio::sync::Mutex        → mutex async-safe
// Arc<Mutex<T>>             → shared state antar task
// ============================================
