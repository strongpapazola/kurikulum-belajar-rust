// ============================================
// 03 - Custom Error Types
// ============================================
// Jalankan: rustc 03_custom_error.rs && ./03_custom_error
//
// Bikin error type sendiri supaya:
// - Pesan error lebih deskriptif
// - Bisa pattern match tipe error spesifik
// - Kode lebih maintainable

use std::fmt;

// ==================
// CUSTOM ERROR — enum dengan variant per jenis error
// ==================
#[derive(Debug)]
enum FakturError {
    NpwpTidakValid { npwp: String, alasan: String },
    TotalTidakValid { total: f64 },
    NomorTidakValid(String),
    DjpGagal { kode: u16, pesan: String },
    ParseError(String),
}

// Implement Display supaya bisa di-print dengan {}
impl fmt::Display for FakturError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FakturError::NpwpTidakValid { npwp, alasan } =>
                write!(f, "NPWP tidak valid '{npwp}': {alasan}"),

            FakturError::TotalTidakValid { total } =>
                write!(f, "Total tidak valid: {total} (harus > 0)"),

            FakturError::NomorTidakValid(nomor) =>
                write!(f, "Nomor faktur tidak valid: '{nomor}' (harus dimulai FKT-)"),

            FakturError::DjpGagal { kode, pesan } =>
                write!(f, "DJP error [{kode}]: {pesan}"),

            FakturError::ParseError(msg) =>
                write!(f, "Parse error: {msg}"),
        }
    }
}

// Implement From untuk konversi error lain → FakturError
impl From<std::num::ParseFloatError> for FakturError {
    fn from(e: std::num::ParseFloatError) -> Self {
        FakturError::ParseError(e.to_string())
    }
}

// ==================
// FUNCTIONS yang return FakturError
// ==================
fn validasi_npwp(npwp: &str) -> Result<String, FakturError> {
    let digit: String = npwp.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    if digit.len() != 15 {
        return Err(FakturError::NpwpTidakValid {
            npwp: npwp.to_string(),
            alasan: format!("hanya {} digit, harus 15", digit.len()),
        });
    }
    Ok(digit)
}

fn validasi_nomor(nomor: &str) -> Result<(), FakturError> {
    if !nomor.starts_with("FKT-") {
        return Err(FakturError::NomorTidakValid(nomor.to_string()));
    }
    Ok(())
}

fn validasi_total(total: f64) -> Result<(), FakturError> {
    if total <= 0.0 {
        return Err(FakturError::TotalTidakValid { total });
    }
    Ok(())
}

fn parse_total(s: &str) -> Result<f64, FakturError> {
    let total: f64 = s.trim().parse()?; // ParseFloatError → FakturError via From
    Ok(total)
}

// Pipeline validasi menggunakan ?
fn buat_faktur(
    nomor: &str,
    npwp: &str,
    total_str: &str,
) -> Result<String, FakturError> {
    validasi_nomor(nomor)?;
    validasi_npwp(npwp)?;
    let total = parse_total(total_str)?;
    validasi_total(total)?;

    let ppn   = total * 0.11;
    let bayar = total + ppn;

    Ok(format!(
        "✅ Faktur {nomor} | DPP: {:.0} | PPN: {:.0} | Total: {:.0}",
        total, ppn, bayar
    ))
}

// ==================
// HANDLE ERROR PER JENIS
// ==================
fn proses_dengan_recovery(nomor: &str, npwp: &str, total_str: &str) {
    match buat_faktur(nomor, npwp, total_str) {
        Ok(hasil) => println!("{hasil}"),
        Err(FakturError::NpwpTidakValid { npwp, alasan }) => {
            println!("❌ NPWP bermasalah ({alasan}) → coba format ulang: {npwp}");
        }
        Err(FakturError::DjpGagal { kode, .. }) if kode == 503 => {
            println!("❌ DJP sedang maintenance, coba lagi nanti");
        }
        Err(e) => {
            println!("❌ Error: {e}");
        }
    }
}

fn main() {
    println!("=== Custom Error Types ===\n");

    // Test berbagai skenario
    let test_cases = vec![
        ("FKT-001", "12.345.678.9-012.345", "1500000"),   // valid
        ("INV-001", "12.345.678.9-012.345", "1500000"),   // nomor salah
        ("FKT-002", "123-INVALID",           "1500000"),   // npwp salah
        ("FKT-003", "12.345.678.9-012.345", "-500"),       // total negatif
        ("FKT-004", "12.345.678.9-012.345", "bukan_angka"),// parse error
    ];

    for (nomor, npwp, total) in &test_cases {
        match buat_faktur(nomor, npwp, total) {
            Ok(hasil) => println!("{hasil}"),
            Err(e)    => println!("❌ {e}"),
        }
    }

    println!("\n=== Handle Error per Jenis ===");
    for (nomor, npwp, total) in &test_cases {
        proses_dengan_recovery(nomor, npwp, total);
    }

    println!("\n=== Debug vs Display ===");
    let err = FakturError::DjpGagal {
        kode: 503,
        pesan: "Service Unavailable".to_string(),
    };
    println!("Display : {err}");
    println!("Debug   : {err:?}");

    // ==================
    // COLLECT errors — kumpulkan semua error sekaligus
    // ==================
    println!("\n=== Kumpulkan Semua Error ===");
    fn validasi_semua(nomor: &str, npwp: &str, total_str: &str) -> Vec<FakturError> {
        let mut errors = Vec::new();
        if let Err(e) = validasi_nomor(nomor)     { errors.push(e); }
        if let Err(e) = validasi_npwp(npwp)       { errors.push(e); }
        if let Ok(t) = parse_total(total_str) {
            if let Err(e) = validasi_total(t)     { errors.push(e); }
        } else if let Err(e) = parse_total(total_str) {
            errors.push(e);
        }
        errors
    }

    let errors = validasi_semua("INV-001", "123", "-500");
    if errors.is_empty() {
        println!("Semua valid!");
    } else {
        println!("Ditemukan {} error:", errors.len());
        for e in &errors {
            println!("  - {e}");
        }
    }
}

// ============================================
// Best Practices:
// 1. Satu error enum per domain/modul
// 2. Selalu impl Display (untuk end-user) + Debug (untuk developer)
// 3. Impl From untuk konversi error library eksternal
// 4. Gunakan thiserror crate di project nyata (lebih ringkas)
//
// Dengan thiserror (library):
// #[derive(Debug, thiserror::Error)]
// enum FakturError {
//     #[error("NPWP tidak valid '{npwp}': {alasan}")]
//     NpwpTidakValid { npwp: String, alasan: String },
// }
// ============================================
