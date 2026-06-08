// ============================================
// 06 - Structs (mirip Class di PHP/JS/Python)
// ============================================
// Jalankan: rustc 06_structs.rs && ./06_structs

// Definisi struct — mirip class tapi tanpa method di dalamnya
#[derive(Debug)] // supaya bisa di-print dengan {:?}
struct Faktur {
    id: u32,
    nomor: String,
    nama_pembeli: String,
    total: f64,
    lunas: bool,
}

// Method ditaruh di blok impl terpisah
impl Faktur {
    // Constructor (convention: fn new)
    fn new(id: u32, nomor: &str, nama_pembeli: &str, total: f64) -> Faktur {
        Faktur {
            id,
            nomor: String::from(nomor),
            nama_pembeli: String::from(nama_pembeli),
            total,
            lunas: false,
        }
    }

    // Method — &self = borrow immutable (tidak ubah data)
    fn tampilkan(&self) {
        println!("=== Faktur #{} ===", self.id);
        println!("Nomor  : {}", self.nomor);
        println!("Pembeli: {}", self.nama_pembeli);
        println!("Total  : Rp {:.0}", self.total);
        println!("Status : {}", if self.lunas { "LUNAS" } else { "BELUM LUNAS" });
    }

    // Method yang menghitung sesuatu
    fn ppn(&self) -> f64 {
        self.total * 0.11
    }

    fn total_dengan_ppn(&self) -> f64 {
        self.total + self.ppn()
    }

    // Mutable method — &mut self = bisa ubah data
    fn bayar(&mut self) {
        self.lunas = true;
        println!("Faktur {} sudah dibayar!", self.nomor);
    }
}

// Struct tuple — lebih ringkas, akses pakai .0, .1, .2
#[derive(Debug)]
struct Koordinat(f64, f64);

// Unit struct — tanpa field, untuk marker/trait
struct DummyStruct;

fn main() {
    // Buat instance
    let mut faktur1 = Faktur::new(1, "FKT-001", "PT Maju Jaya", 1_000_000.0);

    // Akses field
    println!("ID: {}", faktur1.id);
    println!("Total: {}", faktur1.total);

    // Panggil method
    faktur1.tampilkan();
    println!("PPN: Rp {:.0}", faktur1.ppn());
    println!("Total + PPN: Rp {:.0}", faktur1.total_dengan_ppn());

    // Mutable method
    faktur1.bayar();
    faktur1.tampilkan();

    // Debug print
    println!("{:?}", faktur1);
    println!("{:#?}", faktur1); // pretty print

    // Struct update syntax — buat instance baru dari yang lama
    let faktur2 = Faktur {
        id: 2,
        nomor: String::from("FKT-002"),
        nama_pembeli: String::from("CV Sejahtera"),
        ..faktur1 // ambil sisa field dari faktur1
        // Catatan: faktur1 tidak bisa dipakai lagi setelah ini
        // karena field String sudah di-move
    };
    println!("\nFaktur 2: {:#?}", faktur2);

    // Tuple struct
    let lokasi = Koordinat(106.845_599, -6.208_763); // Jakarta
    println!("\nLokasi: ({}, {})", lokasi.0, lokasi.1);
}

// ============================================
// Perbandingan dengan OOP:
//
// PHP:
//   class Faktur {
//     public function __construct(public int $id) {}
//     public function tampilkan() { echo $this->id; }
//   }
//
// Python:
//   class Faktur:
//     def __init__(self, id): self.id = id
//     def tampilkan(self): print(self.id)
//
// Rust:
//   struct Faktur { id: u32 }
//   impl Faktur {
//     fn tampilkan(&self) { println!("{}", self.id); }
//   }
//
// Key: Rust memisahkan data (struct) dan behavior (impl)
// ============================================
