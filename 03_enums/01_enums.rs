// ============================================
// ENUMS — Jauh lebih powerful dari enum di bahasa lain
// ============================================
// Jalankan: rustc 01_enums.rs && ./01_enums

// Enum biasa — mirip enum di PHP/Java
#[derive(Debug)]
enum StatusFaktur {
    Draft,
    Dikirim,
    Dibayar,
    Dibatalkan,
}

// Enum dengan data — ini yang unik di Rust!
// Tiap variant bisa punya data berbeda
#[derive(Debug)]
enum Pesan {
    Teks(String),
    Angka(i32),
    Koordinat { lat: f64, lng: f64 },  // struct-like variant
    Kosong,
}

// Enum untuk domain pajak.io
#[derive(Debug)]
enum JenisPajak {
    PPh21 { tarif: f64 },
    PPh23 { tarif: f64, objek: String },
    PPN,
    PPnBM { tarif: f64 },
}

impl JenisPajak {
    fn tarif(&self) -> f64 {
        match self {
            JenisPajak::PPh21 { tarif } => *tarif,
            JenisPajak::PPh23 { tarif, .. } => *tarif,
            JenisPajak::PPN => 0.11,
            JenisPajak::PPnBM { tarif } => *tarif,
        }
    }

    fn nama(&self) -> &str {
        match self {
            JenisPajak::PPh21 { .. } => "PPh Pasal 21",
            JenisPajak::PPh23 { .. } => "PPh Pasal 23",
            JenisPajak::PPN          => "Pajak Pertambahan Nilai",
            JenisPajak::PPnBM { .. } => "PPnBM",
        }
    }
}

fn main() {
    // ==================
    // Enum biasa
    // ==================
    let status = StatusFaktur::Dikirim;
    println!("Status: {:?}", status);

    match status {
        StatusFaktur::Draft      => println!("Masih draft"),
        StatusFaktur::Dikirim    => println!("Sudah dikirim ke pembeli"),
        StatusFaktur::Dibayar    => println!("Sudah dibayar"),
        StatusFaktur::Dibatalkan => println!("Dibatalkan"),
    }

    // ==================
    // Enum dengan data
    // ==================
    let pesan1 = Pesan::Teks(String::from("Halo dari e-Faktur"));
    let pesan2 = Pesan::Angka(42);
    let pesan3 = Pesan::Koordinat { lat: -6.2, lng: 106.8 };
    let pesan4 = Pesan::Kosong;

    for p in [&pesan1, &pesan2, &pesan3, &pesan4] {
        match p {
            Pesan::Teks(s)              => println!("Teks: {s}"),
            Pesan::Angka(n)             => println!("Angka: {n}"),
            Pesan::Koordinat { lat, lng } => println!("Lokasi: ({lat}, {lng})"),
            Pesan::Kosong               => println!("(kosong)"),
        }
    }

    // ==================
    // Enum pajak
    // ==================
    let pajak1 = JenisPajak::PPh21 { tarif: 0.05 };
    let pajak2 = JenisPajak::PPh23 {
        tarif: 0.02,
        objek: String::from("Jasa konsultansi"),
    };
    let pajak3 = JenisPajak::PPN;

    let daftar = [&pajak1, &pajak2, &pajak3];
    for p in &daftar {
        println!("{}: {:.0}%", p.nama(), p.tarif() * 100.0);
    }

    // ==================
    // if let — pattern matching ringkas untuk satu case
    // ==================
    let pajak = JenisPajak::PPh23 {
        tarif: 0.02,
        objek: String::from("Sewa"),
    };

    if let JenisPajak::PPh23 { tarif, objek } = &pajak {
        println!("\nPPh 23 untuk '{objek}': {:.0}%", tarif * 100.0);
    }
}

// ============================================
// Perbandingan:
//
// PHP:    enum Status { Draft, Sent, Paid }  (PHP 8.1+, basic)
// Java:   enum Status { DRAFT, SENT, PAID }  (tidak bisa bawa data)
// Python: class Status(Enum): DRAFT = 1      (tidak bisa bawa data)
//
// Rust:   enum bisa bawa data berbeda per variant
//         + match harus exhaustive
//         → Jauh lebih expressive!
// ============================================
