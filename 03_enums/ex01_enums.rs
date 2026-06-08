// ============================================
// EXERCISE - Enums & Option
// ============================================
// Jalankan: rustc ex01_enums.rs && ./ex01_enums

// TODO 1: Buat enum "StatusSPT" dengan variant:
//         - BelumLapor
//         - Lapor { tanggal: String }
//         - Terlambat { tanggal: String, hari_terlambat: u32 }
//         - Pembetulan { ke: u32 }
//         Tambahkan #[derive(Debug)]


// TODO 2: Implement method untuk StatusSPT:
//         - fn deskripsi(&self) -> String
//           BelumLapor       → "Belum melaporkan SPT"
//           Lapor            → "Lapor pada {tanggal}"
//           Terlambat        → "Terlambat {hari} hari (lapor: {tanggal})"
//           Pembetulan       → "Pembetulan ke-{ke}"
//         - fn kena_denda(&self) -> bool
//           (hanya Terlambat yang kena denda)


// TODO 3: Buat enum "HasilValidasi" dengan variant:
//         - Valid(String)          → data valid, bawa pesan
//         - TidakValid(Vec<String>) → bawa list error


// TODO 4: Buat function "validasi_faktur" yang menerima
//         (nomor: &str, npwp: &str, total: f64) → HasilValidasi
//         Rules:
//         - nomor harus mulai dengan "FKT-"
//         - npwp harus 15 digit
//         - total harus > 0
//         Kalau semua valid → Valid("Faktur OK")
//         Kalau ada error → TidakValid(vec![semua pesan error])


fn main() {
    // TODO 5: Test StatusSPT — buat masing-masing variant
    //         dan print deskripsinya


    // TODO 6: Test validasi_faktur dengan data:
    //         a) nomor="FKT-001", npwp="12.345.678.9-012.345", total=1_000_000.0
    //         b) nomor="INV-001", npwp="123", total=-500.0
    //         Print hasilnya dengan match


    // TODO 7: Gunakan if let untuk filter hanya StatusSPT::Terlambat
    //         dari list berikut dan print yang kena denda:
    let statuses = vec![
        // isi dengan beberapa StatusSPT variant
    ];


    // TODO 8: Gunakan Option untuk simulasi cari wajib pajak
    //         fn cari_wp(npwp: &str) -> Option<String>
    //         yang return Some(nama) untuk NPWP tertentu, None untuk lainnya
    //         Panggil dengan beberapa NPWP, handle dengan unwrap_or
}

// ============================================
// Expected output (kira-kira):
// BelumLapor: Belum melaporkan SPT, kena denda: false
// Lapor: Lapor pada 2024-03-31, kena denda: false
// Terlambat: Terlambat 5 hari (lapor: 2024-04-05), kena denda: true
// Pembetulan: Pembetulan ke-1, kena denda: false
//
// Validasi FKT-001: Valid - Faktur OK
// Validasi INV-001: Tidak valid - ["Nomor harus mulai FKT-", "NPWP harus 15 digit", "Total harus > 0"]
// ============================================
