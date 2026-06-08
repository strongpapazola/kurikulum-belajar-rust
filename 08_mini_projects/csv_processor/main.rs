// ============================================
// MINI PROJECT: CSV Processor untuk e-Faktur
// ============================================
// Simulasi baca CSV, validasi, hitung PPN, output CSV baru
//
// Untuk run sebagai project nyata:
//   cargo new csv-efaktur
//   copy ke src/main.rs
//   Cargo.toml: csv = "1.3", serde = { version = "1", features = ["derive"] }

use std::collections::HashMap;

// Struct untuk satu baris faktur
#[derive(Debug, Clone)]
struct FakturItem {
    nomor: String,
    npwp_pembeli: String,
    nama_pembeli: String,
    barang: String,
    qty: u32,
    harga_satuan: f64,
}

impl FakturItem {
    fn dpp(&self) -> f64 {
        self.qty as f64 * self.harga_satuan
    }

    fn ppn(&self) -> f64 {
        self.dpp() * 0.11
    }

    fn total(&self) -> f64 {
        self.dpp() + self.ppn()
    }
}

// Validasi NPWP sederhana
fn validasi_npwp(npwp: &str) -> Result<(), String> {
    let digit: String = npwp.chars().filter(|c| c.is_ascii_digit()).collect();
    if digit.len() != 15 {
        return Err(format!("NPWP '{}' tidak valid (harus 15 digit)", npwp));
    }
    Ok(())
}

// Parse CSV string jadi Vec<FakturItem>
fn parse_csv(csv_content: &str) -> (Vec<FakturItem>, Vec<String>) {
    let mut items = Vec::new();
    let mut errors = Vec::new();

    for (i, line) in csv_content.lines().enumerate() {
        if i == 0 { continue; } // skip header

        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() != 6 {
            errors.push(format!("Baris {}: kolom tidak lengkap", i + 1));
            continue;
        }

        // Validasi NPWP
        if let Err(e) = validasi_npwp(cols[1]) {
            errors.push(format!("Baris {}: {}", i + 1, e));
            continue;
        }

        // Parse qty dan harga
        let qty = match cols[4].trim().parse::<u32>() {
            Ok(q) => q,
            Err(_) => {
                errors.push(format!("Baris {}: qty tidak valid '{}'", i + 1, cols[4]));
                continue;
            }
        };

        let harga = match cols[5].trim().parse::<f64>() {
            Ok(h) => h,
            Err(_) => {
                errors.push(format!("Baris {}: harga tidak valid '{}'", i + 1, cols[5]));
                continue;
            }
        };

        items.push(FakturItem {
            nomor:         cols[0].trim().to_string(),
            npwp_pembeli:  cols[1].trim().to_string(),
            nama_pembeli:  cols[2].trim().to_string(),
            barang:        cols[3].trim().to_string(),
            qty,
            harga_satuan:  harga,
        });
    }

    (items, errors)
}

// Generate output CSV e-Faktur
fn generate_output_csv(items: &[FakturItem]) -> String {
    let mut output = String::from("Nomor,NPWP Pembeli,Nama Pembeli,Barang,Qty,Harga,DPP,PPN,Total\n");

    for item in items {
        output.push_str(&format!(
            "{},{},{},{},{},{:.0},{:.0},{:.0},{:.0}\n",
            item.nomor,
            item.npwp_pembeli,
            item.nama_pembeli,
            item.barang,
            item.qty,
            item.harga_satuan,
            item.dpp(),
            item.ppn(),
            item.total(),
        ));
    }

    output
}

// Summary per pembeli
fn summary_per_pembeli(items: &[FakturItem]) -> HashMap<String, (u32, f64)> {
    let mut summary: HashMap<String, (u32, f64)> = HashMap::new();

    for item in items {
        let entry = summary
            .entry(item.nama_pembeli.clone())
            .or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += item.total();
    }

    summary
}

fn main() {
    // Simulasi data CSV input
    let csv_input = "\
nomor,npwp_pembeli,nama_pembeli,barang,qty,harga
FKT-001,12.345.678.9-012.345,PT Maju Jaya,Laptop,2,15000000
FKT-002,98.765.432.1-098.765,CV Sejahtera,Printer,1,3500000
FKT-003,INVALID_NPWP,PT Error,Mouse,5,150000
FKT-004,11.222.333.4-567.890,PT Berkah,Monitor,3,4500000
FKT-005,12.345.678.9-012.345,PT Maju Jaya,Keyboard,10,350000
FKT-006,55.666.777.8-999.INVALID,PT Bad,RAM,2,500000";

    println!("=== CSV Processor e-Faktur ===\n");
    println!("Input CSV:\n{}\n", csv_input);

    // Parse
    let (items, errors) = parse_csv(csv_input);

    // Tampilkan errors
    if !errors.is_empty() {
        println!("=== ERROR VALIDASI ===");
        for e in &errors {
            println!("❌ {e}");
        }
        println!();
    }

    // Tampilkan item valid
    println!("=== FAKTUR VALID ({} item) ===", items.len());
    for item in &items {
        println!("✅ {} | {} | {} | DPP: {:.0} | PPN: {:.0} | Total: {:.0}",
            item.nomor,
            item.nama_pembeli,
            item.barang,
            item.dpp(),
            item.ppn(),
            item.total()
        );
    }

    // Generate output
    println!("\n=== OUTPUT CSV ===");
    let output = generate_output_csv(&items);
    println!("{}", output);

    // Summary
    println!("=== SUMMARY PER PEMBELI ===");
    let summary = summary_per_pembeli(&items);
    let mut summary_vec: Vec<_> = summary.iter().collect();
    summary_vec.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap()); // sort by total

    for (pembeli, (jumlah, total)) in &summary_vec {
        println!("{}: {} faktur, Total Rp {:.0}", pembeli, jumlah, total);
    }

    // Grand total
    let grand_total: f64 = items.iter().map(|i| i.total()).sum();
    let grand_ppn: f64 = items.iter().map(|i| i.ppn()).sum();
    println!("\n=== GRAND TOTAL ===");
    println!("Total PPN    : Rp {:.0}", grand_ppn);
    println!("Grand Total  : Rp {:.0}", grand_total);
    println!("Valid        : {} / {} baris", items.len(), items.len() + errors.len());
}
