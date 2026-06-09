// ============================================
// 02 - String vs &str di Rust
// ============================================
// Jalankan: rustc 02_strings.rs && ./02_strings
//
// Ini yang paling sering bikin bingung di awal:
// Rust punya DUA tipe string yang berbeda!

fn main() {
    // ==================
    // &str vs String
    // ==================

    // &str = string literal, immutable, di stack/binary
    //        ukuran diketahui saat compile time
    let s1: &str = "halo pajak.io";

    // String = owned, mutable, di heap
    //          ukuran bisa berubah saat runtime
    let s2: String = String::from("halo pajak.io");
    let s3: String = "halo pajak.io".to_string(); // cara lain

    println!("&str  : {s1}");
    println!("String: {s2}");
    println!("String: {s3}");

    // ==================
    // BUAT & MODIFIKASI String
    // ==================
    let mut nama = String::new(); // kosong
    nama.push_str("pajak");      // append string
    nama.push('.');              // append satu char
    nama.push_str("io");
    println!("\nDibuild: {nama}");

    // Concatenation dengan +
    let s4 = String::from("Selamat ");
    let s5 = String::from("datang!");
    let s6 = s4 + &s5; // s4 di-move, s5 dipinjam
    // println!("{s4}"); // ❌ s4 sudah di-move
    println!("Concat: {s6}");

    // Concatenation lebih dari 2 → pakai format! (lebih readable)
    let nama2  = String::from("Bintang");
    let pesan2 = format!("Halo, {}! Selamat belajar Rust.", nama2);
    println!("{pesan2}");

    // ==================
    // STRING METHODS
    // ==================
    let teks = String::from("  Nomor Faktur Pajak 001  ");

    println!("\nOriginal  : '{teks}'");
    println!("trim      : '{}'", teks.trim());
    println!("uppercase : '{}'", teks.trim().to_uppercase());
    println!("lowercase : '{}'", teks.trim().to_lowercase());
    println!("len       : {}", teks.len());
    println!("contains  : {}", teks.contains("Faktur"));
    println!("starts    : {}", teks.trim().starts_with("Nomor"));
    println!("ends      : {}", teks.trim().ends_with("001"));
    println!("replace   : '{}'", teks.trim().replace("Faktur", "Invoice"));

    // Split
    let csv = "FKT-001,PT Maju,1000000,PPN";
    let kolom: Vec<&str> = csv.split(',').collect();
    println!("\nCSV split: {:?}", kolom);
    println!("Kolom 1: {}", kolom[0]);
    println!("Kolom 2: {}", kolom[1]);

    // Join
    let kata = vec!["Pajak", "Pertambahan", "Nilai"];
    let kalimat = kata.join(" ");
    println!("Join: {kalimat}");

    // ==================
    // PARSING
    // ==================
    let angka_str = "1_500_000";
    let angka_str_clean = angka_str.replace('_', "");
    let angka: i64 = angka_str_clean.parse().unwrap_or(0);
    println!("\nParse '{}' → {}", angka_str, angka);

    // ==================
    // SLICING
    // ==================
    let npwp = "12.345.678.9-012.345";
    let bagian1 = &npwp[0..9];   // "12.345.67"
    let bagian2 = &npwp[9..];    // "8.9-012.345"
    println!("\nNPWP   : {npwp}");
    println!("Bagian1: {bagian1}");
    println!("Bagian2: {bagian2}");

    // ==================
    // CHARS — iterasi karakter
    // ==================
    let kata2 = "Rust🦀";
    println!("\nChars: ");
    for c in kata2.chars() {
        print!("[{c}] ");
    }
    println!();
    println!("Jumlah char: {}", kata2.chars().count());

    // Hitung digit dari NPWP
    let npwp2 = "12.345.678.9-012.345";
    let digit_count = npwp2.chars().filter(|c| c.is_ascii_digit()).count();
    println!("\nDigit di NPWP: {digit_count}");

    // ==================
    // TIPS: kapan pakai &str vs String?
    // ==================
    // &str → untuk parameter function (terima keduanya)
    // String → kalau butuh ownership, menyimpan di struct, atau perlu modifikasi
    sapa("literal");       // ✅
    sapa(&s2);             // ✅ String auto-deref ke &str
    sapa(&s2[0..5]);       // ✅ slice pun bisa
}

// Pakai &str untuk parameter → lebih fleksibel
fn sapa(nama: &str) {
    println!("Halo, {nama}!");
}

// ============================================
// Analogi:
// &str  → seperti pointer ke text (tidak punya)
// String → seperti Vec<u8> yang owned (punya)
//
// Dari PHP/Python: semua string itu owned
// Di Rust: ada dua → harus pilih yang tepat
// ============================================
