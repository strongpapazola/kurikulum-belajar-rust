# 08 · Mini Projects

> Terapkan semua yang sudah dipelajari ke proyek nyata.

---

## Daftar Project

| Folder | Level | Topik yang Dipakai | Status |
|--------|-------|-------------------|--------|
| [csv_processor](./csv_processor) | ⭐⭐ Menengah | Vec, HashMap, struct, error handling | ✅ Selesai |
| [cli_kalkulator](./cli_kalkulator) | ⭐ Pemula | basic, functions, match, env::args | ✅ Selesai |
| [rest_api](./rest_api) | ⭐⭐⭐ Lanjut | Axum, async, serde, JSON | ✅ Selesai |
| [gempa_bmkg](./gempa_bmkg) | ⭐⭐⭐ Lanjut | reqwest, async, serde, API publik | ✅ Selesai |

---

## 01 · CSV Processor e-Faktur

> Baca data CSV, validasi NPWP, hitung PPN, generate output CSV baru.

📄 [csv_processor/main.rs](./csv_processor/main.rs)

**Cara jalankan:**
```bash
cd csv_processor
rustc main.rs && ./main
```

**Fitur:**
- Parse CSV manual tanpa library
- Validasi NPWP (15 digit)
- Hitung DPP, PPN 11%, total
- Summary per pembeli menggunakan HashMap
- Report grand total

---

## 02 · CLI Kalkulator Pajak

> Kalkulator pajak yang berjalan di terminal — support PPN, PPh 21, PPh 23.

📄 [cli_kalkulator/main.rs](./cli_kalkulator/main.rs)

**Cara jalankan:**
```bash
cd cli_kalkulator
rustc main.rs -o kalkulator

# Mode demo (tanpa argumen)
./kalkulator

# PPN
./kalkulator ppn 5000000

# PPh 21 (gaji tahunan, status menikah)
./kalkulator pph21 120000000 menikah

# PPh 23
./kalkulator pph23 10000000 jasa
./kalkulator pph23 50000000 dividen
```

**Contoh output:**
```
────────────────────────────
  Jenis Pajak : PPh Pasal 21
  DPP         : Rp 120.000.000
  Pajak       : Rp 3.225.000
  Total Bayar : Rp 120.000.000
  Keterangan  : PTKP: Rp 58.500.000 | PKP: Rp 61.500.000 | PPh/bulan: Rp 268.750
────────────────────────────
```

**Konsep yang dipraktekkan:**
- `env::args()` untuk baca argumen CLI
- Custom error handling
- Enum untuk jenis pajak
- Tarif progresif PPh 21

---

## 03 · REST API Pajak

> REST API dengan Axum — framework web async Rust yang sangat cepat.

📄 [rest_api/src/main.rs](./rest_api/src/main.rs)

**Setup & jalankan:**
```bash
cd rest_api
cargo run
```

**Endpoint:**
```
GET  /health
POST /hitung/ppn      { "dpp": 5000000 }
POST /hitung/pph21    { "penghasilan_setahun": 120000000, "menikah": true }
POST /hitung/pph23    { "dpp": 10000000, "objek": "jasa" }
POST /validasi/npwp   { "npwp": "12.345.678.9-012.345" }
```

**Test dengan curl:**
```bash
# Health check
curl http://localhost:3000/health

# Hitung PPN
curl -X POST http://localhost:3000/hitung/ppn \
  -H "Content-Type: application/json" \
  -d '{"dpp": 5000000}'

# Response:
# {
#   "dpp": 5000000.0,
#   "tarif_persen": 11.0,
#   "ppn": 550000.0,
#   "total": 5550000.0
# }

# Hitung PPh 21
curl -X POST http://localhost:3000/hitung/pph21 \
  -H "Content-Type: application/json" \
  -d '{"penghasilan_setahun": 120000000, "menikah": true}'

# Validasi NPWP
curl -X POST http://localhost:3000/validasi/npwp \
  -H "Content-Type: application/json" \
  -d '{"npwp": "12.345.678.9-012.345"}'
```

**Jalankan unit test:**
```bash
cargo test
```

**Konsep yang dipraktekkan:**
- `axum` untuk routing dan handler
- `serde` untuk JSON serialization/deserialization
- `tokio` async runtime
- Request/Response types terpisah
- Unit testing dengan `#[cfg(test)]`

---

## 04 · Monitor Gempa BMKG

> Ambil data gempa real-time dari API publik BMKG, parse JSON-nya, tampilkan di terminal.

📄 [gempa_bmkg/README.md](./gempa_bmkg/README.md) · [src/main.rs](./gempa_bmkg/src/main.rs) · [sederhana.rs](./gempa_bmkg/sederhana.rs)

Project ini punya **dua versi** untuk hal yang sama — versi tanpa dependency sama sekali,
dan versi Cargo yang memakai `reqwest` + `serde`.

**Versi tanpa dependency** (biarkan `curl` yang ambil data, program baca stdin):
```bash
cd gempa_bmkg
rustc sederhana.rs -o gempa
curl -s https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json | ./gempa
```

**Versi Cargo:**
```bash
cd gempa_bmkg
cargo run                            # gempa terbaru
cargo run -- terkini --min-mag 5.5   # 15 gempa terkini, filter magnitudo
cargo run -- dirasakan               # gempa yang dirasakan warga
cargo run -- terbaru --json          # output JSON
cargo run -- terbaru --watch 60      # polling tiap 60 detik
```

**Contoh output:**
```
🌏  GEMPA TERBARU · BMKG

────────────────────────────────────────────────────────────
  🟠  M 5.3 — Sedang
────────────────────────────────────────────────────────────
  Waktu      : 14 Agu 2026 08:14:48 WIB
  Wilayah    : 195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT
  Koordinat  : 5.36 LU , 125.34 BT
  Kedalaman  : 10 km (Dangkal)
  Potensi    : Tidak berpotensi tsunami
  Dirasakan  : tidak ada laporan
  Peta       : https://www.google.com/maps?q=5.36,125.34
  Shakemap   : https://data.bmkg.go.id/DataMKG/TEWS/20260814081725.mmi.jpg
────────────────────────────────────────────────────────────
```

**Konsep yang dipraktekkan:**
- `reqwest` untuk HTTP request async
- Custom error enum + `impl From` supaya operator `?` bekerja
- `#[serde(rename)]` untuk key JSON PascalCase
- `Option<T>` untuk field yang tidak selalu ada
- `HashSet` untuk buang data kembar, `HashMap` untuk statistik
- Data BMKG semuanya bertipe String — konversi angka dikerjakan sendiri

---

## Tips Extend Project Ini

| Project | Ide Pengembangan |
|---------|-----------------|
| CSV Processor | Baca dari file nyata, export ke JSON, validasi lebih lengkap |
| CLI Kalkulator | Mode interaktif (loop input), export ke PDF, history kalkulasi |
| REST API | Tambah database (SQLite/Postgres), auth JWT, rate limiting, Docker |
| Monitor Gempa | Simpan ke SQLite, notifikasi Telegram kalau M ≥ 6.0, hitung jarak ke kotamu |

---

*Selamat — kamu sudah menyelesaikan kurikulum! 🎉*
*Next step: buat project sendiri, atau kontribusi ke open source Rust!* 🦀
