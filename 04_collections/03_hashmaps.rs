// ============================================
// HASHMAP — Key-value store di Rust
// ============================================
// Jalankan: rustc 03_hashmaps.rs && ./03_hashmaps
// HashMap<K, V> mirip array associative PHP / dict Python / object JS

use std::collections::HashMap;

fn main() {
    // ==================
    // BUAT HASHMAP
    // ==================
    let mut kode_pajak: HashMap<&str, &str> = HashMap::new();
    kode_pajak.insert("PPN",   "Pajak Pertambahan Nilai");
    kode_pajak.insert("PPh21", "Pajak Penghasilan Pasal 21");
    kode_pajak.insert("PPh23", "Pajak Penghasilan Pasal 23");
    println!("{:?}", kode_pajak);

    // Dari dua vector
    let keys = vec!["a", "b", "c"];
    let vals = vec![1, 2, 3];
    let map: HashMap<&str, i32> = keys.into_iter().zip(vals).collect();
    println!("{:?}", map);

    // ==================
    // AKSES VALUE
    // ==================
    // get() — return Option<&V>
    match kode_pajak.get("PPN") {
        Some(nama) => println!("PPN = {nama}"),
        None       => println!("Tidak ditemukan"),
    }

    // dengan unwrap_or
    let nama = kode_pajak.get("PPnBM").unwrap_or(&"Tidak diketahui");
    println!("PPnBM = {nama}");

    // contains_key
    println!("Ada PPh21? {}", kode_pajak.contains_key("PPh21"));

    // ==================
    // UPDATE
    // ==================
    // Overwrite
    kode_pajak.insert("PPN", "PPN 11%");

    // entry API — insert kalau belum ada
    kode_pajak.entry("PPnBM").or_insert("Pajak Penjualan Barang Mewah");
    kode_pajak.entry("PPN").or_insert("Tidak akan masuk, sudah ada");
    println!("Setelah update: {:?}", kode_pajak.get("PPN"));
    println!("PPnBM: {:?}", kode_pajak.get("PPnBM"));

    // ==================
    // DELETE
    // ==================
    kode_pajak.remove("PPh23");
    println!("Setelah remove: {:?}", kode_pajak);

    // ==================
    // ITERASI
    // ==================
    println!("\n=== Semua Kode Pajak ===");
    for (kode, nama) in &kode_pajak {
        println!("{kode}: {nama}");
    }

    // ==================
    // CONTOH: Counter / Group By
    // ==================
    let jenis_faktur = vec!["PPN", "PPh21", "PPN", "PPnBM", "PPN", "PPh21"];

    let mut counter: HashMap<&str, u32> = HashMap::new();
    for jenis in &jenis_faktur {
        let count = counter.entry(jenis).or_insert(0);
        *count += 1;
    }
    println!("\n=== Counter Jenis Faktur ===");
    for (jenis, count) in &counter {
        println!("{jenis}: {count} faktur");
    }

    // ==================
    // CONTOH: Group faktur by status
    // ==================
    let data = vec![
        ("FKT-001", "lunas", 1_000_000.0_f64),
        ("FKT-002", "pending", 500_000.0),
        ("FKT-003", "lunas", 2_000_000.0),
        ("FKT-004", "pending", 750_000.0),
        ("FKT-005", "lunas", 3_000_000.0),
    ];

    let mut by_status: HashMap<&str, Vec<f64>> = HashMap::new();
    for (_, status, total) in &data {
        by_status.entry(status).or_insert_with(Vec::new).push(*total);
    }

    println!("\n=== Faktur by Status ===");
    for (status, totals) in &by_status {
        let sum: f64 = totals.iter().sum();
        println!("{status}: {} faktur, total Rp {:.0}", totals.len(), sum);
    }
}

// ============================================
// Perbandingan:
//
// PHP:    $map = ["key" => "val"]; $map["key"] = "new";
// Python: d = {"key": "val"}; d["key"] = "new"
// JS:     const m = {key: "val"}; m.key = "new"
//
// Rust:   let mut m = HashMap::new();
//         m.insert("key", "val");
//         m.get("key") → Option<&V>  ← perlu handle None!
// ============================================
