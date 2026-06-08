// ============================================
// EXERCISE 03 - Functions & Closures
// ============================================
// Jalankan: rustc ex03_functions.rs && ./ex03_functions

fn main() {
    // TODO 1: Panggil function celsius_ke_fahrenheit(100.0)
    //         Expected: "100°C = 212°F"


    // TODO 2: Panggil function hitung_ppn(5_000_000.0)
    //         Expected: "DPP: 5000000, PPN: 550000, Total: 5550000"


    // TODO 3: Panggil function adalah_npwp_valid("12.345.678.9-012.345")
    //         dan ("123") lalu print hasilnya
    //         Expected: "NPWP valid: true" dan "NPWP valid: false"


    // TODO 4: Buat CLOSURE yang menerima dua angka f64
    //         dan return rata-ratanya, lalu panggil dengan (80.0, 90.0)
    //         Expected: "Rata-rata: 85"


    // TODO 5: Gunakan closure + iterator untuk:
    //         - filter angka yang habis dibagi 3 dari vec [1..=20]
    //         - kalikan masing-masing dengan 10
    //         - print hasilnya
    //         Expected: [30, 60, 90, 120, 150, 180]
    let angka: Vec<i32> = (1..=20).collect();


    // TODO 6: Panggil function fibonacci(10)
    //         Expected: "Fibonacci ke-10: 55"

}

// TODO: Implementasi semua function di bawah ini!

// Konversi celsius ke fahrenheit: F = (C * 9/5) + 32
fn celsius_ke_fahrenheit(c: f64) -> f64 {
    todo!() // ganti dengan implementasi
}

// Hitung PPN 11%, return tuple (dpp, ppn, total)
fn hitung_ppn(dpp: f64) -> (f64, f64, f64) {
    todo!()
}

// Validasi NPWP: harus 15 digit angka
fn adalah_npwp_valid(npwp: &str) -> bool {
    todo!()
}

// Fibonacci rekursif
fn fibonacci(n: u64) -> u64 {
    todo!()
}

// ============================================
// Expected output:
// 100°C = 212°F
// DPP: 5000000, PPN: 550000, Total: 5550000
// NPWP valid: true
// NPWP valid: false
// Rata-rata: 85
// [30, 60, 90, 120, 150, 180]
// Fibonacci ke-10: 55
// ============================================
