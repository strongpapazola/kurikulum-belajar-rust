# 08 · Mini Projects

> Terapkan semua yang sudah dipelajari ke proyek nyata.
> Semua project bertema **pajak & keuangan** — relevan untuk dunia kerja!

---

## Daftar Project

| Folder | Level | Topik yang Dipakai | Status |
|--------|-------|-------------------|--------|
| [csv_processor](./csv_processor) | ⭐⭐ Menengah | Vec, HashMap, struct, error handling | ✅ Ada |
| [cli_kalkulator](./cli_kalkulator) | ⭐ Pemula | basic, functions, match, input | 🚧 Soon |
| [rest_api](./rest_api) | ⭐⭐⭐ Lanjut | async, Axum, serde, error handling | 🚧 Soon |

---

## 01 · CSV Processor e-Faktur

> Simulasi baca data CSV, validasi, hitung PPN, dan generate output CSV baru.
> Ini adalah use case nyata di sistem perpajakan!

📄 [Lihat kode → csv_processor/main.rs](./csv_processor/main.rs)

**Yang dipraktekkan:**
- Struct untuk model data (`FakturItem`)
- Validasi NPWP dengan error handling
- Parsing CSV manual
- Vec, HashMap untuk aggregate data
- Generate output CSV

**Cara jalankan:**
```bash
cd csv_processor
rustc main.rs && ./main
```

**Contoh output:**
```
=== CSV Processor e-Faktur ===

=== ERROR VALIDASI ===
❌ Baris 3: NPWP 'INVALID_NPWP' tidak valid
❌ Baris 6: NPWP '55.666.777.8-999.INVALID' tidak valid

=== FAKTUR VALID (4 item) ===
✅ FKT-001 | PT Maju Jaya   | Laptop    | DPP: 30000000 | PPN: 3300000
✅ FKT-002 | CV Sejahtera   | Printer   | DPP: 3500000  | PPN: 385000
...

=== SUMMARY PER PEMBELI ===
PT Maju Jaya: 2 faktur, Total Rp 38885000
CV Sejahtera: 1 faktur, Total Rp 3885000
```

---

## 02 · CLI Kalkulator Pajak (Coming Soon 🚧)

> Kalkulator pajak interaktif di terminal.

**Yang akan dipraktekkan:**
- Input dari user (`std::io`)
- Match untuk pilih jenis pajak
- Struct & method untuk kalkulasi
- Loop untuk menu interaktif

---

## 03 · REST API Pajak (Coming Soon 🚧)

> REST API sederhana dengan endpoint untuk kalkulasi pajak.

**Stack:**
- **Axum** — web framework Rust
- **Serde** — JSON serialization
- **Tokio** — async runtime

**Endpoint yang akan dibuat:**
```
POST /hitung-ppn     → { dpp: 1000000 } → { ppn: 110000, total: 1110000 }
POST /hitung-pph21   → { gaji, npwp }   → { pph21_bulanan, pph21_tahunan }
POST /validasi-npwp  → { npwp }         → { valid: true/false }
```

---

## Tips Mengerjakan Mini Project

1. **Baca kode dari atas ke bawah** — struct dulu, baru impl, baru main
2. **Jalankan dulu** sebelum modifikasi — pastikan kode asli works
3. **Tambah fitur sendiri** setelah paham — latihan terbaik!
4. **Ideas untuk extend:**
   - Baca CSV dari file nyata
   - Export ke format lain (JSON, Excel)
   - Tambah lebih banyak validasi DJP
   - Connect ke API sungguhan

---

*Selamat — kamu sudah menyelesaikan kurikulum! 🎉 Terus berlatih dan buat proyek sendiri!* 🦀
