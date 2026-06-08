// ============================================
// EXERCISE - Ownership & Borrowing
// ============================================
// Jalankan: rustc ex01_ownership.rs && ./ex01_ownership

fn main() {
    // TODO 1: Kode ini error, fix tanpa menghapus println! kedua
    //         Hint: gunakan .clone()
    // let s1 = String::from("FKT-001");
    // let s2 = s1;
    // println!("s1 = {s1}");
    // println!("s2 = {s2}");


    // TODO 2: Kode ini error, fix dengan menambah & pada pemanggilan function
    // let faktur = String::from("FKT-002");
    // let panjang = hitung_panjang(faktur);
    // println!("Faktur: {faktur}, panjang: {panjang}");  // faktur harus masih bisa dipakai


    // TODO 3: Buat function "tambah_ppn" yang menerima &mut String
    //         dan append " (+PPN 11%)" ke string tersebut
    //         lalu panggil dari sini:
    let mut deskripsi = String::from("Laptop Lenovo");
    // tambah_ppn(&mut deskripsi);
    // println!("{deskripsi}");
    // Expected: "Laptop Lenovo (+PPN 11%)"


    // TODO 4: Fix kode ini (ada pelanggaran aturan borrowing)
    // let mut s = String::from("halo");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;  // ← ini masalahnya
    // println!("{r1}, {r2}, {r3}");


    // TODO 5: Buat function "kata_terakhir" yang menerima &str
    //         dan return &str (slice kata terakhir)
    //         Hint: gunakan .split_whitespace().last()
    let kalimat = String::from("Nomor Faktur Pajak");
    // let terakhir = kata_terakhir(&kalimat);
    // println!("Kata terakhir: {terakhir}");
    // Expected: "Pajak"


    // TODO 6: Jelaskan (dengan komentar) kenapa kode ini valid:
    let v = vec![1, 2, 3];
    let first = &v[0];
    println!("first = {first}");
    println!("v = {:?}", v);
    // Kenapa first dan v keduanya bisa dipakai?
    // Jawab: ...

}

// TODO: implementasikan function-function yang dibutuhkan di sini
fn hitung_panjang(s: &String) -> usize {
    todo!()
}

// ============================================
// Expected output:
// s1 = FKT-001
// s2 = FKT-001
// Faktur: FKT-002, panjang: 7
// Laptop Lenovo (+PPN 11%)
// Kata terakhir: Pajak
// first = 1
// v = [1, 2, 3]
// ============================================
