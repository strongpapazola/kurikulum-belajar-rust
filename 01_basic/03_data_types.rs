// ============================================
// 03 - Tipe Data di Rust
// ============================================
// Jalankan: rustc 03_data_types.rs && ./03_data_types

fn main() {
    // ==================
    // INTEGER
    // ==================
    // i = signed (bisa negatif), u = unsigned (positif saja)
    // angka = ukuran bit
    let a: i8  = -128;        // -128 s/d 127
    let b: i32 = -2_000_000;  // paling umum dipakai
    let c: i64 = 9_000_000_000;
    let d: u8  = 255;         // 0 s/d 255
    let e: u32 = 4_294_967_295;
    let f: usize = 100;       // ukuran pointer (untuk index array)
    println!("{a} {b} {c} {d} {e} {f}");

    // Format integer
    println!("Hex: {:x}", 255);    // ff
    println!("Octal: {:o}", 255);  // 377
    println!("Binary: {:b}", 10);  // 1010

    // ==================
    // FLOAT
    // ==================
    let x: f32 = 3.14;
    let y: f64 = 3.141592653589793; // default, lebih presisi
    println!("f32: {x:.4}");
    println!("f64: {y:.10}");

    // ==================
    // BOOLEAN
    // ==================
    let benar: bool = true;
    let salah: bool = false;
    println!("benar: {benar}, salah: {salah}");
    println!("AND: {}", benar && salah);
    println!("OR: {}",  benar || salah);
    println!("NOT: {}", !benar);

    // ==================
    // CHAR — pakai single quote, support unicode
    // ==================
    let huruf: char = 'A';
    let emoji: char = '🦀';
    let arab: char = 'ع';
    println!("char: {huruf} {emoji} {arab}");

    // ==================
    // TUPLE — koleksi tipe berbeda, fixed size
    // ==================
    let profil: (String, i32, bool) = (String::from("Bintang"), 25, true);
    println!("Nama: {}", profil.0);
    println!("Umur: {}", profil.1);
    println!("Aktif: {}", profil.2);

    // Destructuring tuple
    let (nama, umur, aktif) = profil;
    println!("Destructure: {nama}, {umur}, {aktif}");

    // Unit tuple — kosong, dipakai sebagai "void"
    let kosong: () = ();
    println!("Unit: {:?}", kosong);

    // ==================
    // ARRAY — fixed size, tipe sama semua
    // ==================
    let angka: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", angka);
    println!("Index 0: {}", angka[0]);
    println!("Panjang: {}", angka.len());

    // Array dengan nilai sama semua
    let nol = [0; 10]; // 10 elemen, semua 0
    println!("Nol: {:?}", nol);

    // Slice dari array
    let sebagian = &angka[1..4]; // index 1, 2, 3
    println!("Slice: {:?}", sebagian);
}

// ============================================
// Tipe Data Rust vs PHP/JS/Python:
//
// PHP:    int, float, string, bool, array
// JS:     number, string, boolean, null, undefined
// Python: int, float, str, bool, list, tuple, dict
// Rust:   i8/i32/u32/f64, bool, char, tuple, array
//
// Key difference: Rust TIDAK PUNYA null/undefined/None bawaan
// → Diganti dengan Option<T> (dibahas di bagian enums)
// ============================================
