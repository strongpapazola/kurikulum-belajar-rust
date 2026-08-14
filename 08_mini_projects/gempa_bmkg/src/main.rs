// ============================================
// MINI PROJECT: Monitor Gempa BMKG
// ============================================
// Studi kasus ambil data real dari API publik BMKG:
//   https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json
//
// Setup:
//   cd 08_mini_projects/gempa_bmkg
//   cargo run
//
// Contoh pemakaian:
//   cargo run                          # gempa terbaru (default)
//   cargo run -- terkini               # 15 gempa terkini M >= 5.0
//   cargo run -- dirasakan             # 15 gempa yang dirasakan warga
//   cargo run -- terkini --min-mag 5.5 # filter magnitudo
//   cargo run -- terbaru --json        # output JSON (buat piping ke jq)
//   cargo run -- terbaru --watch 60    # polling tiap 60 detik
//
// Konsep yang dipraktekkan:
//   - async/await + tokio        (modul 07)
//   - Result<T, E> & custom error (modul 05)
//   - enum + match + Option       (modul 03)
//   - Vec, HashMap, iterator      (modul 04)
//   - struct + impl + trait Display (modul 01 & 06)

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::Duration;

const BASE_URL: &str = "https://data.bmkg.go.id/DataMKG/TEWS";

// ============================================
// 1. SUMBER DATA
// ============================================
// BMKG menyediakan 3 endpoint dengan bentuk JSON yang berbeda:
// - autogempa.json      -> Infogempa.gempa berupa OBJEK (1 gempa)
// - gempaterkini.json   -> Infogempa.gempa berupa ARRAY (15 gempa M>=5.0)
// - gempadirasakan.json -> Infogempa.gempa berupa ARRAY (15 gempa dirasakan)
#[derive(Debug, Clone, Copy, PartialEq)]
enum Sumber {
    Terbaru,
    Terkini,
    Dirasakan,
}

impl Sumber {
    fn url(&self) -> String {
        let file = match self {
            Sumber::Terbaru => "autogempa.json",
            Sumber::Terkini => "gempaterkini.json",
            Sumber::Dirasakan => "gempadirasakan.json",
        };
        format!("{}/{}", BASE_URL, file)
    }

    fn judul(&self) -> &'static str {
        match self {
            Sumber::Terbaru => "GEMPA TERBARU",
            Sumber::Terkini => "15 GEMPA TERKINI (M ≥ 5.0)",
            Sumber::Dirasakan => "15 GEMPA DIRASAKAN",
        }
    }

    // autogempa.json bentuknya objek tunggal, dua lainnya array
    fn objek_tunggal(&self) -> bool {
        matches!(self, Sumber::Terbaru)
    }
}

// ============================================
// 2. CUSTOM ERROR
// ============================================
// Semua kemungkinan gagal dikumpulkan jadi satu enum.
// Ini pola yang sama seperti di 05_error_handling/03_custom_error.rs
#[derive(Debug)]
enum GempaError {
    Jaringan(reqwest::Error),
    Parse(serde_json::Error),
    StatusHttp(u16),
    Argumen(String),
}

impl fmt::Display for GempaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GempaError::Jaringan(e) => write!(f, "Gagal menghubungi server BMKG: {}", e),
            GempaError::Parse(e) => write!(f, "Format JSON dari BMKG tidak sesuai: {}", e),
            GempaError::StatusHttp(kode) => write!(f, "Server BMKG membalas dengan status HTTP {}", kode),
            GempaError::Argumen(pesan) => write!(f, "Argumen salah: {}", pesan),
        }
    }
}

impl std::error::Error for GempaError {}

// `From` bikin operator `?` otomatis konversi error dari library ke error kita
impl From<reqwest::Error> for GempaError {
    fn from(e: reqwest::Error) -> Self {
        GempaError::Jaringan(e)
    }
}

impl From<serde_json::Error> for GempaError {
    fn from(e: serde_json::Error) -> Self {
        GempaError::Parse(e)
    }
}

// ============================================
// 3. STRUCT HASIL PARSING JSON
// ============================================
// Perhatikan: BMKG memakai key PascalCase ("Tanggal", "Magnitude"),
// sementara konvensi Rust snake_case. `#[serde(rename)]` menjembatani.
//
// Catatan penting: SEMUA nilai dari BMKG bertipe String, termasuk angka
// ("Magnitude": "5.3"). Jadi konversi ke f64 kita lakukan sendiri di impl.
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "Jam")]
    jam: String,
    #[serde(rename = "DateTime")]
    date_time: String,
    #[serde(rename = "Coordinates")]
    coordinates: String,
    #[serde(rename = "Lintang")]
    lintang: String,
    #[serde(rename = "Bujur")]
    bujur: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Kedalaman")]
    kedalaman: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,

    // Field di bawah ini tidak selalu ada di semua endpoint,
    // makanya dibungkus Option<T> supaya parsing tidak gagal.
    #[serde(rename = "Potensi")]
    potensi: Option<String>,
    #[serde(rename = "Dirasakan")]
    dirasakan: Option<String>,
    #[serde(rename = "Shakemap")]
    shakemap: Option<String>,
}

// Struktur pembungkus untuk autogempa.json (objek tunggal)
#[derive(Debug, Deserialize)]
struct ResponseTunggal {
    #[serde(rename = "Infogempa")]
    infogempa: WrapperTunggal,
}

#[derive(Debug, Deserialize)]
struct WrapperTunggal {
    gempa: Gempa,
}

// Struktur pembungkus untuk gempaterkini.json & gempadirasakan.json (array)
#[derive(Debug, Deserialize)]
struct ResponseList {
    #[serde(rename = "Infogempa")]
    infogempa: WrapperList,
}

#[derive(Debug, Deserialize)]
struct WrapperList {
    gempa: Vec<Gempa>,
}

// ============================================
// 4. LOGIKA DOMAIN
// ============================================
impl Gempa {
    // "5.3" -> 5.3 ; kalau gagal parse, anggap 0.0
    fn magnitudo(&self) -> f64 {
        self.magnitude.trim().parse().unwrap_or(0.0)
    }

    // "10 km" -> 10.0
    fn kedalaman_km(&self) -> f64 {
        self.kedalaman
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0)
    }

    // "5.36,125.34" -> (5.36, 125.34)
    fn koordinat(&self) -> Option<(f64, f64)> {
        let (lat, lon) = self.coordinates.split_once(',')?;
        let lat = lat.trim().parse().ok()?;
        let lon = lon.trim().parse().ok()?;
        Some((lat, lon))
    }

    fn skala(&self) -> Skala {
        Skala::dari_magnitudo(self.magnitudo())
    }

    fn kategori_kedalaman(&self) -> &'static str {
        match self.kedalaman_km() {
            d if d < 60.0 => "Dangkal",
            d if d < 300.0 => "Menengah",
            _ => "Dalam",
        }
    }

    // BMKG mengisi "-" kalau gempa tidak dirasakan. Ubah jadi None.
    fn dirasakan_bersih(&self) -> Option<&str> {
        let nilai = self.dirasakan.as_deref()?.trim();
        if nilai.is_empty() || nilai == "-" {
            None
        } else {
            Some(nilai)
        }
    }

    fn tsunami(&self) -> bool {
        self.potensi
            .as_deref()
            .map(|p| p.to_lowercase().contains("berpotensi") && !p.to_lowercase().contains("tidak"))
            .unwrap_or(false)
    }

    // Gambar peta guncangan (hanya tersedia di autogempa.json)
    fn shakemap_url(&self) -> Option<String> {
        let file = self.shakemap.as_deref()?;
        Some(format!("{}/{}", BASE_URL, file))
    }

    fn google_maps_url(&self) -> Option<String> {
        let (lat, lon) = self.koordinat()?;
        Some(format!("https://www.google.com/maps?q={},{}", lat, lon))
    }

    // Endpoint gempadirasakan.json selalu diawali "Pusat gempa berada di ..."
    // Buang awalan itu supaya kolom tabel tidak habis untuk teks yang sama.
    fn wilayah_ringkas(&self) -> &str {
        self.wilayah
            .strip_prefix("Pusat gempa berada di ")
            .unwrap_or(&self.wilayah)
    }

    // Kunci unik untuk deduplikasi — BMKG kadang mengirim entri kembar
    fn kunci(&self) -> String {
        format!("{}|{}", self.date_time, self.coordinates)
    }
}

// Klasifikasi magnitudo versi sederhana
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Skala {
    Kecil,    // < 4.0
    Ringan,   // 4.0 – 4.9
    Sedang,   // 5.0 – 5.9
    Kuat,     // 6.0 – 6.9
    Besar,    // >= 7.0
}

impl Skala {
    fn dari_magnitudo(m: f64) -> Skala {
        match m {
            m if m < 4.0 => Skala::Kecil,
            m if m < 5.0 => Skala::Ringan,
            m if m < 6.0 => Skala::Sedang,
            m if m < 7.0 => Skala::Kuat,
            _ => Skala::Besar,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Skala::Kecil => "Kecil",
            Skala::Ringan => "Ringan",
            Skala::Sedang => "Sedang",
            Skala::Kuat => "Kuat",
            Skala::Besar => "Besar",
        }
    }

    fn ikon(&self) -> &'static str {
        match self {
            Skala::Kecil => "🟢",
            Skala::Ringan => "🟡",
            Skala::Sedang => "🟠",
            Skala::Kuat => "🔴",
            Skala::Besar => "🟣",
        }
    }
}

impl fmt::Display for Skala {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.ikon(), self.label())
    }
}

// ============================================
// 5. AMBIL & PARSE DATA
// ============================================
async fn ambil_gempa(klien: &reqwest::Client, sumber: Sumber) -> Result<Vec<Gempa>, GempaError> {
    let response = klien.get(sumber.url()).send().await?;

    let status = response.status();
    if !status.is_success() {
        return Err(GempaError::StatusHttp(status.as_u16()));
    }

    // Sengaja ambil `text()` dulu (bukan `.json()`) supaya error jaringan
    // dan error parsing JSON bisa dibedakan dengan jelas.
    let body = response.text().await?;
    parse_gempa(&body, sumber)
}

fn parse_gempa(body: &str, sumber: Sumber) -> Result<Vec<Gempa>, GempaError> {
    if sumber.objek_tunggal() {
        let response: ResponseTunggal = serde_json::from_str(body)?;
        Ok(vec![response.infogempa.gempa])
    } else {
        let response: ResponseList = serde_json::from_str(body)?;
        Ok(dedup(response.infogempa.gempa))
    }
}

// BMKG kadang mengirim gempa yang sama dua kali. Buang duplikatnya
// dengan HashSet, tapi pertahankan urutan aslinya (terbaru di atas).
fn dedup(daftar: Vec<Gempa>) -> Vec<Gempa> {
    let mut terlihat = HashSet::new();
    daftar
        .into_iter()
        .filter(|g| terlihat.insert(g.kunci()))
        .collect()
}

// ============================================
// 6. TAMPILAN
// ============================================
fn garis() {
    println!("────────────────────────────────────────────────────────────");
}

fn cetak_detail(g: &Gempa) {
    garis();
    println!("  {}  M {:.1} — {}", g.skala().ikon(), g.magnitudo(), g.skala().label());
    garis();
    println!("  Waktu      : {} {}", g.tanggal, g.jam);
    println!("  Wilayah    : {}", g.wilayah);
    println!("  Koordinat  : {} , {}", g.lintang, g.bujur);
    println!("  Kedalaman  : {} ({})", g.kedalaman, g.kategori_kedalaman());

    if let Some(potensi) = &g.potensi {
        let penanda = if g.tsunami() { "⚠️  " } else { "" };
        println!("  Potensi    : {}{}", penanda, potensi);
    }

    match g.dirasakan_bersih() {
        Some(skala_mmi) => println!("  Dirasakan  : {}", skala_mmi),
        None => println!("  Dirasakan  : tidak ada laporan"),
    }

    if let Some(url) = g.google_maps_url() {
        println!("  Peta       : {}", url);
    }
    if let Some(url) = g.shakemap_url() {
        println!("  Shakemap   : {}", url);
    }
    garis();
}

fn cetak_tabel(daftar: &[Gempa]) {
    // Kolom "DIRASAKAN" hanya relevan kalau datanya memang ada
    // (endpoint gempadirasakan.json). Cek dulu sebelum menambah kolom.
    let ada_mmi = daftar.iter().any(|g| g.dirasakan_bersih().is_some());
    let lebar_wilayah = if ada_mmi { 26 } else { 34 };

    garis();
    print!("  {:<14} {:<6} {:<9} {:<width$}", "WAKTU", "MAG", "KEDALAMAN", "WILAYAH", width = lebar_wilayah);
    if ada_mmi {
        print!(" DIRASAKAN (MMI)");
    }
    println!();
    garis();

    for g in daftar {
        // Potong wilayah yang kepanjangan biar tabel tetap rapi
        let wilayah = potong(g.wilayah_ringkas(), lebar_wilayah);
        print!(
            "  {:<14} {} {:<4.1} {:<9} {:<width$}",
            g.tanggal,
            g.skala().ikon(),
            g.magnitudo(),
            g.kedalaman,
            wilayah,
            width = lebar_wilayah
        );
        if ada_mmi {
            print!(" {}", g.dirasakan_bersih().unwrap_or("-"));
        }
        println!();
    }
    garis();
}

fn potong(teks: &str, maks: usize) -> String {
    // Hati-hati: jangan pakai slice byte langsung, nanti panic di karakter non-ASCII
    let karakter: Vec<char> = teks.chars().collect();
    if karakter.len() <= maks {
        teks.to_string()
    } else {
        let potongan: String = karakter[..maks.saturating_sub(1)].iter().collect();
        format!("{}…", potongan)
    }
}

fn cetak_statistik(daftar: &[Gempa]) {
    if daftar.is_empty() {
        return;
    }

    let total = daftar.len() as f64;
    let magnitudo: Vec<f64> = daftar.iter().map(|g| g.magnitudo()).collect();
    let rata_rata = magnitudo.iter().sum::<f64>() / total;

    // f64 tidak punya Ord (karena ada NaN), jadi pakai partial_cmp
    let terkuat = daftar
        .iter()
        .max_by(|a, b| a.magnitudo().partial_cmp(&b.magnitudo()).unwrap())
        .unwrap();

    // Hitung sebaran per skala pakai HashMap + entry API (modul 04)
    let mut per_skala: HashMap<Skala, usize> = HashMap::new();
    let mut per_kedalaman: HashMap<&str, usize> = HashMap::new();
    for g in daftar {
        *per_skala.entry(g.skala()).or_insert(0) += 1;
        *per_kedalaman.entry(g.kategori_kedalaman()).or_insert(0) += 1;
    }

    println!("\n  📊 STATISTIK ({} gempa)", daftar.len());
    garis();
    println!("  Rata-rata magnitudo : M {:.2}", rata_rata);
    println!("  Terkuat             : M {:.1} — {}", terkuat.magnitudo(), terkuat.wilayah);

    let urutan = [Skala::Besar, Skala::Kuat, Skala::Sedang, Skala::Ringan, Skala::Kecil];
    println!("\n  Sebaran magnitudo:");
    for skala in urutan {
        if let Some(jumlah) = per_skala.get(&skala) {
            println!("    {:<12} {} ({})", skala.label(), "▇".repeat(*jumlah), jumlah);
        }
    }

    println!("\n  Sebaran kedalaman:");
    for kategori in ["Dangkal", "Menengah", "Dalam"] {
        if let Some(jumlah) = per_kedalaman.get(kategori) {
            println!("    {:<12} {} ({})", kategori, "▇".repeat(*jumlah), jumlah);
        }
    }
    garis();
}

fn bantuan() {
    println!(
        r#"
Monitor Gempa BMKG 🇮🇩

PEMAKAIAN:
    gempa [SUMBER] [OPSI]

SUMBER:
    terbaru      Gempa paling baru (default)
    terkini      15 gempa terkini dengan M >= 5.0
    dirasakan    15 gempa terakhir yang dirasakan warga

OPSI:
    --min-mag <ANGKA>   Hanya tampilkan gempa dengan magnitudo >= ANGKA
    --json              Cetak hasil sebagai JSON
    --watch <DETIK>     Cek ulang tiap N detik (Ctrl+C untuk berhenti)
    -h, --help          Tampilkan bantuan ini

CONTOH:
    gempa
    gempa terkini --min-mag 5.5
    gempa dirasakan --json
    gempa terbaru --watch 60

Sumber data: https://data.bmkg.go.id/DataMKG/TEWS/
"#
    );
}

// ============================================
// 7. PARSING ARGUMEN CLI
// ============================================
struct Opsi {
    sumber: Sumber,
    min_mag: f64,
    as_json: bool,
    watch: Option<u64>,
    tampilkan_bantuan: bool,
}

fn parse_argumen(args: &[String]) -> Result<Opsi, GempaError> {
    let mut opsi = Opsi {
        sumber: Sumber::Terbaru,
        min_mag: 0.0,
        as_json: false,
        watch: None,
        tampilkan_bantuan: false,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "terbaru" => opsi.sumber = Sumber::Terbaru,
            "terkini" => opsi.sumber = Sumber::Terkini,
            "dirasakan" => opsi.sumber = Sumber::Dirasakan,
            "--json" => opsi.as_json = true,
            "-h" | "--help" => opsi.tampilkan_bantuan = true,
            "--min-mag" => {
                i += 1;
                let nilai = args
                    .get(i)
                    .ok_or_else(|| GempaError::Argumen("--min-mag butuh angka".into()))?;
                opsi.min_mag = nilai.parse().map_err(|_| {
                    GempaError::Argumen(format!("'{}' bukan angka yang valid", nilai))
                })?;
            }
            "--watch" => {
                i += 1;
                let nilai = args
                    .get(i)
                    .ok_or_else(|| GempaError::Argumen("--watch butuh jumlah detik".into()))?;
                let detik: u64 = nilai.parse().map_err(|_| {
                    GempaError::Argumen(format!("'{}' bukan jumlah detik yang valid", nilai))
                })?;
                if detik < 10 {
                    return Err(GempaError::Argumen(
                        "interval --watch minimal 10 detik, jangan membanjiri server BMKG".into(),
                    ));
                }
                opsi.watch = Some(detik);
            }
            lain => {
                return Err(GempaError::Argumen(format!(
                    "'{}' tidak dikenal. Jalankan dengan --help untuk melihat pilihan.",
                    lain
                )))
            }
        }
        i += 1;
    }

    Ok(opsi)
}

// ============================================
// 8. MAIN
// ============================================
#[tokio::main]
async fn main() {
    // Pola umum: main() tipis, semua logika di fungsi yang mengembalikan Result
    if let Err(e) = jalankan().await {
        eprintln!("\n❌ {}", e);
        std::process::exit(1);
    }
}

async fn jalankan() -> Result<(), GempaError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opsi = parse_argumen(&args)?;

    if opsi.tampilkan_bantuan {
        bantuan();
        return Ok(());
    }

    let klien = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("belajar-rust-gempa/0.1")
        .build()?;

    match opsi.watch {
        None => tampilkan(&klien, &opsi).await,
        Some(detik) => {
            println!("👀 Mode watch aktif — cek tiap {} detik. Ctrl+C untuk berhenti.\n", detik);
            let mut terakhir = String::new();
            loop {
                match ambil_gempa(&klien, opsi.sumber).await {
                    Ok(daftar) => {
                        // Cetak ulang hanya kalau ada gempa baru
                        let sidik = daftar.first().map(|g| g.kunci()).unwrap_or_default();
                        if sidik != terakhir {
                            terakhir = sidik;
                            cetak_hasil(&daftar, &opsi);
                        } else {
                            println!("… belum ada gempa baru");
                        }
                    }
                    // Di mode watch, error jaringan jangan mematikan program
                    Err(e) => eprintln!("⚠️  {} (coba lagi nanti)", e),
                }
                tokio::time::sleep(Duration::from_secs(detik)).await;
            }
        }
    }
}

async fn tampilkan(klien: &reqwest::Client, opsi: &Opsi) -> Result<(), GempaError> {
    let daftar = ambil_gempa(klien, opsi.sumber).await?;
    cetak_hasil(&daftar, opsi);
    Ok(())
}

fn cetak_hasil(daftar: &[Gempa], opsi: &Opsi) {
    // Filter magnitudo minimum
    let hasil: Vec<Gempa> = daftar
        .iter()
        .filter(|g| g.magnitudo() >= opsi.min_mag)
        .cloned()
        .collect();

    if opsi.as_json {
        // unwrap aman: struct kita pasti bisa diserialisasi
        println!("{}", serde_json::to_string_pretty(&hasil).unwrap());
        return;
    }

    println!("\n🌏  {} · BMKG\n", opsi.sumber.judul());

    if hasil.is_empty() {
        println!("  Tidak ada gempa dengan magnitudo ≥ {:.1}", opsi.min_mag);
        return;
    }

    if hasil.len() == 1 {
        cetak_detail(&hasil[0]);
    } else {
        cetak_tabel(&hasil);
        cetak_statistik(&hasil);
    }

    println!("\n  Sumber: {}\n", opsi.sumber.url());
}

// ============================================
// 9. UNIT TEST
// ============================================
// Test tidak menyentuh jaringan — pakai contoh JSON yang di-hardcode,
// jadi `cargo test` tetap jalan walau offline.
#[cfg(test)]
mod tests {
    use super::*;

    const CONTOH_TUNGGAL: &str = r#"{
        "Infogempa": { "gempa": {
            "Tanggal": "14 Agu 2026",
            "Jam": "08:14:48 WIB",
            "DateTime": "2026-08-14T01:14:48+00:00",
            "Coordinates": "5.36,125.34",
            "Lintang": "5.36 LU",
            "Bujur": "125.34 BT",
            "Magnitude": "5.3",
            "Kedalaman": "10 km",
            "Wilayah": "195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT",
            "Potensi": "Tidak berpotensi tsunami",
            "Dirasakan": "-",
            "Shakemap": "20260814081725.mmi.jpg"
        }}
    }"#;

    const CONTOH_LIST: &str = r#"{
        "Infogempa": { "gempa": [
            {
                "Tanggal": "14 Agu 2026", "Jam": "08:14:48 WIB",
                "DateTime": "2026-08-14T01:14:48+00:00",
                "Coordinates": "5.36,125.34", "Lintang": "5.36 LU", "Bujur": "125.34 BT",
                "Magnitude": "5.3", "Kedalaman": "10 km",
                "Wilayah": "195 km BaratLaut TAHUNA", "Potensi": "Tidak berpotensi tsunami"
            },
            {
                "Tanggal": "14 Agu 2026", "Jam": "08:14:48 WIB",
                "DateTime": "2026-08-14T01:14:48+00:00",
                "Coordinates": "5.36,125.34", "Lintang": "5.36 LU", "Bujur": "125.34 BT",
                "Magnitude": "5.3", "Kedalaman": "10 km",
                "Wilayah": "195 km BaratLaut TAHUNA", "Potensi": "Tidak berpotensi tsunami"
            },
            {
                "Tanggal": "13 Agu 2026", "Jam": "01:39:29 WIB",
                "DateTime": "2026-08-13T18:39:29+00:00",
                "Coordinates": "-7.24,107.58", "Lintang": "7.24 LS", "Bujur": "107.58 BT",
                "Magnitude": "6.7", "Kedalaman": "320 km",
                "Wilayah": "25 km selatan Bandung", "Potensi": "Tidak berpotensi tsunami"
            }
        ]}
    }"#;

    fn contoh() -> Gempa {
        parse_gempa(CONTOH_TUNGGAL, Sumber::Terbaru).unwrap().remove(0)
    }

    #[test]
    fn parse_objek_tunggal() {
        let daftar = parse_gempa(CONTOH_TUNGGAL, Sumber::Terbaru).unwrap();
        assert_eq!(daftar.len(), 1);
        assert_eq!(daftar[0].wilayah, "195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT");
    }

    #[test]
    fn parse_array_sekaligus_buang_duplikat() {
        let daftar = parse_gempa(CONTOH_LIST, Sumber::Terkini).unwrap();
        assert_eq!(daftar.len(), 2, "entri kembar harus dibuang");
    }

    #[test]
    fn konversi_angka_dari_string() {
        let g = contoh();
        assert_eq!(g.magnitudo(), 5.3);
        assert_eq!(g.kedalaman_km(), 10.0);
        assert_eq!(g.koordinat(), Some((5.36, 125.34)));
    }

    #[test]
    fn klasifikasi_skala() {
        assert_eq!(Skala::dari_magnitudo(3.2), Skala::Kecil);
        assert_eq!(Skala::dari_magnitudo(4.0), Skala::Ringan);
        assert_eq!(Skala::dari_magnitudo(5.3), Skala::Sedang);
        assert_eq!(Skala::dari_magnitudo(6.9), Skala::Kuat);
        assert_eq!(Skala::dari_magnitudo(7.0), Skala::Besar);
    }

    #[test]
    fn kategori_kedalaman_sesuai_batas() {
        let daftar = parse_gempa(CONTOH_LIST, Sumber::Terkini).unwrap();
        assert_eq!(daftar[0].kategori_kedalaman(), "Dangkal"); // 10 km
        assert_eq!(daftar[1].kategori_kedalaman(), "Dalam"); // 320 km
    }

    #[test]
    fn dirasakan_strip_dianggap_kosong() {
        assert_eq!(contoh().dirasakan_bersih(), None);
    }

    #[test]
    fn potensi_tsunami_tidak_salah_baca() {
        // "Tidak berpotensi tsunami" jangan sampai dibaca sebagai ada potensi
        assert!(!contoh().tsunami());
    }

    #[test]
    fn shakemap_jadi_url_penuh() {
        assert_eq!(
            contoh().shakemap_url(),
            Some(format!("{}/20260814081725.mmi.jpg", BASE_URL))
        );
    }

    #[test]
    fn json_rusak_menghasilkan_error_parse() {
        let hasil = parse_gempa("{ bukan json }", Sumber::Terbaru);
        assert!(matches!(hasil, Err(GempaError::Parse(_))));
    }

    #[test]
    fn argumen_watch_terlalu_cepat_ditolak() {
        let args = vec!["--watch".to_string(), "1".to_string()];
        assert!(matches!(parse_argumen(&args), Err(GempaError::Argumen(_))));
    }

    #[test]
    fn argumen_default_ke_terbaru() {
        let opsi = parse_argumen(&[]).unwrap();
        assert_eq!(opsi.sumber, Sumber::Terbaru);
        assert_eq!(opsi.min_mag, 0.0);
    }

    #[test]
    fn potong_aman_untuk_karakter_unicode() {
        assert_eq!(potong("Bandung", 20), "Bandung");
        assert_eq!(potong("gempa di Pangalengan", 10), "gempa di …");
    }
}
