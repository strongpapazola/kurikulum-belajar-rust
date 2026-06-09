// ============================================
// MINI PROJECT: CLI Kalkulator Pajak
// ============================================
// Cara jalankan (tanpa Cargo, input dari args):
//   rustc main.rs && ./main ppn 5000000
//   rustc main.rs && ./main pph21 120000000 menikah
//   rustc main.rs && ./main pph23 10000000 jasa
//
// Atau dengan Cargo:
//   cargo new cli-pajak && cd cli-pajak
//   copy ke src/main.rs
//   cargo run -- ppn 5000000

use std::env;

// ==================
// DOMAIN TYPES
// ==================
#[derive(Debug)]
enum JenisPajak {
    PPN,
    PPh21 { menikah: bool },
    PPh23 { objek: String },
}

#[derive(Debug)]
struct HasilKalkulasi {
    jenis: String,
    dpp: f64,
    pajak: f64,
    total: f64,
    keterangan: String,
}

impl HasilKalkulasi {
    fn cetak(&self) {
        let separator = "─".repeat(40);
        println!("\n{separator}");
        println!("  Jenis Pajak : {}", self.jenis);
        println!("  DPP         : Rp {}", format_rupiah(self.dpp));
        println!("  Pajak       : Rp {}", format_rupiah(self.pajak));
        println!("  Total Bayar : Rp {}", format_rupiah(self.total));
        if !self.keterangan.is_empty() {
            println!("  Keterangan  : {}", self.keterangan);
        }
        println!("{separator}");
    }
}

// ==================
// KALKULASI
// ==================
fn hitung_ppn(dpp: f64) -> HasilKalkulasi {
    let ppn   = dpp * 0.11;
    let total = dpp + ppn;
    HasilKalkulasi {
        jenis: "PPN (11%)".to_string(),
        dpp,
        pajak: ppn,
        total,
        keterangan: "Tarif PPN berlaku mulai April 2022".to_string(),
    }
}

fn hitung_pph21(penghasilan_setahun: f64, menikah: bool) -> HasilKalkulasi {
    let ptkp = if menikah { 58_500_000.0 } else { 54_000_000.0 };
    let pkp  = (penghasilan_setahun - ptkp).max(0.0);

    let pajak_tahunan = hitung_tarif_progresif(pkp);
    let pajak_bulanan = pajak_tahunan / 12.0;

    HasilKalkulasi {
        jenis: "PPh Pasal 21".to_string(),
        dpp: penghasilan_setahun,
        pajak: pajak_tahunan,
        total: penghasilan_setahun, // take home pay beda
        keterangan: format!(
            "PTKP: Rp {} | PKP: Rp {} | PPh/bulan: Rp {}",
            format_rupiah(ptkp),
            format_rupiah(pkp),
            format_rupiah(pajak_bulanan)
        ),
    }
}

fn hitung_tarif_progresif(pkp: f64) -> f64 {
    let mut pajak = 0.0;

    // Bracket 1: 0 - 60jt = 5%
    let bracket1 = pkp.min(60_000_000.0);
    pajak += bracket1 * 0.05;
    if pkp <= 60_000_000.0 { return pajak; }

    // Bracket 2: 60jt - 250jt = 15%
    let bracket2 = (pkp - 60_000_000.0).min(190_000_000.0);
    pajak += bracket2 * 0.15;
    if pkp <= 250_000_000.0 { return pajak; }

    // Bracket 3: 250jt - 500jt = 25%
    let bracket3 = (pkp - 250_000_000.0).min(250_000_000.0);
    pajak += bracket3 * 0.25;
    if pkp <= 500_000_000.0 { return pajak; }

    // Bracket 4: > 500jt = 30%
    pajak += (pkp - 500_000_000.0) * 0.30;
    pajak
}

fn hitung_pph23(dpp: f64, objek: &str) -> HasilKalkulasi {
    let tarif = match objek.to_lowercase().as_str() {
        "jasa" | "jasa_lain"    => 0.02,
        "sewa" | "sewa_tanah"   => 0.02,
        "dividen"               => 0.15,
        "bunga" | "royalti"     => 0.15,
        _                       => 0.02, // default
    };

    let pajak = dpp * tarif;
    HasilKalkulasi {
        jenis: format!("PPh Pasal 23 - {objek}"),
        dpp,
        pajak,
        total: dpp - pajak, // netto yang diterima
        keterangan: format!(
            "Tarif: {:.0}% | Dipotong oleh pemberi kerja",
            tarif * 100.0
        ),
    }
}

// ==================
// HELPER
// ==================
fn format_rupiah(n: f64) -> String {
    let n_int = n as u64;
    let s = n_int.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push('.'); }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn parse_jenis(args: &[String]) -> Result<(JenisPajak, f64), String> {
    if args.len() < 3 {
        return Err(format!(
            "Usage:\n  {} ppn <nominal>\n  {} pph21 <gaji_tahunan> [menikah]\n  {} pph23 <nominal> [objek]",
            args[0], args[0], args[0]
        ));
    }

    let nominal: f64 = args[2].replace('.', "").replace(',', "")
        .parse()
        .map_err(|_| format!("Nominal '{}' bukan angka valid", args[2]))?;

    if nominal <= 0.0 {
        return Err("Nominal harus lebih dari 0".to_string());
    }

    let jenis = match args[1].to_lowercase().as_str() {
        "ppn" => JenisPajak::PPN,
        "pph21" => {
            let menikah = args.get(3).map_or(false, |s| s == "menikah");
            JenisPajak::PPh21 { menikah }
        }
        "pph23" => {
            let objek = args.get(3).cloned().unwrap_or_else(|| "jasa".to_string());
            JenisPajak::PPh23 { objek }
        }
        lain => return Err(format!("Jenis pajak '{}' tidak dikenal (ppn/pph21/pph23)", lain)),
    };

    Ok((jenis, nominal))
}

fn tampilkan_bantuan(nama_program: &str) {
    println!("\n🧾 Kalkulator Pajak CLI\n");
    println!("Usage:");
    println!("  {nama_program} ppn <nominal>");
    println!("  {nama_program} pph21 <gaji_tahunan> [menikah]");
    println!("  {nama_program} pph23 <nominal> [objek]\n");
    println!("Contoh:");
    println!("  {nama_program} ppn 5000000");
    println!("  {nama_program} pph21 120000000 menikah");
    println!("  {nama_program} pph23 10000000 jasa");
    println!("  {nama_program} pph23 10000000 dividen\n");
    println!("Objek PPh 23: jasa, sewa, dividen, bunga, royalti");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Jalankan demo kalau tidak ada argumen
    if args.len() < 2 {
        println!("🧾 Demo Kalkulator Pajak\n");

        println!("--- PPN ---");
        hitung_ppn(5_000_000.0).cetak();

        println!("\n--- PPh 21 (menikah, gaji 10jt/bulan) ---");
        hitung_pph21(120_000_000.0, true).cetak();

        println!("\n--- PPh 23 - Jasa ---");
        hitung_pph23(10_000_000.0, "jasa").cetak();

        println!("\n--- PPh 23 - Dividen ---");
        hitung_pph23(50_000_000.0, "dividen").cetak();

        println!("\n\nJalankan dengan argumen untuk kalkulator interaktif:");
        tampilkan_bantuan(&args[0]);
        return;
    }

    if args[1] == "--help" || args[1] == "-h" {
        tampilkan_bantuan(&args[0]);
        return;
    }

    match parse_jenis(&args) {
        Ok((jenis, nominal)) => {
            let hasil = match jenis {
                JenisPajak::PPN                 => hitung_ppn(nominal),
                JenisPajak::PPh21 { menikah }   => hitung_pph21(nominal, menikah),
                JenisPajak::PPh23 { ref objek } => hitung_pph23(nominal, objek),
            };
            hasil.cetak();
        }
        Err(e) => {
            eprintln!("❌ Error: {e}");
            std::process::exit(1);
        }
    }
}
