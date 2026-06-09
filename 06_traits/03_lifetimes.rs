// ============================================
// 03 - Lifetimes di Rust
// ============================================
// Jalankan: rustc 03_lifetimes.rs && ./03_lifetimes
//
// Lifetime = berapa lama sebuah reference valid
// Compiler perlu tahu ini untuk mencegah dangling reference
// Kebanyakan kasus lifetime di-infer otomatis (lifetime elision)

// ==================
// MASALAH TANPA LIFETIME ANNOTATION
// ==================

// Ini error — compiler tidak tahu reference mana yang di-return
// fn paling_panjang(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// Dengan lifetime annotation — kasih tau compiler
// 'a artinya: return value hidup selama yang lebih pendek antara x dan y
fn paling_panjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ==================
// LIFETIME DALAM STRUCT
// ==================
// Struct yang menyimpan reference butuh lifetime
#[derive(Debug)]
struct PetikanFaktur<'a> {
    nomor: &'a str,    // reference ke string di luar struct
    keterangan: &'a str,
}

impl<'a> PetikanFaktur<'a> {
    fn tampilkan(&self) {
        println!("Faktur: {} | {}", self.nomor, self.keterangan);
    }

    // Method yang return reference — lifetime di-infer dari &self
    fn nomor(&self) -> &str {
        self.nomor
    }
}

// ==================
// LIFETIME ELISION — aturan yang bisa skip annotation
// ==================

// Rule: kalau hanya 1 parameter reference → output lifetime = input lifetime
// Tidak perlu tulis 'a eksplisit!
fn pertama_kata(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Rule: method dengan &self — output bisa hidup selama &self hidup
struct Laporan {
    judul: String,
    isi: String,
}

impl Laporan {
    fn judul(&self) -> &str {    // &self → &str punya lifetime sama dengan self
        &self.judul
    }

    fn ringkasan(&self) -> &str {
        &self.isi[..self.isi.len().min(50)]
    }
}

// ==================
// STATIC LIFETIME
// ==================
// 'static = reference hidup selama program berjalan
// String literal selalu 'static
const NAMA_SISTEM: &'static str = "pajak.io v2";
// atau singkat:
const VERSI: &str = "2.0.0"; // kompiler tahu ini 'static

fn pesan_selamat_datang() -> &'static str {
    "Selamat datang di sistem perpajakan"
    // String literal = 'static
}

// ==================
// KOMBINASI: Generic + Trait Bound + Lifetime
// ==================
use std::fmt::Display;

fn cetak_yang_panjang<'a, T>(x: &'a str, y: &'a str, extra: T) -> &'a str
where
    T: Display,
{
    println!("Extra info: {extra}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("=== Lifetimes ===\n");

    // paling_panjang
    let s1 = String::from("Faktur Pajak Pertambahan Nilai");
    let hasil;
    {
        let s2 = String::from("PPN");
        hasil = paling_panjang(&s1, &s2);
        println!("Paling panjang: {hasil}");
        // hasil valid di sini karena s1 dan s2 masih ada
    }
    // hasil tidak bisa dipakai di luar scope s2 jika ia meminjam s2
    // Compiler akan error kalau kita coba akses hasil di sini dan ia dari s2

    // Struct dengan lifetime
    println!("\n=== Struct Lifetime ===");
    let nomor   = String::from("FKT-001");
    let ket     = String::from("Penjualan laptop ke PT Maju Jaya");
    let petikan = PetikanFaktur {
        nomor: &nomor,
        keterangan: &ket,
    };
    petikan.tampilkan();
    println!("Nomor: {}", petikan.nomor());

    // Lifetime elision
    println!("\n=== Lifetime Elision ===");
    let kalimat = "Nomor Faktur Pajak Pertambahan Nilai";
    println!("Pertama: {}", pertama_kata(kalimat));

    let laporan = Laporan {
        judul: String::from("Laporan PPh Bulanan"),
        isi: String::from("Total PPh 21 bulan ini sebesar Rp 15.000.000"),
    };
    println!("Judul: {}", laporan.judul());
    println!("Ringkasan: {}", laporan.ringkasan());

    // Static lifetime
    println!("\n=== Static Lifetime ===");
    println!("{NAMA_SISTEM}");
    println!("Versi: {VERSI}");
    println!("{}", pesan_selamat_datang());

    // Kombinasi
    println!("\n=== Generic + Trait + Lifetime ===");
    let a = String::from("Pajak Penghasilan Pasal 21");
    let b = String::from("PPN");
    let panjang = cetak_yang_panjang(&a, &b, "tarif 5%");
    println!("Lebih panjang: {panjang}");
}

// ============================================
// Ringkasan Lifetime Rules (Elision):
// 1. Tiap reference parameter dapat lifetime sendiri
// 2. Kalau hanya 1 input ref → output ref punya lifetime yang sama
// 3. Kalau ada &self → output ref punya lifetime &self
//
// Kapan perlu eksplisit:
// - Fungsi return reference dari beberapa parameter
// - Struct yang menyimpan reference
// - Kombinasi complex
//
// Analoginya:
// Lifetime = masa berlaku "izin pinjam"
// Compiler memastikan tidak ada yang pakai barang
// setelah izin pinjam habis (= pemilik sudah drop)
// ============================================
