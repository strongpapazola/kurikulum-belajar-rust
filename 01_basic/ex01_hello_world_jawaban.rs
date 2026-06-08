// ============================================
// JAWABAN - Exercise 01 Hello World
// ============================================
// Coba kerjain sendiri dulu sebelum lihat ini!
// Jalankan: rustc ex01_hello_world_jawaban.rs && ./ex01_hello_world_jawaban

fn main() {
    // TODO 1
    println!("Halo, Bintang!");

    // TODO 2
    println!("{} adalah jawaban", 42);

    // TODO 3
    let pi = 3.14159_f64;
    println!("Pi = {pi:.2}");

    // TODO 4
    let nama = "pajak.io";
    let versi = 2;
    println!("Selamat datang di {nama} versi {versi}");

    // TODO 5
    let data = [10, 20, 30, 40, 50];
    println!("{:?}", data);

    // TODO 6
    println!("{:>20}", "pajak.io");

    // TODO 7
    eprintln!("Koneksi DJP gagal!");
}
