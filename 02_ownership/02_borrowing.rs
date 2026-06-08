// ============================================
// BORROWING — Pinjam tanpa ambil ownership
// ============================================
// Jalankan: rustc 02_borrowing.rs && ./02_borrowing
//
// Aturan Borrowing:
// 1. Boleh banyak &reference (immutable) sekaligus
// 2. HANYA BOLEH SATU &mut reference di satu waktu
// 3. Reference harus selalu valid (no dangling)

fn main() {
    // ==================
    // IMMUTABLE REFERENCE (&)
    // ==================
    let s = String::from("halo pajak.io");

    let r1 = &s; // borrow pertama
    let r2 = &s; // borrow kedua — boleh, dua-duanya immutable
    println!("{r1} dan {r2}"); // ✅

    // Tidak bisa ubah melalui reference immutable
    // r1.push_str("!"); // ❌ ERROR: cannot borrow as mutable

    // ==================
    // MUTABLE REFERENCE (&mut)
    // ==================
    let mut s2 = String::from("halo");

    {
        let r3 = &mut s2;
        r3.push_str(" dunia");
        println!("{r3}"); // ✅
    } // r3 selesai dipakai di sini

    // Sekarang boleh buat mutable reference baru
    let r4 = &mut s2;
    r4.push_str("!");
    println!("{r4}"); // "halo dunia!"

    // ==================
    // TIDAK BOLEH: mutable + immutable reference bersamaan
    // ==================
    let mut s3 = String::from("test");
    let imut = &s3;         // immutable ref
    // let mutbl = &mut s3; // ❌ ERROR: cannot borrow as mutable
                            // karena imut masih aktif

    println!("{imut}");     // setelah ini imut selesai
    let mutbl = &mut s3;    // ✅ sekarang boleh
    mutbl.push_str("!");
    println!("{mutbl}");

    // ==================
    // FUNCTION dengan REFERENCE
    // ==================
    let mut faktur = String::from("FKT-001");

    // Pass immutable ref — function hanya baca
    let panjang = hitung_panjang(&faktur);
    println!("Panjang '{}' = {}", faktur, panjang); // faktur masih valid

    // Pass mutable ref — function bisa ubah
    tambah_suffix(&mut faktur);
    println!("Setelah diubah: {faktur}");

    // ==================
    // SLICE — reference ke bagian data
    // ==================
    let kalimat = String::from("halo dunia rust");

    // String slice
    let kata_pertama = &kalimat[0..4]; // "halo"
    let kata_kedua   = &kalimat[5..10]; // "dunia"
    println!("{kata_pertama} | {kata_kedua}");

    // Fungsi yang return slice
    let pertama = ambil_kata_pertama(&kalimat);
    println!("Kata pertama: {pertama}");

    // Array slice
    let angka = [1, 2, 3, 4, 5];
    let tengah = &angka[1..4]; // [2, 3, 4]
    println!("Tengah: {:?}", tengah);
}

fn hitung_panjang(s: &String) -> usize {
    s.len()
} // s tidak di-drop karena hanya dipinjam

fn tambah_suffix(s: &mut String) {
    s.push_str("/2024");
}

fn ambil_kata_pertama(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// ============================================
// Summary:
//
// &T       = immutable reference, boleh banyak
// &mut T   = mutable reference, hanya satu
//
// Ini yang bikin Rust bebas dari:
// - Data race (concurrent modification)
// - Use after free
// - Double free
// ============================================
