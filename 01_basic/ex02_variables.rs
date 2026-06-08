// ============================================
// EXERCISE 02 - Variables & Data Types
// ============================================
// Jalankan: rustc ex02_variables.rs && ./ex02_variables

fn main() {
    // TODO 1: Buat variable IMMUTABLE bernama "kota" dengan nilai "Jakarta"
    //         lalu print nilainya


    // TODO 2: Buat variable MUTABLE bernama "saldo" dengan nilai 1_000_000
    //         tambah 500_000, lalu print hasilnya
    //         Expected: "Saldo: 1500000"


    // TODO 3: Gunakan SHADOWING untuk konversi:
    //         - buat variable "input" berisi "  42  " (string dengan spasi)
    //         - shadow "input" menjadi hasil trim (hilangkan spasi)
    //         - shadow lagi "input" menjadi angka integer (parse)
    //         - print: "Angka: 42"
    let input = "  42  ";


    // TODO 4: Buat KONSTANTA untuk:
    //         - TARIF_PPN = 0.11 (f64)
    //         - BATAS_PKP = 4_800_000_000 (u64) → batas omzet PKP
    //         lalu hitung dan print PPN dari transaksi Rp 10.000.000
    //         Expected: "PPN: 1100000"


    // TODO 5: Destructuring tuple
    //         Buat tuple (nama, npwp, omzet) untuk sebuah perusahaan
    //         Destructure ke 3 variable terpisah, lalu print masing-masing


    // TODO 6: Perbaiki kode ini supaya bisa compile:
    //         (hapus komentar dan fix errornya)
    // let angka = 10;
    // angka = 20;
    // println!("{angka}");

}

// ============================================
// Expected output:
// Jakarta
// Saldo: 1500000
// Angka: 42
// PPN: 1100000
// (output tuple sesuai data yang kamu buat)
// 20
// ============================================
