// ============================================
// 02 - Operator ? (Question Mark)
// ============================================
// Jalankan: rustc 02_question_mark.rs && ./02_question_mark
//
// Operator ? adalah cara idiomatis Rust untuk propagate error
// Pengganti verbose match { Ok(v) => v, Err(e) => return Err(e) }

use std::num::ParseFloatError;
use std::num::ParseIntError;

// ==================
// TANPA ? — verbose
// ==================
fn parse_dan_tambah_verbose(a: &str, b: &str) -> Result<f64, ParseFloatError> {
    let x = match a.parse::<f64>() {
        Ok(v)  => v,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<f64>() {
        Ok(v)  => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}

// ==================
// DENGAN ? — ringkas
// ==================
fn parse_dan_tambah(a: &str, b: &str) -> Result<f64, ParseFloatError> {
    let x: f64 = a.parse()?; // kalau Err → langsung return Err
    let y: f64 = b.parse()?;
    Ok(x + y)
}

// ==================
// CHAINING ? — pipeline proses
// ==================
fn hitung_ppn_dari_string(total_str: &str) -> Result<String, ParseFloatError> {
    let total: f64 = total_str.trim().parse()?;
    let ppn   = total * 0.11;
    let bayar = total + ppn;
    Ok(format!(
        "DPP: {:.0} | PPN: {:.0} | Total: {:.0}",
        total, ppn, bayar
    ))
}

// ==================
// ? dengan MULTIPLE ERROR TYPE
// ==================
#[derive(Debug)]
enum AppError {
    ParseError(String),
    ValidationError(String),
}

impl From<ParseFloatError> for AppError {
    fn from(e: ParseFloatError) -> Self {
        AppError::ParseError(e.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e.to_string())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::ParseError(s)      => write!(f, "Parse error: {s}"),
            AppError::ValidationError(s) => write!(f, "Validasi error: {s}"),
        }
    }
}

// Dengan From impl, ? otomatis konversi error type
fn proses_faktur(total_str: &str, qty_str: &str) -> Result<f64, AppError> {
    let total: f64 = total_str.parse()?; // ParseFloatError → AppError otomatis
    let qty: i32   = qty_str.parse()?;   // ParseIntError → AppError otomatis

    if qty <= 0 {
        return Err(AppError::ValidationError(
            "Qty harus lebih dari 0".to_string()
        ));
    }

    Ok(total * qty as f64)
}

// ==================
// ? dalam NESTED FUNCTION — chain yang panjang
// ==================
fn step1(input: &str) -> Result<f64, AppError> {
    let n: f64 = input.parse()?;
    if n < 0.0 {
        return Err(AppError::ValidationError("Nilai harus positif".to_string()));
    }
    Ok(n)
}

fn step2(n: f64) -> Result<f64, AppError> {
    if n > 1_000_000_000.0 {
        return Err(AppError::ValidationError("Nilai terlalu besar".to_string()));
    }
    Ok(n * 1.11) // tambah PPN
}

fn step3(n: f64) -> Result<String, AppError> {
    Ok(format!("Final: Rp {:.0}", n))
}

// Pipeline bersih dengan ?
fn pipeline(input: &str) -> Result<String, AppError> {
    let a = step1(input)?;
    let b = step2(a)?;
    let c = step3(b)?;
    Ok(c)
}

fn main() {
    println!("=== Tanpa ? (verbose) ===");
    println!("{:?}", parse_dan_tambah_verbose("10.5", "20.3"));
    println!("{:?}", parse_dan_tambah_verbose("abc", "20.3"));

    println!("\n=== Dengan ? (ringkas) ===");
    println!("{:?}", parse_dan_tambah("10.5", "20.3"));
    println!("{:?}", parse_dan_tambah("abc", "20.3"));

    println!("\n=== Chaining ? ===");
    match hitung_ppn_dari_string("  5000000  ") {
        Ok(hasil) => println!("✅ {hasil}"),
        Err(e)    => println!("❌ {e}"),
    }
    match hitung_ppn_dari_string("bukan_angka") {
        Ok(hasil) => println!("✅ {hasil}"),
        Err(e)    => println!("❌ {e}"),
    }

    println!("\n=== Multiple Error Type ===");
    let tests = vec![
        ("1500000", "3"),
        ("bukan_angka", "3"),
        ("1500000", "bukan_angka"),
        ("1500000", "0"),
        ("1500000", "-1"),
    ];
    for (total, qty) in tests {
        match proses_faktur(total, qty) {
            Ok(hasil) => println!("✅ total={total}, qty={qty} → Rp {:.0}", hasil),
            Err(e)    => println!("❌ total={total}, qty={qty} → {e}"),
        }
    }

    println!("\n=== Pipeline ===");
    for input in ["5000000", "-100", "abc", "999999999999"] {
        match pipeline(input) {
            Ok(hasil) => println!("✅ {input} → {hasil}"),
            Err(e)    => println!("❌ {input} → {e}"),
        }
    }
}

// ============================================
// Aturan ?:
// 1. Hanya bisa dipakai dalam function yang return Result atau Option
// 2. Tipe error harus match atau ada implementasi From
// 3. Kalau return Option → ? konversi None jadi early return None
//
// Idiom Rust:
//   val?           → unwrap atau propagate error
//   val.unwrap()   → unwrap atau PANIC (hindari di production)
//   val.expect()   → unwrap atau PANIC dengan pesan custom
// ============================================
