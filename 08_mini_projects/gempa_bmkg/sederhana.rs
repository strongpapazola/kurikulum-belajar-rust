// ============================================
// VERSI SEDERHANA — TANPA DEPENDENCY SAMA SEKALI
// ============================================
// Versi Cargo (src/main.rs) memakai reqwest + serde. File ini sengaja
// dibuat tanpa library apa pun supaya bisa dijalankan dengan `rustc` biasa,
// persis seperti file materi lain di kurikulum ini.
//
// Caranya: biarkan `curl` yang mengambil data, program kita baca dari stdin.
//
//   rustc sederhana.rs -o gempa
//   curl -s https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json | ./gempa
//
// Konsep yang dipraktekkan:
//   - baca stdin (std::io)
//   - manipulasi &str: find, split, trim (modul 04)
//   - Option<T> dan operator ? di dalam fungsi yang return Option (modul 03)
//   - struct + impl (modul 01)

use std::io::Read;

// ============================================
// MINI PARSER JSON
// ============================================
// Kita TIDAK menulis parser JSON lengkap — hanya cukup untuk ambil satu
// nilai string berdasarkan nama key. Untuk data BMKG yang bentuknya datar
// dan semua nilainya String, ini sudah cukup.
//
// CATATAN: untuk project sungguhan pakailah `serde_json`. Fungsi ini
// dibuat manual murni supaya kelihatan apa yang sebenarnya terjadi.
fn ambil_field(json: &str, key: &str) -> Option<String> {
    let pola = format!("\"{}\"", key);
    let mulai = json.find(&pola)?;

    // Lompati nama key, lalu cari titik dua pemisahnya
    let sisa = &json[mulai + pola.len()..];
    let titik_dua = sisa.find(':')?;
    let sisa = &sisa[titik_dua + 1..];

    // Cari tanda kutip pembuka nilai
    let kutip_buka = sisa.find('"')?;
    let sisa = &sisa[kutip_buka + 1..];

    // Baca sampai tanda kutip penutup (hormati escape \")
    let mut nilai = String::new();
    let mut chars = sisa.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Ambil karakter setelah backslash apa adanya
                if let Some(berikut) = chars.next() {
                    nilai.push(berikut);
                }
            }
            '"' => return Some(nilai),
            lain => nilai.push(lain),
        }
    }

    None // kutip penutup tidak ketemu — JSON tidak lengkap
}

// ============================================
// STRUCT
// ============================================
#[derive(Debug)]
struct Gempa {
    tanggal: String,
    jam: String,
    magnitude: String,
    kedalaman: String,
    wilayah: String,
    lintang: String,
    bujur: String,
    potensi: String,
    dirasakan: String,
}

impl Gempa {
    fn dari_json(json: &str) -> Option<Gempa> {
        // `?` di sini bekerja pada Option: kalau salah satu field wajib
        // tidak ada, seluruh fungsi langsung mengembalikan None.
        Some(Gempa {
            tanggal: ambil_field(json, "Tanggal")?,
            jam: ambil_field(json, "Jam")?,
            magnitude: ambil_field(json, "Magnitude")?,
            kedalaman: ambil_field(json, "Kedalaman")?,
            wilayah: ambil_field(json, "Wilayah")?,
            lintang: ambil_field(json, "Lintang")?,
            bujur: ambil_field(json, "Bujur")?,
            // Field opsional: kalau tidak ada, pakai nilai default
            potensi: ambil_field(json, "Potensi").unwrap_or_else(|| "-".to_string()),
            dirasakan: ambil_field(json, "Dirasakan").unwrap_or_else(|| "-".to_string()),
        })
    }

    fn magnitudo(&self) -> f64 {
        self.magnitude.trim().parse().unwrap_or(0.0)
    }

    fn kedalaman_km(&self) -> f64 {
        self.kedalaman
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0)
    }

    fn skala(&self) -> &'static str {
        match self.magnitudo() {
            m if m < 4.0 => "🟢 Kecil",
            m if m < 5.0 => "🟡 Ringan",
            m if m < 6.0 => "🟠 Sedang",
            m if m < 7.0 => "🔴 Kuat",
            _ => "🟣 Besar",
        }
    }

    fn kategori_kedalaman(&self) -> &'static str {
        match self.kedalaman_km() {
            d if d < 60.0 => "Dangkal",
            d if d < 300.0 => "Menengah",
            _ => "Dalam",
        }
    }

    fn cetak(&self) {
        let garis = "────────────────────────────────────────────────";
        println!("\n{}", garis);
        println!("  {}  M {:.1}", self.skala(), self.magnitudo());
        println!("{}", garis);
        println!("  Waktu     : {} {}", self.tanggal, self.jam);
        println!("  Wilayah   : {}", self.wilayah);
        println!("  Koordinat : {} , {}", self.lintang, self.bujur);
        println!("  Kedalaman : {} ({})", self.kedalaman, self.kategori_kedalaman());
        println!("  Potensi   : {}", self.potensi);
        if self.dirasakan != "-" && !self.dirasakan.is_empty() {
            println!("  Dirasakan : {}", self.dirasakan);
        }
        println!("{}\n", garis);
    }
}

// ============================================
// MAIN
// ============================================
fn main() {
    let mut json = String::new();

    if let Err(e) = std::io::stdin().read_to_string(&mut json) {
        eprintln!("❌ Gagal membaca stdin: {}", e);
        std::process::exit(1);
    }

    if json.trim().is_empty() {
        eprintln!("❌ Tidak ada data masuk.");
        eprintln!();
        eprintln!("Pakai seperti ini:");
        eprintln!("  curl -s https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json | ./gempa");
        std::process::exit(1);
    }

    match Gempa::dari_json(&json) {
        Some(gempa) => {
            println!("\n🌏  GEMPA TERBARU · BMKG");
            gempa.cetak();
        }
        None => {
            eprintln!("❌ Data yang masuk bukan JSON gempa BMKG yang dikenali.");
            std::process::exit(1);
        }
    }
}

// ============================================
// TEST — jalankan dengan: rustc --test sederhana.rs && ./sederhana
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    const CONTOH: &str = r#"{"Infogempa":{"gempa":{"Tanggal":"14 Agu 2026",
        "Jam":"08:14:48 WIB","DateTime":"2026-08-14T01:14:48+00:00",
        "Coordinates":"5.36,125.34","Lintang":"5.36 LU","Bujur":"125.34 BT",
        "Magnitude":"5.3","Kedalaman":"10 km",
        "Wilayah":"195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT",
        "Potensi":"Tidak berpotensi tsunami","Dirasakan":"-",
        "Shakemap":"20260814081725.mmi.jpg"}}}"#;

    #[test]
    fn ambil_field_dasar() {
        assert_eq!(ambil_field(CONTOH, "Magnitude"), Some("5.3".to_string()));
        assert_eq!(ambil_field(CONTOH, "Kedalaman"), Some("10 km".to_string()));
    }

    #[test]
    fn field_tidak_ada_menghasilkan_none() {
        assert_eq!(ambil_field(CONTOH, "TidakAda"), None);
    }

    #[test]
    fn escape_backslash_ditangani() {
        let json = r#"{"Wilayah":"laut 25 km \"selatan\" Bandung"}"#;
        assert_eq!(
            ambil_field(json, "Wilayah"),
            Some("laut 25 km \"selatan\" Bandung".to_string())
        );
    }

    #[test]
    fn parse_gempa_lengkap() {
        let g = Gempa::dari_json(CONTOH).expect("harus berhasil di-parse");
        assert_eq!(g.magnitudo(), 5.3);
        assert_eq!(g.kedalaman_km(), 10.0);
        assert_eq!(g.kategori_kedalaman(), "Dangkal");
        assert_eq!(g.skala(), "🟠 Sedang");
    }

    #[test]
    fn json_ngawur_menghasilkan_none() {
        assert!(Gempa::dari_json("{ bukan json }").is_none());
    }
}
