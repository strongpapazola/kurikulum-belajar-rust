// ============================================
// 02 - Variables, Mutability, Shadowing, Const
// ============================================
// Jalankan: rustc 02_variables.rs && ./02_variables

fn main() {
    // --- Immutable by default ---
    // Beda dari PHP/JS/Python — variable di Rust TIDAK BISA diubah kecuali pakai mut
    let x = 5;
    println!("x = {x}");

    // x = 10; // ❌ ERROR: cannot assign twice to immutable variable

    // --- Mutable dengan mut ---
    let mut y = 5;
    println!("y awal = {y}");
    y = 10; // ✅ OK karena mut
    println!("y setelah = {y}");

    // --- Shadowing — bisa re-declare variable dengan nama sama ---
    // Beda dengan mut: shadowing bisa GANTI TIPE
    let z = 5;
    let z = z + 1;       // shadow z pertama
    let z = z * 2;       // shadow z kedua
    println!("z = {z}"); // 12

    // Shadowing bisa ganti tipe (ini yang tidak bisa dilakukan mut)
    let tipe = "hello";
    println!("tipe = {tipe}");        // &str
    let tipe = tipe.len();
    println!("tipe = {tipe}");        // usize (integer)

    // --- Const — konstanta, WAJIB ada tipe, berlaku global ---
    const MAX_TRANSAKSI: u32 = 1_000_000; // underscore untuk readability angka besar
    const NAMA_APLIKASI: &str = "pajak.io";
    println!("Max transaksi: {MAX_TRANSAKSI}");
    println!("Aplikasi: {NAMA_APLIKASI}");

    // --- Type inference vs explicit ---
    let otomatis = 42;        // Rust tahu ini i32
    let eksplisit: i64 = 42; // kita tentukan sendiri
    println!("{otomatis} {eksplisit}");

    // --- Multiple assignment (tuple) ---
    let (a, b, c) = (1, 2, 3);
    println!("a={a}, b={b}, c={c}");
}

// ============================================
// Perbandingan:
//
// PHP:    $nama = "budi";   (selalu mutable)
// JS:     const x = 5;      let y = 5;
// Python: x = 5             (selalu mutable)
// Rust:   let x = 5;        (immutable)
//         let mut x = 5;    (mutable)
// ============================================
