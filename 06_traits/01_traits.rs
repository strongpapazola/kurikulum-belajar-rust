// ============================================
// 01 - Traits di Rust
// ============================================
// Jalankan: rustc 01_traits.rs && ./01_traits
//
// Trait = kontrak behavior yang harus dipenuhi suatu type
// Mirip interface di PHP/Java, tapi lebih powerful

use std::fmt;

// ==================
// DEFINISI TRAIT
// ==================
trait HitungPajak {
    // Method yang WAJIB diimplementasikan
    fn tarif(&self) -> f64;
    fn nama(&self) -> &str;

    // Method dengan DEFAULT implementasi (tidak wajib di-override)
    fn hitung(&self, dpp: f64) -> f64 {
        dpp * self.tarif()
    }

    fn cetak_info(&self, dpp: f64) {
        println!(
            "{}: DPP={:.0} | Tarif={:.0}% | Pajak={:.0}",
            self.nama(),
            dpp,
            self.tarif() * 100.0,
            self.hitung(dpp)
        );
    }
}

// ==================
// STRUCT + IMPL TRAIT
// ==================
struct PPN;

impl HitungPajak for PPN {
    fn tarif(&self) -> f64 { 0.11 }
    fn nama(&self) -> &str { "PPN" }
}

struct PPh21 {
    nama_wajib: String,
    ptkp: f64,
}

impl HitungPajak for PPh21 {
    fn tarif(&self) -> f64 { 0.05 } // tarif bracket pertama

    fn nama(&self) -> &str { "PPh Pasal 21" }

    // Override default implementation
    fn hitung(&self, penghasilan_bruto: f64) -> f64 {
        let pkp = (penghasilan_bruto - self.ptkp).max(0.0);
        match pkp as u64 {
            0..=60_000_000          => pkp * 0.05,
            60_000_001..=250_000_000 => 3_000_000.0 + (pkp - 60_000_000.0) * 0.15,
            _                       => 31_500_000.0 + (pkp - 250_000_000.0) * 0.25,
        }
    }
}

struct PPh23 {
    objek: String,
    tarif_persen: f64,
}

impl HitungPajak for PPh23 {
    fn tarif(&self) -> f64 { self.tarif_persen / 100.0 }
    fn nama(&self) -> &str { "PPh Pasal 23" }
}

// ==================
// TRAIT SEBAGAI PARAMETER
// ==================

// Cara 1: impl Trait syntax (lebih readable)
fn cetak_pajak(pajak: &impl HitungPajak, dpp: f64) {
    pajak.cetak_info(dpp);
}

// Cara 2: Trait Bound syntax (lebih flexible untuk generic)
fn cetak_pajak_generic<T: HitungPajak>(pajak: &T, dpp: f64) {
    pajak.cetak_info(dpp);
}

// Multiple trait bound
fn cetak_detail<T: HitungPajak + fmt::Debug>(pajak: &T, dpp: f64) {
    println!("{:?}", pajak);
    pajak.cetak_info(dpp);
}

// ==================
// TRAIT OBJECT — dynamic dispatch (seperti interface PHP)
// ==================
fn proses_semua(pajak_list: &[Box<dyn HitungPajak>], dpp: f64) {
    println!("\n=== Semua Pajak untuk DPP {:.0} ===", dpp);
    let mut total = 0.0;
    for pajak in pajak_list {
        let bayar = pajak.hitung(dpp);
        println!("  {}: Rp {:.0}", pajak.nama(), bayar);
        total += bayar;
    }
    println!("  Total pajak: Rp {:.0}", total);
}

// ==================
// TRAIT RETURN TYPE
// ==================
fn buat_pajak(jenis: &str) -> Box<dyn HitungPajak> {
    match jenis {
        "ppn"   => Box::new(PPN),
        "pph23" => Box::new(PPh23 {
            objek: "Jasa konsultansi".to_string(),
            tarif_persen: 2.0,
        }),
        _       => Box::new(PPN), // default
    }
}

// ==================
// STANDARD TRAITS — yang built-in di Rust
// ==================
#[derive(Debug, Clone, PartialEq)]
struct Faktur {
    nomor: String,
    total: f64,
}

// Implement Display (untuk println! dengan {})
impl fmt::Display for Faktur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Faktur[{}] = Rp {:.0}", self.nomor, self.total)
    }
}

// Implement Default
impl Default for Faktur {
    fn default() -> Self {
        Faktur {
            nomor: String::from("FKT-000"),
            total: 0.0,
        }
    }
}

impl PPh21 {
    fn new(nama: &str, menikah: bool) -> Self {
        PPh21 {
            nama_wajib: nama.to_string(),
            ptkp: if menikah { 58_500_000.0 } else { 54_000_000.0 },
        }
    }
}

impl fmt::Debug for PPh21 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPh21({}, PTKP={})", self.nama_wajib, self.ptkp)
    }
}

fn main() {
    println!("=== Traits ===\n");

    // Pakai trait
    let ppn   = PPN;
    let pph21 = PPh21::new("Budi Santoso", true);
    let pph23 = PPh23 { objek: "Sewa".to_string(), tarif_persen: 2.0 };

    let dpp = 10_000_000.0_f64;

    cetak_pajak(&ppn, dpp);
    cetak_pajak(&pph21, dpp * 12.0); // annual
    cetak_pajak(&pph23, dpp);

    // Trait object — Vec bisa campur berbagai type!
    let pajak_list: Vec<Box<dyn HitungPajak>> = vec![
        Box::new(PPN),
        Box::new(PPh23 { objek: "Jasa".to_string(), tarif_persen: 2.0 }),
    ];
    proses_semua(&pajak_list, dpp);

    // Dynamic dispatch
    let p = buat_pajak("ppn");
    println!("\nbuat_pajak: {} = {:.0}", p.nama(), p.hitung(5_000_000.0));

    // Standard traits
    println!("\n=== Standard Traits ===");
    let f1 = Faktur { nomor: "FKT-001".to_string(), total: 1_500_000.0 };
    let f2 = f1.clone();           // Clone
    println!("Display : {f1}");
    println!("Debug   : {f1:?}");
    println!("Equal   : {}", f1 == f2); // PartialEq

    let f_default = Faktur::default();
    println!("Default : {f_default}");
}

// ============================================
// Trait vs Interface:
//
// PHP:    interface HitungPajak { public function tarif(): float; }
//         class PPN implements HitungPajak { public function tarif() { return 0.11; } }
//
// Java:   interface HitungPajak { double tarif(); }
//         class PPN implements HitungPajak { public double tarif() { return 0.11; } }
//
// Rust:   trait HitungPajak { fn tarif(&self) -> f64; }
//         impl HitungPajak for PPN { fn tarif(&self) -> f64 { 0.11 } }
//
// Keunggulan Rust:
// - Bisa impl trait untuk type dari library lain (orphan rule)
// - Default method implementation
// - Trait bound di compile time (zero-cost abstraction)
// ============================================
