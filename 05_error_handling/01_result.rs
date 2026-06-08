// ============================================
// RESULT<T, E> — Error Handling di Rust
// ============================================
// Jalankan: rustc 01_result.rs && ./01_result
//
// Result<T, E> adalah:
//   Ok(T)  → sukses, bawa nilai
//   Err(E) → gagal, bawa error
//
// Tidak ada try/catch di Rust — error adalah nilai biasa

use std::num::ParseIntError;

// Function yang bisa gagal → return Result
fn parse_npwp(input: &str) -> Result<String, String> {
    // NPWP format: XX.XXX.XXX.X-XXX.XXX (15 digit)
    let digit_only: String = input.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    if digit_only.len() != 15 {
        return Err(format!("NPWP harus 15 digit, dapat: {}", digit_only.len()));
    }

    Ok(digit_only)
}

fn hitung_pph21(gaji: f64, npwp: &str) -> Result<f64, String> {
    let _ = parse_npwp(npwp)?; // ? = kalau Err, langsung return Err

    if gaji < 0.0 {
        return Err(String::from("Gaji tidak boleh negatif"));
    }

    let ptkp = 54_000_000.0;
    let penghasilan_kena = (gaji - ptkp).max(0.0);
    let pajak = penghasilan_kena * 0.05;

    Ok(pajak)
}

fn parse_angka(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>() // parse() sudah return Result
}

fn main() {
    // ==================
    // CARA HANDLE RESULT
    // ==================

    // 1. match — paling explicit
    match parse_npwp("12.345.678.9-012.345") {
        Ok(npwp)  => println!("NPWP valid: {npwp}"),
        Err(e)    => println!("Error: {e}"),
    }

    match parse_npwp("123") {
        Ok(npwp)  => println!("NPWP: {npwp}"),
        Err(e)    => println!("Error: {e}"),
    }

    // 2. if let
    if let Ok(npwp) = parse_npwp("12.345.678.9-012.345") {
        println!("if let OK: {npwp}");
    }

    // 3. unwrap() — PANIC kalau Err (hindari di production)
    let npwp = parse_npwp("12.345.678.9-012.345").unwrap();
    println!("unwrap: {npwp}");

    // 4. unwrap_or() — default kalau error
    let npwp = parse_npwp("INVALID").unwrap_or(String::from("000000000000000"));
    println!("unwrap_or: {npwp}");

    // 5. expect() — seperti unwrap tapi pesan error lebih jelas
    let npwp = parse_npwp("12.345.678.9-012.345")
        .expect("NPWP harus valid di titik ini");
    println!("expect: {npwp}");

    // ==================
    // CHAINING dengan ?
    // ==================
    match hitung_pph21(100_000_000.0, "12.345.678.9-012.345") {
        Ok(pajak)  => println!("\nPPh 21: Rp {:.0}", pajak),
        Err(e)     => println!("\nError hitung PPh 21: {e}"),
    }

    match hitung_pph21(100_000_000.0, "INVALID") {
        Ok(pajak)  => println!("PPh 21: Rp {:.0}", pajak),
        Err(e)     => println!("Error: {e}"),
    }

    // ==================
    // MAP & AND_THEN
    // ==================
    let hasil = parse_angka("42")
        .map(|n| n * 2);       // transform Ok value
    println!("\nmap: {:?}", hasil); // Ok(84)

    let hasil = parse_angka("abc")
        .map(|n| n * 2);
    println!("map err: {:?}", hasil); // Err(...)

    let hasil = parse_angka("42")
        .and_then(|n| {
            if n > 0 { Ok(n * 100) } else { Err("must be positive".parse::<i32>().unwrap_err()) }
        });
    println!("and_then: {:?}", hasil);

    // ==================
    // COLLECT RESULTS
    // ==================
    let inputs = vec!["1", "2", "abc", "4"];
    let results: Vec<Result<i32, _>> = inputs.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("\nResults: {:?}", results);

    // Collect — stop di error pertama
    let valid: Result<Vec<i32>, _> = vec!["1", "2", "3"].iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("All valid: {:?}", valid); // Ok([1,2,3])

    let invalid: Result<Vec<i32>, _> = vec!["1", "abc", "3"].iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Has invalid: {:?}", invalid); // Err(...)
}

// ============================================
// Perbandingan:
//
// PHP:    try { ... } catch (Exception $e) { ... }
// JS:     try { ... } catch (e) { ... }
// Python: try: ... except Exception as e: ...
//
// Rust:   match fungsi() {
//             Ok(val) => ...,
//             Err(e)  => ...,
//         }
//
// Key: di Rust error adalah nilai return biasa
//      Tidak ada exception yang "muncul tiba-tiba"
//      Setiap function yang bisa gagal → wajib return Result
// ============================================
