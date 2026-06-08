// ============================================
// EXERCISE 04 - Control Flow
// ============================================
// Jalankan: rustc ex04_control_flow.rs && ./ex04_control_flow

fn main() {
    // TODO 1: Buat match untuk status HTTP berikut
    //         200 → "OK", 201 → "Created", 400 → "Bad Request",
    //         401 → "Unauthorized", 404 → "Not Found", 500 → "Server Error"
    //         selain itu → "Unknown"
    let codes = vec![200, 201, 400, 401, 404, 500, 999];
    for code in codes {
        // print: "200: OK", "201: Created", dst
    }

    // TODO 2: Gunakan IF sebagai ekspresi
    //         Buat variable "kategori" berdasarkan omzet:
    //         < 500jt         → "Sangat Kecil"
    //         500jt - 4.8M    → "Kecil"
    //         4.8M - 50M      → "Menengah"
    //         > 50M           → "Besar"
    //         print: "Omzet 1M: Kategori Menengah"
    let omzet = 1_000_000_000_u64;


    // TODO 3: Gunakan LOOP untuk hitung bunga berbunga
    //         Modal awal: Rp 10.000.000, bunga 5%/tahun
    //         Loop sampai modal >= Rp 20.000.000 (dobel)
    //         Print berapa tahun yang dibutuhkan
    //         Expected: "Modal dobel setelah X tahun"


    // TODO 4: FizzBuzz versi pajak — for loop 1 sampai 30:
    //         - habis dibagi 3 → print "PPh"
    //         - habis dibagi 5 → print "PPN"
    //         - habis dibagi 3 dan 5 → print "PPh+PPN"
    //         - lainnya → print angkanya


    // TODO 5: Gunakan while + counter untuk print deret ini:
    //         1, 2, 4, 8, 16, 32, 64, 128
    //         (tiap elemen = elemen sebelumnya * 2)


    // TODO 6: Match dengan GUARD (kondisi tambahan)
    //         Untuk setiap nilai berikut, tentukan grade pajak:
    //         0        → "Nihil"
    //         1..100rb → "Kecil" (plus print nilainya)
    //         100rb..1M → "Menengah"
    //         _ jika genap → "Besar (genap)"
    //         _ → "Besar (ganjil)"
    let pajak_values = vec![0i64, 50_000, 500_000, 2_000_000, 3_000_001];
    for pajak in pajak_values {
        // tulis match di sini
    }
}

// ============================================
// Expected output:
// 200: OK
// 201: Created
// 400: Bad Request
// 401: Unauthorized
// 404: Not Found
// 500: Server Error
// 999: Unknown
// Omzet 1M: Kategori Menengah
// Modal dobel setelah X tahun
// 1 2 PPh 4 PPN PPh 7 8 PPh PPN 11 PPh 13 14 PPh+PPN ...
// 1 2 4 8 16 32 64 128
// 0: Nihil
// 50000: Kecil
// 500000: Menengah
// 2000000: Besar (genap)
// 3000001: Besar (ganjil)
// ============================================
