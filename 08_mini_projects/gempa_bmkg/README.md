# 🌏 Monitor Gempa BMKG

> Studi kasus: ambil data gempa **real-time** dari API publik BMKG, parse JSON-nya, dan tampilkan di terminal.

> **Versi interaktifnya ada di web:** [Modul 08 · Studi Kasus: API BMKG](https://strongpapazola.github.io/kurikulum-belajar-rust/)
> — bisa langsung edit & jalankan kodenya di browser tanpa install apa pun.

API yang dipakai (gratis, tanpa API key):

| Endpoint | Isi | Bentuk JSON |
|----------|-----|-------------|
| [`autogempa.json`](https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json) | 1 gempa terbaru | objek |
| [`gempaterkini.json`](https://data.bmkg.go.id/DataMKG/TEWS/gempaterkini.json) | 15 gempa terkini M ≥ 5.0 | array |
| [`gempadirasakan.json`](https://data.bmkg.go.id/DataMKG/TEWS/gempadirasakan.json) | 15 gempa yang dirasakan warga | array |

---

## Dua Versi

Project ini sengaja punya **dua implementasi** untuk hal yang sama:

| File | Dependency | Cara jalan | Cocok untuk |
|------|-----------|-----------|-------------|
| [`sederhana.rs`](./sederhana.rs) | ❌ tidak ada | `rustc` + pipe dari `curl` | memahami dasarnya |
| [`src/main.rs`](./src/main.rs) | ✅ reqwest, tokio, serde | `cargo run` | cara yang sebenarnya dipakai |

Mulai dari `sederhana.rs` dulu, baru pindah ke versi Cargo.

---

## Versi 1 — Tanpa Dependency

Program hanya membaca **stdin**; biar `curl` yang mengambil datanya.

```bash
cd 08_mini_projects/gempa_bmkg
rustc sederhana.rs -o gempa
curl -s https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json | ./gempa
```

Output:

```
🌏  GEMPA TERBARU · BMKG

────────────────────────────────────────────────
  🟠 Sedang  M 5.3
────────────────────────────────────────────────
  Waktu     : 14 Agu 2026 08:14:48 WIB
  Wilayah   : 195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT
  Koordinat : 5.36 LU , 125.34 BT
  Kedalaman : 10 km (Dangkal)
  Potensi   : Tidak berpotensi tsunami
────────────────────────────────────────────────
```

Jalankan test-nya:

```bash
rustc --test sederhana.rs -o test_gempa && ./test_gempa
```

**Yang dipelajari di sini:** baca stdin, cari substring, `Option<T>` + operator `?`, `match` dengan guard.
Parser JSON-nya ditulis manual dan sengaja minimalis — untuk melihat apa yang sebenarnya dikerjakan `serde_json`.

---

## Versi 2 — Cargo (async + serde)

```bash
cd 08_mini_projects/gempa_bmkg
cargo run
```

### Pemakaian

```bash
cargo run                            # gempa terbaru (default)
cargo run -- terkini                 # 15 gempa terkini M >= 5.0
cargo run -- dirasakan               # 15 gempa yang dirasakan warga
cargo run -- terkini --min-mag 5.5   # filter magnitudo
cargo run -- terbaru --json          # output JSON, enak buat di-pipe ke jq
cargo run -- terbaru --watch 60      # cek ulang tiap 60 detik
cargo run -- --help
```

### Contoh output daftar

```
🌏  15 GEMPA TERKINI (M ≥ 5.0) · BMKG

────────────────────────────────────────────────────────────
  WAKTU          MAG    KEDALAMAN WILAYAH
────────────────────────────────────────────────────────────
  14 Agu 2026    🟠 5.7  48 km     175 km TimurLaut TAHUNA-KEP.SANGI…
  09 Agu 2026    🟠 5.5  40 km     84 km Tenggara ENGGANO-BENGKULU
  05 Agu 2026    🔴 6.4  10 km     221 km BaratLaut PULAUKARATUNG-SU…
────────────────────────────────────────────────────────────

  📊 STATISTIK (8 gempa)
────────────────────────────────────────────────────────────
  Rata-rata magnitudo : M 5.76
  Terkuat             : M 6.4 — 221 km BaratLaut PULAUKARATUNG-SULUT

  Sebaran magnitudo:
    Kuat         ▇ (1)
    Sedang       ▇▇▇▇▇▇▇ (7)

  Sebaran kedalaman:
    Dangkal      ▇▇▇▇▇▇▇ (7)
    Menengah     ▇ (1)
────────────────────────────────────────────────────────────
```

### Test

```bash
cargo test
```

12 test, semuanya offline (pakai contoh JSON yang di-hardcode) — jadi tetap jalan tanpa internet.

---

## Hal Menarik dari Data BMKG

Beberapa jebakan nyata yang bagus untuk latihan:

**1. Semua nilai bertipe String, termasuk angka**

```json
"Magnitude": "5.3",
"Kedalaman": "10 km",
"Coordinates": "5.36,125.34"
```

Jadi konversinya kita kerjakan sendiri:

```rust
fn magnitudo(&self) -> f64 {
    self.magnitude.trim().parse().unwrap_or(0.0)
}

fn kedalaman_km(&self) -> f64 {
    self.kedalaman
        .split_whitespace()   // "10 km" -> ["10", "km"]
        .next()               // Option<&str>
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0)
}
```

**2. Bentuk JSON-nya beda-beda antar endpoint**

`autogempa.json` → `Infogempa.gempa` berupa **objek**.
`gempaterkini.json` → `Infogempa.gempa` berupa **array**.

Karena itu ada dua struct pembungkus, dan `parse_gempa()` memilih yang mana berdasarkan sumbernya.

**3. Field opsional**

`Potensi` tidak ada di `gempadirasakan.json`, `Shakemap` hanya ada di `autogempa.json`.
Solusinya `Option<String>` — kalau tidak, parsing akan gagal:

```rust
#[serde(rename = "Potensi")]
potensi: Option<String>,
```

**4. Key JSON PascalCase, konvensi Rust snake_case**

Dijembatani `#[serde(rename = "...")]`.

**5. "Tidak berpotensi tsunami" mengandung kata "berpotensi"**

Pengecekan naif `contains("berpotensi")` akan salah membaca. Ada unit test khusus untuk ini.

**6. BMKG kadang mengirim entri kembar**

Dibuang dengan `HashSet`, tapi urutan aslinya dipertahankan:

```rust
fn dedup(daftar: Vec<Gempa>) -> Vec<Gempa> {
    let mut terlihat = HashSet::new();
    daftar
        .into_iter()
        .filter(|g| terlihat.insert(g.kunci()))
        .collect()
}
```

**7. Nama wilayah mengandung karakter non-ASCII**

Memotong string dengan slice byte (`&teks[..34]`) bisa **panic** di tengah karakter multi-byte.
Makanya `potong()` bekerja di level `char`, bukan byte.

---

## Konsep yang Dipraktekkan

| Modul | Dipakai di mana |
|-------|----------------|
| 01 Basic | struct, impl, fungsi, format string |
| 03 Enums | `enum Sumber`, `enum Skala`, `Option<T>`, `match` guard |
| 04 Collections | `Vec`, `HashMap` entry API, `HashSet` untuk dedup, iterator chain |
| 05 Error Handling | `enum GempaError`, `impl Display`, `impl From` supaya `?` jalan |
| 06 Traits | `impl fmt::Display`, `impl std::error::Error` |
| 07 Async | `#[tokio::main]`, `async fn`, `.await`, `tokio::time::sleep` |

---

## Ide Pengembangan

- Simpan riwayat gempa ke SQLite, lalu buat grafik frekuensi per bulan
- Kirim notifikasi (Telegram bot / desktop notification) kalau M ≥ 6.0
- Hitung jarak gempa ke kota kamu pakai rumus Haversine — koordinatnya sudah tersedia
- Gabungkan dengan project `rest_api`: bikin endpoint `GET /gempa/terbaru` yang mem-proxy BMKG plus caching
- Ganti parser manual di `sederhana.rs` dengan parser JSON rekursif buatan sendiri (latihan enum + Box)
- Tampilkan peta ASCII sebaran gempa berdasarkan koordinat

---

## Catatan Etika Pemakaian API

Data BMKG bersifat publik dan gratis, tapi tetap dipakai dengan wajar:

- Interval `--watch` dibatasi minimal **10 detik** oleh program ini
- Data BMKG sendiri hanya diperbarui saat ada gempa baru — polling tiap detik tidak ada gunanya
- `User-Agent` diisi jelas supaya trafiknya bisa dikenali
- Untuk peringatan dini yang sesungguhnya, **selalu rujuk kanal resmi BMKG**

---

*Sumber data: [BMKG — Badan Meteorologi, Klimatologi, dan Geofisika](https://data.bmkg.go.id/gempabumi/)* 🇮🇩
