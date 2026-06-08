// ============================================
// OPTION<T> — Pengganti null di Rust
// ============================================
// Jalankan: rustc 02_option.rs && ./02_option
//
// Di PHP/JS/Python ada null/undefined/None yang bisa muncul tak terduga
// Di Rust: TIDAK ADA null. Diganti Option<T> yang harus di-handle secara eksplisit
//
// Option<T> adalah:
//   Some(T)  → ada nilainya
//   None     → tidak ada nilai

fn cari_faktur(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("FKT-001")),
        2 => Some(String::from("FKT-002")),
        _ => None, // tidak ditemukan
    }
}

fn bagi(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    // ==================
    // CARA HANDLE OPTION
    // ==================

    // 1. match — paling explicit
    let faktur = cari_faktur(1);
    match faktur {
        Some(f) => println!("Ditemukan: {f}"),
        None    => println!("Tidak ditemukan"),
    }

    // 2. if let — lebih ringkas kalau hanya peduli Some
    if let Some(f) = cari_faktur(2) {
        println!("Faktur: {f}");
    }

    // 3. unwrap() — langsung ambil nilai, PANIC kalau None
    //    Hindari di production! Pakai untuk testing/prototyping
    let f = cari_faktur(1).unwrap();
    println!("Unwrap: {f}");

    // 4. unwrap_or() — kalau None, pakai default
    let f = cari_faktur(99).unwrap_or(String::from("TIDAK ADA"));
    println!("unwrap_or: {f}");

    // 5. unwrap_or_else() — kalau None, jalankan closure
    let f = cari_faktur(99).unwrap_or_else(|| {
        format!("Default-{}", 99)
    });
    println!("unwrap_or_else: {f}");

    // 6. ? operator di dalam function yang return Option
    // (lihat contoh di bawah)

    // ==================
    // CHAINING OPTION
    // ==================
    let hasil = cari_faktur(1)
        .map(|f| f.to_uppercase());         // transform kalau Some
    println!("map: {:?}", hasil);

    let hasil = cari_faktur(99)
        .map(|f| f.to_uppercase());         // None tetap None
    println!("map None: {:?}", hasil);

    // filter
    let hasil = cari_faktur(1)
        .filter(|f| f.starts_with("FKT")); // None kalau kondisi false
    println!("filter: {:?}", hasil);

    // and_then (flatMap) — chaining multiple Option
    let hasil = cari_faktur(1)
        .and_then(|f| {
            if f.len() > 3 { Some(f) } else { None }
        });
    println!("and_then: {:?}", hasil);

    // ==================
    // DIVISION EXAMPLE
    // ==================
    match bagi(10.0, 3.0) {
        Some(h) => println!("10/3 = {:.4}", h),
        None    => println!("Tidak bisa bagi dengan nol"),
    }

    match bagi(10.0, 0.0) {
        Some(h) => println!("Hasil: {h}"),
        None    => println!("Error: bagi dengan nol"),
    }

    // ==================
    // IS_SOME / IS_NONE
    // ==================
    let ada = cari_faktur(1);
    let tidak_ada = cari_faktur(99);

    println!("ada.is_some() = {}", ada.is_some());       // true
    println!("tidak_ada.is_none() = {}", tidak_ada.is_none()); // true

    // Contoh penggunaan di dalam function yang return Option
    if let Some(hasil) = proses_faktur(1) {
        println!("Proses berhasil: {hasil}");
    }
}

// Fungsi yang return Option, pakai ? untuk early return None
fn proses_faktur(id: u32) -> Option<String> {
    let faktur = cari_faktur(id)?;   // kalau None → langsung return None
    let upper = faktur.to_uppercase();
    Some(format!("Processed: {upper}"))
}

// ============================================
// Perbandingan:
//
// PHP:    function cari($id) { return null; }
//         $f = cari(1);
//         if ($f !== null) { ... }  // mudah lupa cek null!
//
// Python: def cari(id): return None
//         f = cari(1)
//         if f is not None: ...     // mudah lupa
//
// Rust:   fn cari(id: u32) -> Option<String> { None }
//         match cari(1) {           // WAJIB handle None
//             Some(f) => ...,       // compiler error kalau tidak!
//             None => ...,
//         }
//
// → Rust memaksa kamu handle "tidak ada nilai" secara eksplisit
// ============================================
