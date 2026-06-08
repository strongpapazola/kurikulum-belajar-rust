// ============================================
// OWNERSHIP — Konsep paling unik di Rust
// ============================================
// Jalankan: rustc 01_ownership.rs && ./01_ownership
//
// 3 Aturan Ownership:
// 1. Setiap nilai punya satu "owner"
// 2. Hanya boleh ada SATU owner di satu waktu
// 3. Ketika owner keluar dari scope → nilai di-drop (memory bebas)

fn main() {
    // ==================
    // STACK vs HEAP
    // ==================
    // Tipe primitif (i32, bool, f64, dll) → disimpan di STACK → di-copy otomatis
    let a = 5;
    let b = a;       // COPY — a masih valid
    println!("a={a}, b={b}"); // keduanya bisa dipakai

    // String, Vec, dll → disimpan di HEAP → di-MOVE
    let s1 = String::from("halo");
    let s2 = s1;     // MOVE — s1 tidak valid lagi!
    // println!("{s1}"); // ❌ ERROR: value borrowed after move
    println!("{s2}"); // ✅ hanya s2 yang valid

    // ==================
    // CLONE — kalau mau copy heap data
    // ==================
    let s3 = String::from("halo");
    let s4 = s3.clone(); // deep copy — keduanya valid
    println!("s3={s3}, s4={s4}");

    // ==================
    // OWNERSHIP & FUNCTION
    // ==================
    let nama = String::from("Bintang");
    ambil_ownership(nama); // nama di-MOVE ke function
    // println!("{nama}"); // ❌ ERROR: nama sudah berpindah ke function

    // Solusi 1: return value balik
    let nama2 = String::from("Bintang");
    let nama2 = kembalikan_ownership(nama2); // ambil balik
    println!("nama2 = {nama2}"); // ✅

    // Solusi 2: pakai borrowing (& reference) → lebih idiomatis
    let nama3 = String::from("Bintang");
    pinjam_saja(&nama3); // pakai & → tidak move
    println!("nama3 = {nama3}"); // ✅ masih valid!

    // ==================
    // DROP — memory otomatis bebas saat keluar scope
    // ==================
    {
        let _temp = String::from("hanya sementara");
        println!("dalam scope: _temp ada");
    } // ← _temp di-drop di sini, memory langsung bebas
    // println!("{_temp}"); // ❌ tidak bisa, sudah keluar scope

    println!("\nTidak ada memory leak! Rust otomatis bersih-bersih 🧹");
}

fn ambil_ownership(s: String) {
    println!("Dapat ownership dari: {s}");
} // s di-drop di sini

fn kembalikan_ownership(s: String) -> String {
    println!("Dapat dan kembalikan: {s}");
    s // return → transfer ownership ke caller
}

fn pinjam_saja(s: &String) {
    println!("Pinjam: {s}");
} // s tidak di-drop karena cuma pinjam

// ============================================
// Kenapa ownership?
// → Tidak butuh garbage collector (GC)
// → Tidak ada memory leak
// → Tidak ada dangling pointer
// → Memory safety dijamin saat compile time
// ============================================
