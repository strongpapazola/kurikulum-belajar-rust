// ============================================
// 01 - Hello World & Cara Print di Rust
// ============================================
// Jalankan: rustc 01_hello_world.rs && ./01_hello_world

fn main() {
    // --- println! → print dengan newline ---
    println!("Halo, Dunia!");
    println!("Belajar Rust dari background PHP/Node/Python");

    // --- print! → print TANPA newline ---
    print!("Halo ");
    print!("Bintang ");
    println!(); // newline manual

    // --- Format string dengan {} ---
    let nama = "Bintang";
    let umur = 25;
    println!("Nama: {}, Umur: {}", nama, umur);

    // --- Named argument (lebih readable) ---
    println!("Nama: {nama}, Umur: {umur}");

    // --- format! → buat string tanpa print ---
    let pesan = format!("Halo, {}!", nama);
    println!("{}", pesan);

    // --- Debug print dengan {:?} ---
    let angka = [1, 2, 3, 4, 5];
    println!("Array: {:?}", angka);
    println!("Array pretty: {:#?}", angka); // lebih rapi

    // --- Formatting angka ---
    let pi = 3.14159265;
    println!("Pi: {:.2}", pi);       // 2 desimal: 3.14
    println!("Pi: {:8.3}", pi);      // lebar 8, 3 desimal
    println!("Padding: {:>10}", "kanan");  // rata kanan
    println!("Padding: {:<10}", "kiri");   // rata kiri
    println!("Padding: {:^10}", "tengah"); // tengah

    // --- eprintln! → print ke stderr (untuk error/log) ---
    eprintln!("Ini pesan error/log ke stderr");
}

// ============================================
// Perbandingan dengan bahasa lain:
//
// PHP:    echo "Hello " . $nama;
// JS:     console.log(`Hello ${nama}`)
// Python: print(f"Hello {nama}")
// Rust:   println!("Hello {nama}")
// ============================================
