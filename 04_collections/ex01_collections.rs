// ============================================
// EXERCISE - Collections (Vec & HashMap)
// ============================================
// Jalankan: rustc ex01_collections.rs && ./ex01_collections

use std::collections::HashMap;

fn main() {
    // ===== VEC =====

    // TODO 1: Buat Vec berisi nilai ujian 10 siswa (bebas nilainya, 60-100)
    //         Hitung dan print: nilai tertinggi, terendah, dan rata-rata


    // TODO 2: Dari Vec nilai di atas:
    //         - filter yang lulus (>= 75)
    //         - urutkan dari tertinggi ke terendah
    //         - print hasilnya
    //         Expected: "Lulus (X siswa): [95, 90, ...]"


    // TODO 3: Buat Vec of tuples (nama_barang, qty, harga_satuan):
    //         [("Laptop", 2, 15_000_000.0),
    //          ("Monitor", 3, 4_500_000.0),
    //          ("Keyboard", 5, 350_000.0),
    //          ("Mouse", 5, 150_000.0),
    //          ("Headset", 2, 800_000.0)]
    //
    //         Hitung subtotal tiap barang (qty * harga)
    //         Print dalam format:
    //         "Laptop       x2  = Rp 30,000,000"
    //         Hitung grand total di akhir


    // TODO 4: Gunakan .windows(2) untuk cek apakah Vec sudah terurut
    //         Test dengan: [1, 3, 5, 7, 9]  → "Sudah terurut"
    //                      [1, 5, 3, 7, 9]  → "Belum terurut"


    // ===== HASHMAP =====

    // TODO 5: Buat HashMap<String, f64> untuk kurs mata uang:
    //         USD → 16_000.0, EUR → 17_500.0, SGD → 12_000.0, JPY → 105.0
    //         Konversi 100 USD, 50 EUR, 1000 JPY ke Rupiah dan print


    // TODO 6: Dari list transaksi berikut, hitung total per kategori:
    let transaksi = vec![
        ("Penjualan", 5_000_000.0_f64),
        ("Pembelian", 2_000_000.0),
        ("Penjualan", 3_500_000.0),
        ("Jasa",      1_200_000.0),
        ("Pembelian", 800_000.0),
        ("Jasa",      2_500_000.0),
        ("Penjualan", 7_000_000.0),
    ];
    // Buat HashMap<&str, f64> yang isi total per kategori
    // Print hasilnya diurutkan by kategori


    // TODO 7: Word frequency counter
    //         Hitung berapa kali tiap kata muncul:
    let teks = "pajak penghasilan pajak pertambahan nilai pajak penjualan pajak penghasilan";
    // Expected: {"pajak": 4, "penghasilan": 2, "pertambahan": 1, "nilai": 1, "penjualan": 1}
    // Print 3 kata yang paling sering muncul

}

// ============================================
// Expected output (sebagian):
// Tertinggi: XX, Terendah: XX, Rata-rata: XX.X
// Lulus (X siswa): [...]
// Laptop       x2  = Rp 30000000
// ...
// Grand Total: Rp XX
// 100 USD = Rp 1,600,000
// ...
// Penjualan: Rp 15,500,000
// Pembelian: Rp 2,800,000
// Jasa:      Rp 3,700,000
// Top 3 kata: pajak(4), penghasilan(2), ...
// ============================================
