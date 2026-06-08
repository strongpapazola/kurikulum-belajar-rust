// ============================================
// EXERCISE 05 - Structs
// ============================================
// Jalankan: rustc ex05_structs.rs && ./ex05_structs

// TODO 1: Buat struct "Karyawan" dengan field:
//         - nama: String
//         - npwp: String
//         - gaji_pokok: f64
//         - tunjangan: f64
//         - sudah_menikah: bool
//         Tambahkan #[derive(Debug)]


// TODO 2: Implementasi method untuk Karyawan:
//         - fn new(...) -> Karyawan   (constructor)
//         - fn gaji_bruto(&self) -> f64  (gaji_pokok + tunjangan)
//         - fn ptkp(&self) -> f64
//           (menikah: 58_500_000, tidak menikah: 54_000_000)
//         - fn penghasilan_kena_pajak(&self) -> f64
//           (gaji_bruto - ptkp, minimum 0)
//         - fn pph21_tahunan(&self) -> f64
//           (pkp 0-60jt: 5%, 60-250jt: 15%, 250-500jt: 25%, >500jt: 30%)
//         - fn pph21_bulanan(&self) -> f64
//           (pph21_tahunan / 12)
//         - fn tampilkan_slip(&self)
//           (print slip gaji lengkap)


// TODO 3: Buat struct "Perusahaan" dengan field:
//         - nama: String
//         - npwp: String
//         - karyawans: Vec<Karyawan>
// Dan method:
//         - fn tambah_karyawan(&mut self, k: Karyawan)
//         - fn total_gaji(&self) -> f64
//         - fn total_pph21(&self) -> f64
//         - fn laporan(&self)  (print semua karyawan + total)


fn main() {
    // TODO 4: Buat 3 karyawan:
    //         - "Budi Santoso", NPWP "12.345.678.9-012.345",
    //           gaji 8jt, tunjangan 2jt, menikah
    //         - "Siti Aminah", NPWP "98.765.432.1-098.765",
    //           gaji 12jt, tunjangan 3jt, tidak menikah
    //         - "Agus Prayogo", NPWP "11.222.333.4-567.890",
    //           gaji 25jt, tunjangan 5jt, menikah


    // TODO 5: Buat perusahaan "PT Maju Jaya" dan tambahkan semua karyawan


    // TODO 6: Cetak laporan perusahaan (panggil method laporan())


    // TODO 7: Cari karyawan dengan PPh21 tertinggi dan print namanya
    //         Hint: gunakan .iter().max_by()

}

// ============================================
// Expected output (kira-kira):
// ===== SLIP GAJI =====
// Nama        : Budi Santoso
// NPWP        : 12.345.678.9-012.345
// Gaji Bruto  : Rp 10,000,000
// PTKP        : Rp 58,500,000
// PKP         : Rp 61,500,000
// PPh21/tahun : Rp 3,225,000
// PPh21/bulan : Rp 268,750
// =====================
// ... (dst untuk karyawan lain)
//
// ===== LAPORAN PERUSAHAAN PT Maju Jaya =====
// Total Gaji  : Rp XX
// Total PPh21 : Rp XX
// ============================================
