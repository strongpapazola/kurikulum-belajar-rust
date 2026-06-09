// ============================================
// EXERCISE - Traits & Generics
// ============================================
// Jalankan: rustc ex01_traits.rs && ./ex01_traits

use std::fmt;

// TODO 1: Buat trait "Deskripsi" dengan:
//         - fn id(&self) -> String         (wajib impl)
//         - fn kategori(&self) -> &str     (wajib impl)
//         - fn ringkasan(&self) -> String  (default: gabungan id + kategori)


// TODO 2: Buat struct "Faktur" dan "WajibPajak", implementasikan trait Deskripsi:
//
// struct Faktur {
//     nomor: String, total: f64, nama_pembeli: String
// }
// → id()       = nomor
// → kategori() = "Dokumen Pajak"
// → ringkasan() = "[FKT-001] Dokumen Pajak - PT Maju Jaya - Rp 1500000"
//
// struct WajibPajak {
//     npwp: String, nama: String, jenis: String  // "OP" atau "Badan"
// }
// → id()       = npwp
// → kategori() = "Wajib Pajak Orang Pribadi" atau "Wajib Pajak Badan"
// → ringkasan() pakai default dari trait


// TODO 3: Buat trait "Kalkulasi" dengan:
//         - fn hitung(&self) -> f64
//         - fn label(&self) -> &str
//         Lalu buat struct PPN, PPh21, PPh23 yang impl Kalkulasi
//         dengan tarif masing-masing


// TODO 4: Buat generic function:
//         fn tampilkan_semua<T: Deskripsi>(items: &[T])
//         yang print ringkasan() tiap item


// TODO 5: Buat generic function:
//         fn total_pajak<T: Kalkulasi>(pajak_list: &[T], dpp: f64) -> f64
//         yang return jumlah semua pajak dari list


// TODO 6: Buat struct generic:
//         struct Cache<T> { data: Vec<(String, T)> }
//         dengan method:
//         - fn simpan(&mut self, key: &str, val: T)
//         - fn ambil(&self, key: &str) -> Option<&T>
//         - fn hapus(&mut self, key: &str) -> bool
//         Test dengan Cache<f64> untuk cache hasil kalkulasi pajak


// TODO 7: Implement fmt::Display untuk Faktur
//         format: "Faktur FKT-001 | PT Maju Jaya | Rp 1,500,000"


fn main() {
    // TODO 8: Test semua yang sudah dibuat:

    // Buat beberapa Faktur dan WajibPajak
    // Panggil tampilkan_semua()

    // Buat vec pajak (PPN, PPh21, PPh23)
    // Panggil total_pajak()

    // Pakai Cache<f64> untuk simpan hasil hitung PPN beberapa transaksi

    // Print Faktur dengan Display
}

// ============================================
// Expected output (kira-kira):
// === Faktur ===
// [FKT-001] Dokumen Pajak - PT Maju Jaya - Rp 1500000
// [FKT-002] Dokumen Pajak - CV Sejahtera - Rp 800000
//
// === Wajib Pajak ===
// [12.345.678.9-012.345] Wajib Pajak Badan
// [98.765.432.1-098.765] Wajib Pajak Orang Pribadi
//
// Total PPN + PPh23 untuk DPP 10.000.000: Rp 1.300.000
//
// Cache hit: Some(1100000.0)
// Cache miss: None
// ============================================
