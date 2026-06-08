// ============================================
// EXERCISE - Error Handling
// ============================================
// Jalankan: rustc ex01_error_handling.rs && ./ex01_error_handling

use std::num::ParseFloatError;

// TODO 1: Buat custom error enum "FakturError" dengan variant:
//         - NpwpTidakValid(String)
//         - TotalNegative(f64)
//         - NomorTidakValid(String)
//         - DjpGagal { kode: u32, pesan: String }
//
//         Implementasikan std::fmt::Display untuk FakturError
//         supaya bisa di-print dengan {}


// TODO 2: Buat function dengan signature ini:
//         fn buat_faktur(nomor: &str, npwp: &str, total: f64)
//             -> Result<String, FakturError>
//
//         Validasi:
//         - nomor harus mulai "FKT-" → NomorTidakValid
//         - npwp harus 15 digit → NpwpTidakValid
//         - total harus > 0 → TotalNegative
//         Kalau semua OK → Ok("Faktur {nomor} berhasil dibuat")


// TODO 3: Buat function:
//         fn parse_total(input: &str) -> Result<f64, ParseFloatError>
//         yang parse string ke f64 (gunakan .parse())


// TODO 4: Buat function yang chain beberapa Result dengan ?:
//         fn proses_input(nomor: &str, npwp: &str, total_str: &str)
//             -> Result<String, String>
//
//         Di dalamnya:
//         1. parse total_str ke f64 (map_err ke String)
//         2. panggil buat_faktur (map_err ke String)
//         3. return hasilnya


fn main() {
    // TODO 5: Test buat_faktur dengan berbagai skenario:
    let test_cases = vec![
        ("FKT-001", "12.345.678.9-012.345", 1_000_000.0),  // valid
        ("INV-001", "12.345.678.9-012.345", 1_000_000.0),  // nomor salah
        ("FKT-002", "123-INVALID",          1_000_000.0),  // npwp salah
        ("FKT-003", "12.345.678.9-012.345", -500.0),       // total negatif
    ];

    for (nomor, npwp, total) in test_cases {
        match buat_faktur(nomor, npwp, total) {
            Ok(pesan)  => println!("✅ {pesan}"),
            Err(e)     => println!("❌ Error: {e}"),
        }
    }

    // TODO 6: Test proses_input:
    println!("\n--- Proses Input ---");
    // valid
    // println!("{:?}", proses_input("FKT-001", "12.345.678.9-012.345", "1500000"));
    // total bukan angka
    // println!("{:?}", proses_input("FKT-001", "12.345.678.9-012.345", "abc"));


    // TODO 7: Gunakan unwrap_or_else untuk handle error dengan default value:
    //         Coba parse "bukan_angka" ke f64, kalau gagal return 0.0
    let nilai: f64 = "bukan_angka".parse().unwrap_or(0.0);
    println!("\nParse gagal default: {nilai}");


    // TODO 8: Collect Vec<Result> — pisahkan sukses dan gagal:
    let inputs = vec!["1000", "abc", "2500", "xyz", "5000"];
    // Parse semua, pisahkan yang Ok dan yang Err
    // Print: "Berhasil: [1000, 2500, 5000]"
    //        "Gagal: 2 item"

}

// ============================================
// Expected output:
// ✅ Faktur FKT-001 berhasil dibuat
// ❌ Error: Nomor tidak valid: INV-001 (harus mulai dengan FKT-)
// ❌ Error: NPWP tidak valid: 123-INVALID (harus 15 digit)
// ❌ Error: Total tidak boleh negatif: -500
// ...
// Berhasil: [1000.0, 2500.0, 5000.0]
// Gagal: 2 item
// ============================================
