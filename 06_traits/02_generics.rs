// ============================================
// 02 - Generics di Rust
// ============================================
// Jalankan: rustc 02_generics.rs && ./02_generics
//
// Generics = kode yang bisa bekerja dengan berbagai tipe
// Mirip template di C++ atau generics di Java/TypeScript

// ==================
// GENERIC FUNCTION
// ==================

// Tanpa generic — harus buat fungsi terpisah per tipe
fn terbesar_i32(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list { if item > max { max = item; } }
    max
}

// Dengan generic — satu fungsi untuk semua tipe yang support PartialOrd
fn terbesar<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    max
}

// ==================
// GENERIC STRUCT
// ==================
#[derive(Debug)]
struct Pasangan<T, U> {
    pertama: T,
    kedua: U,
}

impl<T: std::fmt::Display, U: std::fmt::Display> Pasangan<T, U> {
    fn new(pertama: T, kedua: U) -> Self {
        Pasangan { pertama, kedua }
    }

    fn tampilkan(&self) {
        println!("({}, {})", self.pertama, self.kedua);
    }
}

// ==================
// GENERIC DENGAN MULTIPLE TRAIT BOUND
// ==================
use std::fmt;

fn cetak_dan_bandingkan<T>(a: T, b: T)
where
    T: fmt::Display + PartialOrd  // where clause lebih readable
{
    if a > b {
        println!("{a} lebih besar dari {b}");
    } else {
        println!("{a} lebih kecil atau sama dengan {b}");
    }
}

// ==================
// GENERIC STRUCT — use case nyata: Repository pattern
// ==================
#[derive(Debug, Clone)]
struct InMemoryRepo<T> {
    data: Vec<T>,
}

impl<T: Clone + fmt::Debug> InMemoryRepo<T> {
    fn new() -> Self {
        InMemoryRepo { data: Vec::new() }
    }

    fn simpan(&mut self, item: T) {
        self.data.push(item);
    }

    fn ambil_semua(&self) -> &[T] {
        &self.data
    }

    fn jumlah(&self) -> usize {
        self.data.len()
    }
}

// Bisa dipakai untuk tipe apapun!
#[derive(Debug, Clone)]
struct Faktur {
    nomor: String,
    total: f64,
}

#[derive(Debug, Clone)]
struct WajibPajak {
    npwp: String,
    nama: String,
}

// ==================
// GENERIC DENGAN RETURN TYPE
// ==================
fn wrap_ok<T>(val: T) -> Result<T, String> {
    Ok(val)
}

fn ambil_atau_default<T: Default + Clone>(opt: Option<T>) -> T {
    opt.unwrap_or_default()
}

// ==================
// GENERIC ENUM — seperti Option dan Result di stdlib
// ==================
#[derive(Debug)]
enum Nullable<T> {
    Ada(T),
    Kosong,
}

impl<T: fmt::Display> Nullable<T> {
    fn tampilkan(&self) {
        match self {
            Nullable::Ada(v)  => println!("Ada: {v}"),
            Nullable::Kosong  => println!("Kosong"),
        }
    }

    fn atau(self, default: T) -> T {
        match self {
            Nullable::Ada(v) => v,
            Nullable::Kosong => default,
        }
    }
}

fn main() {
    println!("=== Generic Functions ===");
    let angka = vec![34, 50, 25, 100, 65];
    println!("Terbesar i32: {}", terbesar_i32(&angka));
    println!("Terbesar generic: {}", terbesar(&angka));

    let float  = vec![3.14, 2.71, 1.41];
    let string = vec!["pajak", "ppn", "pph"];
    println!("Terbesar f64: {}", terbesar(&float));
    println!("Terbesar str: {}", terbesar(&string));

    println!("\n=== Generic Struct ===");
    let p1 = Pasangan::new("NPWP", "12.345.678.9-012.345");
    let p2 = Pasangan::new(1, 3.14);
    let p3 = Pasangan::new("Total", 1_500_000_u64);
    p1.tampilkan();
    p2.tampilkan();
    p3.tampilkan();

    println!("\n=== Multiple Trait Bound ===");
    cetak_dan_bandingkan(100, 200);
    cetak_dan_bandingkan("alpha", "beta");

    println!("\n=== Generic Repository ===");
    let mut faktur_repo: InMemoryRepo<Faktur> = InMemoryRepo::new();
    faktur_repo.simpan(Faktur { nomor: "FKT-001".to_string(), total: 1_500_000.0 });
    faktur_repo.simpan(Faktur { nomor: "FKT-002".to_string(), total: 2_300_000.0 });
    faktur_repo.simpan(Faktur { nomor: "FKT-003".to_string(), total: 800_000.0 });

    println!("Faktur tersimpan: {}", faktur_repo.jumlah());
    for f in faktur_repo.ambil_semua() {
        println!("  {} → Rp {:.0}", f.nomor, f.total);
    }

    let mut wp_repo: InMemoryRepo<WajibPajak> = InMemoryRepo::new();
    wp_repo.simpan(WajibPajak { npwp: "12.345.678.9-012.345".to_string(), nama: "PT Maju".to_string() });
    println!("\nWP tersimpan: {}", wp_repo.jumlah());

    println!("\n=== Generic Enum ===");
    let ada: Nullable<String> = Nullable::Ada("FKT-001".to_string());
    let kosong: Nullable<String> = Nullable::Kosong;
    ada.tampilkan();
    kosong.tampilkan();

    let val = Nullable::Kosong;
    println!("atau: {}", val.atau("DEFAULT".to_string()));
}

// ============================================
// Generic di berbagai bahasa:
//
// TypeScript: function terbesar<T extends number>(list: T[]): T
// Java:       public <T extends Comparable<T>> T terbesar(List<T> list)
// Python:     (duck typing, tidak perlu generic eksplisit)
//
// Rust: fn terbesar<T: PartialOrd>(list: &[T]) -> &T
//
// Keunggulan Rust: monomorphization
// → Generic dikompilasi jadi versi spesifik per tipe
// → Zero-cost abstraction, performa sama dengan non-generic
// ============================================
