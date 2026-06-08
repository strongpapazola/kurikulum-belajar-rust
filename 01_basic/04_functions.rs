// ============================================
// 04 - Functions di Rust
// ============================================
// Jalankan: rustc 04_functions.rs && ./04_functions

fn main() {
    // Memanggil function
    sapa();
    sapa_nama("Bintang");

    let hasil = tambah(10, 20);
    println!("10 + 20 = {hasil}");

    let luas = hitung_luas(5.0, 3.0);
    println!("Luas: {luas}");

    // Multiple return value pakai tuple
    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Min: {min}, Max: {max}");

    // Closures (function anonymous — mirip arrow function JS / lambda Python)
    let kali_dua = |x: i32| x * 2;
    println!("kali_dua(5) = {}", kali_dua(5));

    // Closure capture variable dari luar
    let faktor = 3;
    let kali_faktor = |x: i32| x * faktor;
    println!("kali_faktor(5) = {}", kali_faktor(5));

    // Higher order function (function sebagai parameter)
    let angka = vec![1, 2, 3, 4, 5];
    let genap: Vec<i32> = angka.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 10)
        .collect();
    println!("Genap * 10: {:?}", genap);

    // Memanggil function rekursif
    println!("Faktorial 5 = {}", faktorial(5));
}

// --- Function tanpa parameter, tanpa return ---
fn sapa() {
    println!("Halo dari function!");
}

// --- Function dengan parameter ---
fn sapa_nama(nama: &str) {
    println!("Halo, {}!", nama);
}

// --- Function dengan return value ---
// Tidak perlu keyword return kalau ekspresi terakhir (tanpa titik koma!)
fn tambah(a: i32, b: i32) -> i32 {
    a + b  // tidak ada titik koma = ini return value
}

// Sama seperti di atas, tapi explicit return
fn hitung_luas(panjang: f64, lebar: f64) -> f64 {
    return panjang * lebar; // explicit return boleh pakai titik koma
}

// --- Return multiple value pakai tuple ---
fn min_max(data: &[i32]) -> (i32, i32) {
    let mut min = data[0];
    let mut max = data[0];
    for &val in data.iter() {
        if val < min { min = val; }
        if val > max { max = val; }
    }
    (min, max) // return tuple
}

// --- Rekursif ---
fn faktorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * faktorial(n - 1)
    }
}

// ============================================
// Perbandingan:
//
// PHP:    function tambah($a, $b) { return $a + $b; }
// JS:     function tambah(a, b) { return a + b; }
//         const tambah = (a, b) => a + b;
// Python: def tambah(a, b): return a + b
// Rust:   fn tambah(a: i32, b: i32) -> i32 { a + b }
//
// Key: Rust WAJIB tipe parameter & return type
//      Baris terakhir tanpa ; = otomatis jadi return value
// ============================================
