// ============================================
// Curriculum Data — semua konten pelajaran
// ============================================

const CURRICULUM = [
  {
    id: '01_basic',
    title: '01 · Basic',
    icon: '📝',
    lessons: [
      {
        id: 'hello_world',
        title: 'Hello World & Print',
        content: `
# Hello World & Print

Pertama, kita lihat cara print output di Rust. Hampir semua program dimulai dari sini!

## println! vs print!

\`println!\` = print dengan newline di akhir
\`print!\` = print tanpa newline

<div class="concept-box">
<strong>Perbedaan dari PHP/Python:</strong> Di Rust, print adalah <strong>macro</strong> (ada tanda <code>!</code>), bukan function biasa. Ini karena Rust butuh cek format string saat compile time.
</div>

## Format String

Rust punya sistem format yang powerful dengan \`{}\`:

\`\`\`rust
let nama = "Bintang";
let umur = 25;

println!("Nama: {}, Umur: {}", nama, umur);  // cara lama
println!("Nama: {nama}, Umur: {umur}");       // cara baru (Rust 1.58+)
\`\`\`

## Format Angka

\`\`\`rust
let pi = 3.14159_f64;
println!("{:.2}", pi);     // 3.14 (2 desimal)
println!("{:>10}", "ok");  // "        ok" (rata kanan, lebar 10)
println!("{:<10}", "ok");  // "ok        " (rata kiri)
println!("{:0>5}", 42);    // "00042" (padding dengan 0)
\`\`\`

## Debug Print

Untuk print struct, array, vec — pakai \`{:?}\` atau \`{:#?}\` (pretty):

\`\`\`rust
let data = vec![1, 2, 3];
println!("{:?}",  data);   // [1, 2, 3]
println!("{:#?}", data);   // pretty print
\`\`\`

## eprintln! — Print ke Stderr

\`\`\`rust
eprintln!("Error: koneksi DJP gagal!");  // ke stderr
\`\`\`

<div class="concept-box tip">
<strong>Tips:</strong> Di production, gunakan library logging seperti <code>tracing</code> atau <code>log</code> — bukan println! langsung.
</div>
        `,
        defaultCode: `fn main() {
    // Basic print
    println!("Halo, Dunia! 🦀");

    // Format string
    let nama = "Bintang";
    let saldo = 1_500_000.0_f64;
    println!("Nama: {nama}");
    println!("Saldo: Rp {saldo:.0}");

    // Format angka
    let pi = 3.14159_f64;
    println!("Pi = {pi:.3}");
    println!("Hex: {:x}", 255);
    println!("Padding: {:>10}", "kanan");

    // Debug print
    let data = [10, 20, 30, 40, 50];
    println!("Array: {:?}", data);
}`,
        exercise: {
          title: 'Exercise: Hello World',
          desc: `Selesaikan semua TODO di editor!`,
          tasks: [
            'Print nama kamu dengan format: "Halo, [nama]! Selamat datang."',
            'Print nilai pi (3.14159) dengan 2 angka desimal. Format: "Pi ≈ 3.14"',
            'Print teks "pajak.io" rata kanan dengan lebar 20 karakter',
            'Buat variable npwp = "12.345.678.9-012.345", print: "NPWP: [npwp] (15 digit)"',
            'Print array [100, 200, 300] menggunakan debug format',
          ],
          starterCode: `fn main() {
    // TODO 1: Print nama kamu
    // Expected: "Halo, [nama]! Selamat datang."


    // TODO 2: Print pi dengan 2 desimal
    let pi = 3.14159_f64;
    // Expected: "Pi ≈ 3.14"


    // TODO 3: Print "pajak.io" rata kanan, lebar 20
    // Expected: "            pajak.io"


    // TODO 4: Print NPWP
    let npwp = "12.345.678.9-012.345";
    // Expected: "NPWP: 12.345.678.9-012.345 (15 digit)"


    // TODO 5: Debug print array
    let nominal = [100, 200, 300];
    // Expected: [100, 200, 300]

}`,
          hints: [
            'Gunakan println!("Halo, {}! ...", nama) atau println!("Halo, {nama}! ...")',
            'Format 2 desimal: {pi:.2}',
            'Rata kanan dengan lebar 20: {:>20}',
            'String format: {} untuk normal, {:?} untuk debug',
          ]
        }
      },
      {
        id: 'variables',
        title: 'Variables & Tipe Data',
        content: `
# Variables & Tipe Data

## Immutable by Default

Di Rust, variable **tidak bisa diubah** kecuali eksplisit pakai \`mut\`. Ini beda dari PHP/Python/JS!

\`\`\`rust
let x = 5;       // immutable — tidak bisa diubah
let mut y = 5;   // mutable — bisa diubah
y = 10;          // ✅ OK
// x = 10;       // ❌ ERROR
\`\`\`

<div class="concept-box warn">
<strong>Kenapa immutable by default?</strong> Mencegah bug yang tidak disengaja. Kalau data tidak perlu berubah, Rust paksa kamu untuk eksplisit — lebih aman!
</div>

## Shadowing

Bisa re-deklarasi variable dengan nama sama — bahkan beda tipe!

\`\`\`rust
let angka = "42";        // &str
let angka = angka.parse::<i32>().unwrap();  // i32
println!("{angka}");     // 42 (integer)
\`\`\`

## Tipe Integer

\`\`\`rust
let a: i8   = -128;        // -128 s/d 127
let b: i32  = 2_000_000;   // paling umum (default)
let c: i64  = 9_000_000_000;
let d: u8   = 255;         // unsigned 0–255
let e: u32  = 4_000_000;
let f: usize = 0;          // untuk index/pointer
\`\`\`

## Tipe Float, Bool, Char

\`\`\`rust
let tarif: f64 = 0.11;    // default float
let aktif: bool = true;
let simbol: char = '🦀';  // unicode!
\`\`\`

## Tuple & Array

\`\`\`rust
// Tuple — tipe berbeda, fixed size
let (nama, npwp, omzet) = ("PT Maju", "123...", 5_000_000.0);

// Array — tipe sama, fixed size
let tarif: [f64; 3] = [0.05, 0.11, 0.15];
println!("{}", tarif[0]);  // 0.05
\`\`\`

<table class="compare-table">
<tr><th>Bahasa</th><th>Mutable</th><th>Immutable</th></tr>
<tr><td>PHP</td><td><code>$x = 5</code></td><td>(tidak ada, selalu mutable)</td></tr>
<tr><td>JS</td><td><code>let x = 5</code></td><td><code>const x = 5</code></td></tr>
<tr><td>Python</td><td><code>x = 5</code></td><td>(tidak ada, selalu mutable)</td></tr>
<tr><td>Rust</td><td><code>let mut x = 5</code></td><td><code>let x = 5</code> (default!)</td></tr>
</table>
        `,
        defaultCode: `fn main() {
    // Immutable vs mutable
    let x = 10;
    let mut saldo = 1_000_000_i64;
    saldo += 500_000;
    println!("x={x}, saldo={saldo}");

    // Shadowing — bisa ganti tipe!
    let input = "  5000000  ";
    let input = input.trim();
    let input: i64 = input.parse().unwrap();
    println!("Input: {input}");

    // Berbagai tipe
    let tarif_ppn: f64 = 0.11;
    let aktif: bool = true;
    let dpp: i64 = 10_000_000;
    let ppn = (dpp as f64 * tarif_ppn) as i64;
    println!("PPN: {ppn}, aktif: {aktif}");

    // Tuple destructuring
    let faktur = ("FKT-001", "PT Maju", 5_000_000.0_f64);
    let (nomor, pembeli, total) = faktur;
    println!("{nomor} | {pembeli} | {total:.0}");
}`,
        exercise: {
          title: 'Exercise: Variables',
          desc: 'Praktekkan variables, mutability, dan tipe data.',
          tasks: [
            'Buat variable MUTABLE "saldo" = 5_000_000, tambah 2_500_000, kurangi 500_000. Print hasilnya.',
            'Gunakan SHADOWING: buat "npwp_raw" = "  12.345.678.9-012.345  ", trim spasi, lalu hitung panjang digitnya saja.',
            'Buat CONST TARIF_PPN: f64 = 0.11 dan TARIF_PPH23: f64 = 0.02. Hitung pajak dari DPP 8_500_000.',
            'Buat tuple (nama, gaji, menikah) untuk seorang karyawan, destructure dan print PTKP-nya (menikah: 58.5jt, tidak: 54jt).',
          ],
          starterCode: `const TARIF_PPN: f64 = 0.11;
const TARIF_PPH23: f64 = 0.02;

fn main() {
    // TODO 1: Mutable variable saldo
    // Expected: "Saldo akhir: 7000000"


    // TODO 2: Shadowing NPWP
    let npwp_raw = "  12.345.678.9-012.345  ";
    // shadow 1: trim
    // shadow 2: hitung digit saja (hint: .chars().filter(|c| c.is_ascii_digit()).count())
    // Expected: "Digit NPWP: 15"


    // TODO 3: Hitung PPN dan PPh23 dari DPP 8_500_000
    let dpp: f64 = 8_500_000.0;
    // Expected: "PPN: 935000, PPh23: 170000"


    // TODO 4: Tuple karyawan
    // (nama, gaji_bulanan, menikah)
    // Expected: "Budi | PTKP: 58500000"

}`,
          hints: [
            'let mut saldo: i64 = 5_000_000; saldo += ...; saldo -= ...;',
            'let npwp = npwp_raw.trim(); lalu let digit_count = npwp.chars().filter(|c| c.is_ascii_digit()).count();',
            'Kalikan dpp * TARIF_PPN dan dpp * TARIF_PPH23, cast ke i64 dengan as i64',
            'let (nama, gaji, menikah) = ("Budi", 8_000_000, true); let ptkp = if menikah { 58_500_000 } else { 54_000_000 };',
          ]
        }
      },
      {
        id: 'functions',
        title: 'Functions & Closures',
        content: `
# Functions & Closures

## Definisi Function

\`\`\`rust
// Tipe parameter dan return WAJIB ditulis
fn tambah(a: i32, b: i32) -> i32 {
    a + b  // TANPA titik koma = return value!
}

// Dengan titik koma = bukan return value
fn sapa(nama: &str) {
    println!("Halo, {nama}!");
    // return unit () secara implisit
}
\`\`\`

<div class="concept-box">
<strong>Baris terakhir tanpa titik koma</strong> = otomatis jadi return value. Ini sangat idiomatis di Rust!
</div>

## Multiple Return — Tuple

\`\`\`rust
fn hitung_ppn(dpp: f64) -> (f64, f64, f64) {
    let ppn   = dpp * 0.11;
    let total = dpp + ppn;
    (dpp, ppn, total)  // return tuple
}

let (dpp, ppn, total) = hitung_ppn(5_000_000.0);
\`\`\`

## Closures — Anonymous Function

Mirip arrow function JS atau lambda Python:

\`\`\`rust
// JS:     const kali = (x) => x * 2
// Python: kali = lambda x: x * 2
// Rust:
let kali = |x: i32| x * 2;
println!("{}", kali(5));  // 10

// Closure bisa capture variable dari luar
let tarif = 0.11;
let hitung = |dpp: f64| dpp * tarif;
\`\`\`

## Higher-Order Functions

\`\`\`rust
let nominal = vec![1_000_000.0, 2_500_000.0, 800_000.0, 3_200_000.0];

// filter + map + collect
let kena_ppn: Vec<f64> = nominal.iter()
    .filter(|&&n| n >= 1_000_000.0)
    .map(|&n| n * 0.11)
    .collect();

let total_ppn: f64 = kena_ppn.iter().sum();
println!("Total PPN: {total_ppn:.0}");
\`\`\`
        `,
        defaultCode: `fn tambah(a: f64, b: f64) -> f64 {
    a + b  // tanpa titik koma = return value
}

fn hitung_ppn(dpp: f64) -> (f64, f64, f64) {
    let ppn   = dpp * 0.11;
    let total = dpp + ppn;
    (dpp, ppn, total)
}

fn pph21(penghasilan: f64, menikah: bool) -> f64 {
    let ptkp = if menikah { 58_500_000.0 } else { 54_000_000.0 };
    let pkp  = (penghasilan - ptkp).max(0.0);
    pkp * 0.05
}

fn main() {
    println!("Tambah: {}", tambah(3.0, 7.0));

    let (dpp, ppn, total) = hitung_ppn(5_000_000.0);
    println!("DPP: {dpp:.0} | PPN: {ppn:.0} | Total: {total:.0}");

    println!("PPh21: {:.0}", pph21(120_000_000.0, true));

    // Closures
    let tarif_ppn = 0.11_f64;
    let hitung = |dpp: f64| dpp * tarif_ppn;
    println!("PPN closure: {:.0}", hitung(8_000_000.0));

    // Iterator
    let faktur = vec![500_000.0, 2_000_000.0, 1_500_000.0, 3_000_000.0];
    let total: f64 = faktur.iter().filter(|&&n| n > 1_000_000.0).sum();
    println!("Total > 1jt: {total:.0}");
}`,
        exercise: {
          title: 'Exercise: Functions',
          desc: 'Implementasikan function-function di bawah ini!',
          tasks: [
            'Implementasikan fn celsius_ke_fahrenheit(c: f64) -> f64 (rumus: F = C*9/5 + 32)',
            'Implementasikan fn validasi_npwp(npwp: &str) -> bool (valid jika 15 digit angka)',
            'Implementasikan fn hitung_pph21(gaji_tahunan: f64, menikah: bool) -> (f64, f64) yang return (pph21_tahunan, pph21_bulanan)',
            'Buat closure yang hitung diskon: jika total >= 10jt dapat 5%, jika >= 5jt dapat 2%, lainnya 0%',
            'Gunakan iterator: dari vec nominal, filter >= 1jt, kalikan 1.11 (tambah PPN), sum.',
          ],
          starterCode: `// TODO 1: implementasi
fn celsius_ke_fahrenheit(c: f64) -> f64 {
    todo!()
}

// TODO 2: implementasi
fn validasi_npwp(npwp: &str) -> bool {
    todo!()
}

// TODO 3: implementasi (return tuple)
fn hitung_pph21(gaji_tahunan: f64, menikah: bool) -> (f64, f64) {
    todo!()
}

fn main() {
    // Test TODO 1
    println!("0°C   = {:.1}°F", celsius_ke_fahrenheit(0.0));    // 32.0
    println!("100°C = {:.1}°F", celsius_ke_fahrenheit(100.0));  // 212.0

    // Test TODO 2
    println!("Valid: {}", validasi_npwp("12.345.678.9-012.345")); // true
    println!("Valid: {}", validasi_npwp("123-INVALID"));          // false

    // Test TODO 3
    let (tahunan, bulanan) = hitung_pph21(120_000_000.0, true);
    println!("PPh21/tahun: {tahunan:.0}, /bulan: {bulanan:.0}");

    // TODO 4: buat closure hitung_diskon
    // let hitung_diskon = |total: f64| -> f64 { ... };
    // println!("Diskon 12jt: {:.0}", hitung_diskon(12_000_000.0));  // 600000
    // println!("Diskon 6jt:  {:.0}", hitung_diskon(6_000_000.0));   // 120000
    // println!("Diskon 2jt:  {:.0}", hitung_diskon(2_000_000.0));   // 0

    // TODO 5: iterator chain
    let nominal = vec![500_000.0, 2_000_000.0, 1_500_000.0, 800_000.0, 5_000_000.0];
    // filter >= 1jt, tambah PPN 11%, sum
    // Expected: "Total + PPN: ~9405000"
}`,
          hints: [
            'celsius_ke_fahrenheit: return c * 9.0 / 5.0 + 32.0',
            'validasi_npwp: hitung digit dengan .chars().filter(|c| c.is_ascii_digit()).count() == 15',
            'hitung_pph21: ptkp = if menikah { 58_500_000.0 } else { 54_000_000.0 }; pkp = (gaji - ptkp).max(0.0); pajak = pkp * 0.05;',
            'Closure: |total: f64| if total >= 10_000_000.0 { total * 0.05 } else if total >= 5_000_000.0 { total * 0.02 } else { 0.0 }',
            'nominal.iter().filter(|&&n| n >= 1_000_000.0).map(|&n| n * 1.11).sum::<f64>()',
          ]
        }
      },
      {
        id: 'control_flow',
        title: 'Control Flow',
        content: `
# Control Flow

## if / else — Bisa Jadi Ekspresi!

\`\`\`rust
let nilai = 85;

// Biasa
if nilai >= 80 { println!("Lulus"); } else { println!("Tidak lulus"); }

// Sebagai ekspresi — assign langsung (seperti ternary)
let status = if nilai >= 80 { "Lulus" } else { "Tidak lulus" };
\`\`\`

## match — Switch Case yang Powerful

\`\`\`rust
let kode = 404;
match kode {
    200        => println!("OK"),
    201        => println!("Created"),
    400..=499  => println!("Client Error"),   // range
    500 | 503  => println!("Server Error"),   // multiple
    _          => println!("Unknown"),        // wajib ada!
}
\`\`\`

<div class="concept-box warn">
<strong>match harus exhaustive!</strong> Semua kemungkinan harus di-cover. Compiler error kalau ada yang ketinggalan.
</div>

## for — Iterasi

\`\`\`rust
// Range
for i in 1..=5 { print!("{i} "); }   // 1 2 3 4 5 (inklusif)
for i in 0..5  { print!("{i} "); }   // 0 1 2 3 4 (eksklusif)

// Array/Vec
let buah = vec!["apel", "mangga", "jeruk"];
for b in &buah { println!("{b}"); }

// Dengan index
for (i, b) in buah.iter().enumerate() {
    println!("{i}: {b}");
}
\`\`\`

## loop — Infinite Loop dengan Return Value

\`\`\`rust
let mut counter = 0;
let hasil = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }  // return value!
};
println!("{hasil}"); // 20
\`\`\`
        `,
        defaultCode: `fn main() {
    // if sebagai ekspresi
    let skor = 87;
    let grade = if skor >= 90 { "A" }
                else if skor >= 80 { "B" }
                else if skor >= 70 { "C" }
                else { "D" };
    println!("Grade: {grade}");

    // match
    let status_http = 404;
    let pesan = match status_http {
        200        => "OK",
        201        => "Created",
        400        => "Bad Request",
        404        => "Not Found",
        500..=599  => "Server Error",
        _          => "Unknown",
    };
    println!("HTTP {status_http}: {pesan}");

    // for dengan enumerate
    let faktur = vec!["FKT-001", "FKT-002", "FKT-003"];
    for (i, f) in faktur.iter().enumerate() {
        println!("{}: {}", i + 1, f);
    }

    // FizzBuzz
    for i in 1..=15 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _      => println!("{i}"),
        }
    }
}`,
        exercise: {
          title: 'Exercise: Control Flow',
          desc: 'Selesaikan semua TODO!',
          tasks: [
            'Gunakan match untuk kategorikan omzet: <500jt="Sangat Kecil", <4.8M="Kecil", <50M="Menengah", lainnya="Besar"',
            'Buat loop hitung bunga berbunga: modal 10jt, bunga 5%/tahun, sampai dobel. Print berapa tahun.',
            'FizzBuzz versi pajak 1-30: habis 3→"PPh", habis 5→"PPN", habis 15→"PPh+PPN", lainnya print angka',
            'Gunakan for + enumerate untuk print daftar faktur dengan nomor urut, dan hitung total di akhir.',
          ],
          starterCode: `fn main() {
    // TODO 1: Kategori omzet dengan match
    let omzet: u64 = 2_500_000_000;
    // Expected: "Omzet 2.5M: Menengah"


    // TODO 2: Bunga berbunga
    // Modal 10jt, bunga 5%/tahun, hitung tahun sampai dobel (>= 20jt)
    // Expected: "Modal dobel setelah X tahun"


    // TODO 3: FizzBuzz versi pajak (1-30)
    // habis 3 → "PPh", habis 5 → "PPN", habis 15 → "PPh+PPN"


    // TODO 4: Daftar faktur + total
    let fakturs = vec![
        ("FKT-001", 1_500_000.0_f64),
        ("FKT-002", 2_300_000.0),
        ("FKT-003", 800_000.0),
        ("FKT-004", 4_200_000.0),
    ];
    // Print: "1. FKT-001 — Rp 1500000"
    // Di akhir: "Total: Rp X"

}`,
          hints: [
            'match omzet { 0..=499_999_999 => "Sangat Kecil", ... }  — gunakan range dengan u64',
            'let mut modal = 10_000_000.0_f64; let mut tahun = 0; loop { modal *= 1.05; tahun += 1; if modal >= 20_000_000.0 { break; } }',
            'for i in 1..=30 { match (i % 3, i % 5) { (0,0)=>"PPh+PPN", (0,_)=>"PPh", (_,0)=>"PPN", _=>print i } }',
            'for (i, (nomor, total)) in fakturs.iter().enumerate() { ... } lalu sum dengan .iter().map(|(_, t)| t).sum::<f64>()',
          ]
        }
      },
    ]
  },
  {
    id: '02_ownership',
    title: '02 · Ownership',
    icon: '🔐',
    lessons: [
      {
        id: 'ownership',
        title: 'Ownership',
        content: `
# Ownership — Konsep Paling Unik Rust

Ini yang membedakan Rust dari semua bahasa lain. Pahami ini, dan kamu sudah menguasai 50% Rust!

## Kenapa Ownership?

Rust tidak punya garbage collector (GC), tapi juga tidak pakai manual \`malloc/free\`.

\`\`\`
PHP/Python/Go/Java  → GC (otomatis, tapi GC pause)
C/C++               → Manual (cepat, tapi berbahaya)
Rust                → Ownership (cepat + aman, dijamin compiler) ✅
\`\`\`

## 3 Aturan Ownership

1. **Setiap nilai punya satu owner**
2. **Hanya boleh ada SATU owner di satu waktu**
3. **Saat owner keluar scope → nilai di-drop (memory bebas)**

## Move — Ownership Berpindah

\`\`\`rust
let s1 = String::from("halo");
let s2 = s1;          // ownership PINDAH ke s2

println!("{s1}");     // ❌ COMPILE ERROR! s1 sudah tidak valid
println!("{s2}");     // ✅ OK
\`\`\`

<div class="concept-box warn">
Tipe primitif (\`i32, f64, bool, char\`) di-COPY bukan di-move karena ukurannya fix dan kecil.
</div>

## Clone — Kalau Butuh Copy Data Heap

\`\`\`rust
let s1 = String::from("halo");
let s2 = s1.clone();  // deep copy — s1 dan s2 keduanya valid

println!("{s1}");     // ✅
println!("{s2}");     // ✅
\`\`\`

## Borrowing — Pinjam Tanpa Move

\`\`\`rust
fn panjang(s: &String) -> usize { s.len() }

let nama = String::from("pajak.io");
let p = panjang(&nama);  // pinjam dengan &
println!("{nama}: {p}"); // ✅ nama masih valid!
\`\`\`

## Aturan Borrowing

\`\`\`rust
// ✅ Boleh banyak immutable reference
let r1 = &s;
let r2 = &s;

// ✅ Boleh SATU mutable reference
let r3 = &mut s;

// ❌ TIDAK BOLEH: mutable + immutable bersamaan
\`\`\`
        `,
        defaultCode: `fn main() {
    // Primitif: di-COPY
    let x = 5;
    let y = x;
    println!("x={x}, y={y}"); // keduanya valid

    // String: di-MOVE
    let s1 = String::from("halo");
    let s2 = s1; // s1 sekarang tidak valid
    // println!("{s1}"); // ERROR: s1 sudah moved
    println!("{s2}"); // OK

    // Clone: keduanya valid
    let a = String::from("pajak.io");
    let b = a.clone();
    println!("{a} | {b}");

    // Borrowing: pinjam tanpa ambil ownership
    let nama = String::from("Bintang");
    let panjang = hitung_panjang(&nama); // pakai &
    println!("{nama}: {panjang} karakter"); // nama masih valid!

    // Mutable reference
    let mut faktur = String::from("FKT-001");
    tambah_suffix(&mut faktur);
    println!("{faktur}");
}

fn hitung_panjang(s: &String) -> usize {
    s.len()
}

fn tambah_suffix(s: &mut String) {
    s.push_str("/2024");
}`,
        exercise: {
          title: 'Exercise: Ownership & Borrowing',
          desc: 'Fix kode yang error dan lengkapi implementasi!',
          tasks: [
            'Fix: buat s1 masih valid setelah di-assign ke s2 (gunakan clone)',
            'Implementasikan fn hitung_panjang(s: &String) -> usize',
            'Implementasikan fn tambah_ppn(s: &mut String) yang append " (+PPN 11%)"',
            'Fix aturan borrowing yang dilanggar di TODO 4',
            'Implementasikan fn kata_pertama(s: &str) -> &str',
          ],
          starterCode: `fn main() {
    // TODO 1: Fix ini supaya s1 masih bisa dipakai
    let s1 = String::from("FKT-001");
    let s2 = s1; // ownership pindah!
    // println!("{s1}"); // ini harus bisa jalan
    println!("{s2}");

    // TODO 2: Panggil hitung_panjang dengan benar
    let faktur = String::from("FKT-001");
    // let p = hitung_panjang(...);
    // println!("Panjang {faktur}: {p}");

    // TODO 3: Panggil tambah_ppn
    let mut item = String::from("Laptop Lenovo");
    // tambah_ppn(...);
    // println!("{item}");  // Expected: "Laptop Lenovo (+PPN 11%)"

    // TODO 4: Fix pelanggaran borrowing
    let mut s = String::from("halo");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // ← ini masalahnya! gimana fix-nya?
    println!("{r1} {r2} {r3}");

    // TODO 5: Panggil kata_pertama
    let kalimat = String::from("Nomor Faktur Pajak");
    // let pertama = kata_pertama(&kalimat);
    // println!("Pertama: {pertama}");  // "Nomor"
}

fn hitung_panjang(s: &String) -> usize {
    todo!()
}

fn tambah_ppn(s: &mut String) {
    todo!()
}

fn kata_pertama(s: &str) -> &str {
    todo!()
}`,
          hints: [
            'Gunakan s1.clone() sebelum assign ke s2',
            'hitung_panjang: cukup return s.len()',
            'tambah_ppn: gunakan s.push_str(" (+PPN 11%)")',
            'Pindahkan println! r1 r2 SEBELUM bikin r3. Setelah r1 r2 tidak dipakai lagi, baru buat r3.',
            'kata_pertama: s.split_whitespace().next().unwrap_or("")',
          ]
        }
      }
    ]
  },
  {
    id: '03_enums',
    title: '03 · Enums & Option',
    icon: '🎯',
    lessons: [
      {
        id: 'enums_option',
        title: 'Enum & Option<T>',
        content: `
# Enum & Option\<T\>

## Enum Biasa

\`\`\`rust
enum StatusFaktur {
    Draft,
    Dikirim,
    Dibayar,
    Dibatalkan,
}
\`\`\`

## Enum dengan Data — Unik di Rust!

Setiap variant bisa bawa tipe data berbeda:

\`\`\`rust
enum StatusSPT {
    BelumLapor,
    Lapor { tanggal: String },
    Terlambat { tanggal: String, hari: u32 },
    Pembetulan { ke: u32 },
}
\`\`\`

<div class="concept-box">
Ini yang tidak bisa dilakukan enum di PHP/Java — di sana enum hanya bisa menyimpan nilai konstan sederhana.
</div>

## Option\<T\> — Pengganti Null

Di Rust **tidak ada null**. Diganti \`Option<T>\`:

\`\`\`rust
fn cari_faktur(id: u32) -> Option<String> {
    if id == 1 { Some(String::from("FKT-001")) }
    else { None }
}

// WAJIB handle keduanya
match cari_faktur(1) {
    Some(f) => println!("Ditemukan: {f}"),
    None    => println!("Tidak ada"),
}

// Cara ringkas
if let Some(f) = cari_faktur(2) {
    println!("{f}");
}

// Dengan default
let f = cari_faktur(99).unwrap_or(String::from("TIDAK ADA"));
\`\`\`

## Pattern Matching

\`\`\`rust
match status {
    StatusSPT::BelumLapor      => "Belum lapor",
    StatusSPT::Lapor { tanggal } => format!("Lapor: {tanggal}"),
    StatusSPT::Terlambat { hari, .. } => format!("Terlambat {hari} hari"),
    StatusSPT::Pembetulan { ke }      => format!("Pembetulan ke-{ke}"),
}
\`\`\`
        `,
        defaultCode: `#[derive(Debug)]
enum JenisPajak {
    PPN,
    PPh21 { tarif: f64 },
    PPh23 { objek: String, tarif: f64 },
}

impl JenisPajak {
    fn nama(&self) -> &str {
        match self {
            JenisPajak::PPN          => "PPN",
            JenisPajak::PPh21 { .. } => "PPh Pasal 21",
            JenisPajak::PPh23 { .. } => "PPh Pasal 23",
        }
    }

    fn tarif(&self) -> f64 {
        match self {
            JenisPajak::PPN                  => 0.11,
            JenisPajak::PPh21 { tarif }      => *tarif,
            JenisPajak::PPh23 { tarif, .. }  => *tarif,
        }
    }

    fn hitung(&self, dpp: f64) -> f64 {
        dpp * self.tarif()
    }
}

fn cari_wp(npwp: &str) -> Option<String> {
    match npwp {
        "12.345.678.9-012.345" => Some(String::from("PT Maju Jaya")),
        "98.765.432.1-098.765" => Some(String::from("CV Sejahtera")),
        _ => None,
    }
}

fn main() {
    let daftar = vec![
        JenisPajak::PPN,
        JenisPajak::PPh21 { tarif: 0.05 },
        JenisPajak::PPh23 { objek: "Jasa".to_string(), tarif: 0.02 },
    ];

    let dpp = 10_000_000.0_f64;
    for p in &daftar {
        println!("{}: Rp {:.0}", p.nama(), p.hitung(dpp));
    }

    // Option
    println!("\\n--- Cari WP ---");
    for npwp in ["12.345.678.9-012.345", "00.000.000.0-000.000"] {
        match cari_wp(npwp) {
            Some(nama) => println!("✅ {npwp} → {nama}"),
            None       => println!("❌ {npwp} → Tidak ditemukan"),
        }
    }
}`,
        exercise: {
          title: 'Exercise: Enum & Option',
          desc: 'Buat sistem status SPT dan validasi faktur!',
          tasks: [
            'Buat enum StatusSPT dengan variant: BelumLapor, Lapor{tanggal}, Terlambat{tanggal, hari}, Pembetulan{ke}',
            'Implementasikan method deskripsi(&self) -> String dan kena_denda(&self) -> bool untuk StatusSPT',
            'Buat fn cari_wp(npwp: &str) -> Option<String> dengan minimal 3 data hardcoded',
            'Gunakan if let untuk filter hanya StatusSPT yang kena denda dari sebuah Vec',
            'Chain Option: cari_wp → map ke uppercase → unwrap_or "TIDAK DIKENAL"',
          ],
          starterCode: `#[derive(Debug)]
enum StatusSPT {
    // TODO 1: tambahkan variant
}

impl StatusSPT {
    // TODO 2: implementasikan
    fn deskripsi(&self) -> String {
        todo!()
    }

    fn kena_denda(&self) -> bool {
        todo!()
    }
}

// TODO 3: implementasikan
fn cari_wp(npwp: &str) -> Option<String> {
    todo!()
}

fn main() {
    let statuses = vec![
        // StatusSPT::BelumLapor,
        // StatusSPT::Lapor { tanggal: "2024-03-31".to_string() },
        // StatusSPT::Terlambat { tanggal: "2024-04-05".to_string(), hari: 5 },
        // StatusSPT::Pembetulan { ke: 1 },
    ];

    // Print semua deskripsi + kena denda
    for s in &statuses {
        println!("{} | denda: {}", s.deskripsi(), s.kena_denda());
    }

    // TODO 4: filter yang kena denda saja


    // TODO 5: chain Option
    // let nama = cari_wp("12.345...").map(...).unwrap_or(...);
}`,
          hints: [
            'enum StatusSPT { BelumLapor, Lapor { tanggal: String }, Terlambat { tanggal: String, hari: u32 }, Pembetulan { ke: u32 } }',
            'match self { StatusSPT::BelumLapor => "Belum lapor".to_string(), ... }',
            'Hanya Terlambat yang kena_denda == true',
            'statuses.iter().filter(|s| s.kena_denda()).for_each(|s| println!("{}", s.deskripsi()))',
            'cari_wp(npwp).map(|n| n.to_uppercase()).unwrap_or("TIDAK DIKENAL".to_string())',
          ]
        }
      }
    ]
  },
  {
    id: '04_collections',
    title: '04 · Collections',
    icon: '📦',
    lessons: [
      {
        id: 'vec_hashmap',
        title: 'Vec & HashMap',
        content: `
# Vec & HashMap

## Vec\<T\> — Dynamic Array

\`\`\`rust
let mut v = vec![1, 2, 3];
v.push(4);
v.pop();          // return Option<T>

// Akses aman
match v.get(10) {
    Some(val) => println!("{val}"),
    None      => println!("tidak ada"),  // tidak panic!
}
\`\`\`

## Iterator — Cara Idiomatis

\`\`\`rust
let nominal: Vec<f64> = vec![500_000.0, 2_000_000.0, 1_500_000.0];

// filter + map + collect
let kena_ppn: Vec<f64> = nominal.iter()
    .filter(|&&n| n >= 1_000_000.0)
    .map(|&n| n * 1.11)
    .collect();

let total: f64 = nominal.iter().sum();
let max  = nominal.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
\`\`\`

## HashMap\<K, V\> — Key-Value Store

\`\`\`rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("PPN", 0.11);

// Akses → Option<&V>
if let Some(&tarif) = map.get("PPN") {
    println!("Tarif: {tarif}");
}

// Entry API — insert kalau belum ada
let counter = map.entry("PPh").or_insert(0.0);
*counter += 1.0;
\`\`\`

## String vs &str

\`\`\`rust
let s1: &str   = "literal";           // fixed, di binary
let s2: String = String::from("..."); // owned, di heap, mutable

// Fungsi: pakai &str (lebih fleksibel)
fn tampilkan(s: &str) { println!("{s}"); }
tampilkan("literal");  // ✅
tampilkan(&s2);        // ✅ otomatis deref
\`\`\`
        `,
        defaultCode: `use std::collections::HashMap;

fn main() {
    // Vec operations
    let mut faktur = vec![
        ("FKT-001", 1_500_000.0_f64),
        ("FKT-002", 2_300_000.0),
        ("FKT-003", 800_000.0),
        ("FKT-004", 5_000_000.0),
    ];

    // Sort by total descending
    faktur.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let total: f64 = faktur.iter().map(|(_, t)| t).sum();
    let ppn:   f64 = faktur.iter().map(|(_, t)| t * 0.11).sum();

    println!("=== Faktur ===");
    for (nomor, total) in &faktur {
        println!("{nomor}: Rp {total:.0}");
    }
    println!("Total: {total:.0} | PPN: {ppn:.0}");

    // HashMap — group by status
    let data = vec![
        ("FKT-001", "lunas",   1_500_000.0_f64),
        ("FKT-002", "pending", 2_300_000.0),
        ("FKT-003", "lunas",   800_000.0),
        ("FKT-004", "pending", 5_000_000.0),
    ];

    let mut by_status: HashMap<&str, f64> = HashMap::new();
    for (_, status, total) in &data {
        *by_status.entry(status).or_insert(0.0) += total;
    }

    println!("\\n=== By Status ===");
    let mut sorted: Vec<_> = by_status.iter().collect();
    sorted.sort_by_key(|(k, _)| **k);
    for (status, total) in sorted {
        println!("{status}: Rp {total:.0}");
    }
}`,
        exercise: {
          title: 'Exercise: Collections',
          desc: 'Proses data faktur dengan Vec dan HashMap!',
          tasks: [
            'Dari Vec nominal, hitung: max, min, rata-rata, dan jumlah yang >= 1jt',
            'Group Vec transaksi by kategori menggunakan HashMap, print total per kategori',
            'Word frequency: hitung berapa kali tiap kata muncul di string, print top 3',
            'Dari Vec tuple (nama, skor), cari nilai tertinggi dan terendah menggunakan iterator',
          ],
          starterCode: `use std::collections::HashMap;

fn main() {
    // TODO 1: statistik Vec
    let nominal = vec![
        500_000.0_f64, 2_000_000.0, 1_500_000.0,
        800_000.0, 5_000_000.0, 300_000.0, 3_200_000.0,
    ];
    // Hitung: max, min, rata-rata, count >= 1jt
    // Expected: "Max: 5000000, Min: 300000, Avg: ..., Count>=1jt: 4"


    // TODO 2: Group by kategori
    let transaksi = vec![
        ("Penjualan", 5_000_000.0_f64),
        ("Pembelian", 2_000_000.0),
        ("Penjualan", 3_500_000.0),
        ("Jasa",      1_200_000.0),
        ("Pembelian", 800_000.0),
        ("Jasa",      2_500_000.0),
    ];
    // Expected:
    // Jasa: Rp 3700000
    // Pembelian: Rp 2800000
    // Penjualan: Rp 8500000


    // TODO 3: Word frequency
    let teks = "pajak penghasilan pajak pertambahan nilai pajak penjualan penghasilan";
    // Hitung frekuensi tiap kata
    // Print top 3 kata paling sering
    // Expected: "pajak: 3, penghasilan: 2, ..."


    // TODO 4: Max dan min dari Vec tuple
    let siswa = vec![
        ("Budi", 85), ("Siti", 92), ("Agus", 78),
        ("Rina", 95), ("Doni", 70),
    ];
    // Cari siswa dengan nilai tertinggi dan terendah
    // Expected: "Tertinggi: Rina (95), Terendah: Doni (70)"
}`,
          hints: [
            'max: nominal.iter().cloned().fold(f64::NEG_INFINITY, f64::max); min: f64::INFINITY; avg: sum/len',
            'for (kat, total) in &transaksi { *by_kat.entry(kat).or_insert(0.0) += total; }',
            'for kata in teks.split_whitespace() { *freq.entry(kata).or_insert(0) += 1; } lalu sort by value desc',
            'siswa.iter().max_by_key(|(_, s)| s) dan min_by_key',
          ]
        }
      }
    ]
  },
  {
    id: '05_error_handling',
    title: '05 · Error Handling',
    icon: '⚠️',
    lessons: [
      {
        id: 'result',
        title: 'Result & Error Handling',
        content: `
# Error Handling di Rust

Tidak ada try/catch. Error adalah **nilai return biasa** yang harus di-handle.

## Result\<T, E\>

\`\`\`rust
fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Tidak bisa bagi dengan nol"))
    } else {
        Ok(a / b)
    }
}

match bagi(10.0, 0.0) {
    Ok(h)  => println!("Hasil: {h}"),
    Err(e) => println!("Error: {e}"),
}
\`\`\`

## Operator ? — Propagate Error

\`\`\`rust
fn proses(a: &str, b: &str) -> Result<f64, String> {
    let x: f64 = a.parse().map_err(|e: _| e.to_string())?;
    let y: f64 = b.parse().map_err(|e: _| e.to_string())?;
    Ok(x + y)
}
\`\`\`

<div class="concept-box">
<strong>Operator ?</strong> = kalau Err, langsung return Err ke caller. Seperti throw tapi eksplisit dan tracked oleh compiler.
</div>

## Custom Error

\`\`\`rust
#[derive(Debug)]
enum FakturError {
    NpwpTidakValid(String),
    TotalNegatif(f64),
    DjpGagal { kode: u16, pesan: String },
}

impl std::fmt::Display for FakturError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FakturError::NpwpTidakValid(s) =>
                write!(f, "NPWP tidak valid: {s}"),
            FakturError::TotalNegatif(v) =>
                write!(f, "Total negatif: {v}"),
            FakturError::DjpGagal { kode, pesan } =>
                write!(f, "DJP [{kode}]: {pesan}"),
        }
    }
}
\`\`\`

## Handle Result

\`\`\`rust
hasil.unwrap()                    // panic kalau Err
hasil.unwrap_or(default)          // default kalau Err
hasil.unwrap_or_else(|e| ...)     // closure kalau Err
hasil.map(|v| v * 2)              // transform Ok value
hasil.map_err(|e| e.to_string())  // transform Err
\`\`\`
        `,
        defaultCode: `use std::fmt;

#[derive(Debug)]
enum PajakError {
    NpwpTidakValid { npwp: String },
    TotalNegatif(f64),
    ParseError(String),
}

impl fmt::Display for PajakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PajakError::NpwpTidakValid { npwp } =>
                write!(f, "NPWP tidak valid: '{npwp}'"),
            PajakError::TotalNegatif(v) =>
                write!(f, "Total negatif: {v}"),
            PajakError::ParseError(s) =>
                write!(f, "Parse error: {s}"),
        }
    }
}

fn validasi_npwp(npwp: &str) -> Result<(), PajakError> {
    let digit = npwp.chars().filter(|c| c.is_ascii_digit()).count();
    if digit != 15 {
        return Err(PajakError::NpwpTidakValid { npwp: npwp.to_string() });
    }
    Ok(())
}

fn parse_total(s: &str) -> Result<f64, PajakError> {
    s.trim().parse::<f64>()
        .map_err(|e| PajakError::ParseError(e.to_string()))
}

fn buat_faktur(npwp: &str, total_str: &str) -> Result<String, PajakError> {
    validasi_npwp(npwp)?;
    let total = parse_total(total_str)?;
    if total <= 0.0 {
        return Err(PajakError::TotalNegatif(total));
    }
    Ok(format!("Faktur OK | NPWP: {npwp} | Total: {total:.0}"))
}

fn main() {
    let cases = vec![
        ("12.345.678.9-012.345", "1500000"),
        ("INVALID", "1500000"),
        ("12.345.678.9-012.345", "abc"),
        ("12.345.678.9-012.345", "-500"),
    ];

    for (npwp, total) in cases {
        match buat_faktur(npwp, total) {
            Ok(msg)  => println!("✅ {msg}"),
            Err(e)   => println!("❌ {e}"),
        }
    }
}`,
        exercise: {
          title: 'Exercise: Error Handling',
          desc: 'Buat sistem validasi faktur dengan error handling yang proper!',
          tasks: [
            'Buat enum AppError dengan variant: NpwpInvalid, TotalInvalid, NomorInvalid, ParseError',
            'Implement Display untuk AppError',
            'Buat fn validasi_faktur(nomor, npwp, total_str) -> Result<String, AppError> dengan ? operator',
            'Kumpulkan SEMUA error sekaligus (bukan berhenti di error pertama) menggunakan Vec<AppError>',
            'Gunakan unwrap_or_else untuk fallback ke nilai default saat error',
          ],
          starterCode: `use std::fmt;

// TODO 1: buat enum
#[derive(Debug)]
enum AppError {
    // NpwpInvalid(String),
    // TotalInvalid(String),
    // NomorInvalid(String),
    // ParseError(String),
}

// TODO 2: implement Display
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

// TODO 3: implementasi dengan ?
fn validasi_faktur(nomor: &str, npwp: &str, total_str: &str) -> Result<String, AppError> {
    todo!()
}

// TODO 4: kumpulkan semua error (jangan berhenti di pertama)
fn validasi_semua(nomor: &str, npwp: &str, total_str: &str) -> Vec<AppError> {
    let mut errors = Vec::new();
    // cek nomor
    // cek npwp
    // cek total
    errors
}

fn main() {
    // Test validasi_faktur
    let cases = vec![
        ("FKT-001", "12.345.678.9-012.345", "1500000"),  // valid
        ("INV-001", "12.345.678.9-012.345", "1500000"),  // nomor salah
        ("FKT-002", "123",                  "1500000"),  // npwp salah
        ("FKT-003", "12.345.678.9-012.345", "-500"),     // total negatif
    ];

    for (nomor, npwp, total) in &cases {
        match validasi_faktur(nomor, npwp, total) {
            Ok(msg)  => println!("✅ {msg}"),
            Err(e)   => println!("❌ {e}"),
        }
    }

    // TODO 5: gunakan unwrap_or_else
    let total: f64 = "bukan_angka".parse().unwrap_or_else(|_| {
        println!("Parse gagal, pakai default 0");
        0.0
    });
    println!("Total: {total}");
}`,
          hints: [
            'enum AppError { NpwpInvalid(String), TotalInvalid(String), NomorInvalid(String), ParseError(String) }',
            'match self { AppError::NpwpInvalid(s) => write!(f, "NPWP invalid: {s}"), ... }',
            'Gunakan ? setelah setiap langkah validasi. Map error dengan .map_err(|_| AppError::ParseError(...))',
            'Jangan pakai ?, pakai if let Err(e) = ... { errors.push(e); } untuk kumpulkan semua',
            '"bukan_angka".parse::<f64>().unwrap_or_else(|_| 0.0)',
          ]
        }
      }
    ]
  },
  {
    id: '06_traits',
    title: '06 · Traits & Generics',
    icon: '🧩',
    lessons: [
      {
        id: 'traits_generics',
        title: 'Traits & Generics',
        content: `
# Traits & Generics

## Trait — Kontrak Behavior

Mirip interface PHP/Java, tapi lebih powerful.

\`\`\`rust
trait HitungPajak {
    fn tarif(&self) -> f64;          // wajib diimplementasikan
    fn nama(&self) -> &str;

    fn hitung(&self, dpp: f64) -> f64 {  // default method
        dpp * self.tarif()
    }
}

struct PPN;
impl HitungPajak for PPN {
    fn tarif(&self) -> f64 { 0.11 }
    fn nama(&self)  -> &str { "PPN" }
}
\`\`\`

## Trait sebagai Parameter

\`\`\`rust
// impl Trait syntax
fn cetak(pajak: &impl HitungPajak, dpp: f64) {
    println!("{}: {:.0}", pajak.nama(), pajak.hitung(dpp));
}

// Trait object — bisa campur berbagai tipe
let daftar: Vec<Box<dyn HitungPajak>> = vec![
    Box::new(PPN),
    Box::new(PPh23 { tarif: 0.02 }),
];
\`\`\`

## Generics — Kode untuk Semua Tipe

\`\`\`rust
fn terbesar<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    max
}

// Generic struct
struct Repo<T> { data: Vec<T> }

impl<T> Repo<T> {
    fn simpan(&mut self, item: T) { self.data.push(item); }
    fn semua(&self) -> &[T] { &self.data }
}
\`\`\`

<div class="concept-box tip">
<strong>Zero-cost abstraction:</strong> Generic di Rust dikompilasi jadi versi spesifik per tipe (monomorphization). Performa sama dengan kode non-generic!
</div>

## Standard Traits

\`\`\`rust
#[derive(Debug, Clone, PartialEq)]
struct Faktur { nomor: String, total: f64 }

// Display — untuk println! {}
impl std::fmt::Display for Faktur {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Faktur[{}] Rp {:.0}", self.nomor, self.total)
    }
}
\`\`\`
        `,
        defaultCode: `use std::fmt;

trait HitungPajak {
    fn tarif(&self) -> f64;
    fn nama(&self) -> &str;

    fn hitung(&self, dpp: f64) -> f64 { dpp * self.tarif() }

    fn info(&self, dpp: f64) {
        println!("  {}: {:.0}% → Rp {:.0}", self.nama(), self.tarif()*100.0, self.hitung(dpp));
    }
}

struct PPN;
struct PPh21 { ptkp: f64 }
struct PPh23 { tarif_pct: f64 }

impl HitungPajak for PPN {
    fn tarif(&self) -> f64 { 0.11 }
    fn nama(&self)  -> &str { "PPN" }
}

impl HitungPajak for PPh21 {
    fn tarif(&self) -> f64 { 0.05 }
    fn nama(&self)  -> &str { "PPh 21" }
    fn hitung(&self, penghasilan: f64) -> f64 {
        (penghasilan - self.ptkp).max(0.0) * self.tarif()
    }
}

impl HitungPajak for PPh23 {
    fn tarif(&self) -> f64 { self.tarif_pct }
    fn nama(&self)  -> &str { "PPh 23" }
}

// Generic repo
struct Repo<T> { data: Vec<T> }
impl<T> Repo<T> {
    fn new() -> Self { Repo { data: Vec::new() } }
    fn simpan(&mut self, item: T) { self.data.push(item); }
    fn count(&self) -> usize { self.data.len() }
}

#[derive(Debug, Clone)]
struct Faktur { nomor: String, total: f64 }

impl fmt::Display for Faktur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Faktur {} | Rp {:.0}", self.nomor, self.total)
    }
}

fn main() {
    let daftar: Vec<Box<dyn HitungPajak>> = vec![
        Box::new(PPN),
        Box::new(PPh21 { ptkp: 58_500_000.0 }),
        Box::new(PPh23 { tarif_pct: 0.02 }),
    ];

    println!("=== Kalkulasi Pajak (DPP: 100jt) ===");
    let dpp = 100_000_000.0;
    for p in &daftar { p.info(dpp); }

    // Generic repo
    let mut repo: Repo<Faktur> = Repo::new();
    repo.simpan(Faktur { nomor: "FKT-001".to_string(), total: 1_500_000.0 });
    repo.simpan(Faktur { nomor: "FKT-002".to_string(), total: 2_300_000.0 });
    println!("\\nRepo berisi {} faktur", repo.count());
    println!("{}", repo.data[0]); // pakai Display
}`,
        exercise: {
          title: 'Exercise: Traits & Generics',
          desc: 'Buat trait system untuk domain pajak!',
          tasks: [
            'Buat trait Validatable dengan method validate(&self) -> Result<(), String>',
            'Implementasikan Validatable untuk struct NPWP dan struct Faktur',
            'Buat generic function fn validasi_semua<T: Validatable>(items: &[T]) -> Vec<String> yang return semua error',
            'Buat generic struct Stack<T> dengan push, pop, peek, is_empty',
            'Implement Display untuk NPWP dan Faktur',
          ],
          starterCode: `use std::fmt;

// TODO 1: buat trait
trait Validatable {
    fn validate(&self) -> Result<(), String>;
}

// TODO 2: implementasikan untuk struct berikut
struct NPWP { raw: String }

struct Faktur {
    nomor: String,
    npwp:  NPWP,
    total: f64,
}

// TODO 4: generic Stack
struct Stack<T> {
    // data: ...
}

impl<T> Stack<T> {
    fn new() -> Self { todo!() }
    fn push(&mut self, item: T) { todo!() }
    fn pop(&mut self) -> Option<T> { todo!() }
    fn peek(&self) -> Option<&T> { todo!() }
    fn is_empty(&self) -> bool { todo!() }
}

// TODO 3: generic function
fn validasi_semua<T: Validatable>(items: &[T]) -> Vec<String> {
    todo!()
}

fn main() {
    // Test NPWP
    let valid   = NPWP { raw: "12.345.678.9-012.345".to_string() };
    let invalid = NPWP { raw: "123".to_string() };
    println!("{:?}", valid.validate());   // Ok(())
    println!("{:?}", invalid.validate()); // Err("...")

    // Test validasi_semua
    let npwps = vec![
        NPWP { raw: "12.345.678.9-012.345".to_string() },
        NPWP { raw: "INVALID".to_string() },
        NPWP { raw: "98.765.432.1-098.765".to_string() },
    ];
    let errors = validasi_semua(&npwps);
    println!("Errors: {:?}", errors);

    // Test Stack
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1); stack.push(2); stack.push(3);
    println!("Peek: {:?}", stack.peek()); // Some(3)
    println!("Pop:  {:?}", stack.pop());  // Some(3)
    println!("Peek: {:?}", stack.peek()); // Some(2)
}`,
          hints: [
            'trait Validatable { fn validate(&self) -> Result<(), String>; }',
            'Untuk NPWP: hitung digit, kalau bukan 15 return Err. Untuk Faktur: validate npwp lalu cek total > 0',
            'validasi_semua: items.iter().filter_map(|item| item.validate().err()).collect()',
            'Stack<T> { data: Vec<T> }; push = data.push; pop = data.pop; peek = data.last()',
            'impl fmt::Display for NPWP { fn fmt ... { write!(f, "{}", self.raw) } }',
          ]
        }
      }
    ]
  },
  {
    id: '07_async',
    title: '07 · Async Rust',
    icon: '⚡',
    lessons: [
      {
        id: 'async_basic',
        title: 'Async / Await',
        content: `
# Async / Await di Rust

Rust punya async/await seperti JS/Python, tapi perlu runtime eksplisit (Tokio).

## Setup

Tambah ke \`Cargo.toml\`:
\`\`\`toml
[dependencies]
tokio = { version = "1", features = ["full"] }
\`\`\`

## Syntax Dasar

\`\`\`rust
// JS:   async function ambil() { await fetch(...) }
// Rust:
async fn ambil_data(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Data-{id}")
}

#[tokio::main]
async fn main() {
    let data = ambil_data(1).await;
    println!("{data}");
}
\`\`\`

## Sequential vs Concurrent

\`\`\`rust
// Sequential — lambat (300ms total)
let d1 = ambil(1).await;
let d2 = ambil(2).await;
let d3 = ambil(3).await;

// Concurrent — cepat (~100ms)
let (d1, d2, d3) = tokio::join!(ambil(1), ambil(2), ambil(3));
\`\`\`

## Spawn Task

\`\`\`rust
let handle = tokio::spawn(async {
    ambil_data(99).await
});
let hasil = handle.await.unwrap();
\`\`\`

## Error Handling di Async

\`\`\`rust
async fn kirim_ke_djp(faktur: &str) -> Result<String, String> {
    // ... simulasi API call
    Ok(format!("NSFP-{faktur}"))
}

match kirim_ke_djp("FKT-001").await {
    Ok(nsfp) => println!("✅ {nsfp}"),
    Err(e)   => println!("❌ {e}"),
}
\`\`\`

<div class="concept-box warn">
<strong>Catatan:</strong> Kode async butuh Cargo untuk run karena perlu dependensi Tokio. Contoh di sini tidak bisa dijalankan langsung di playground ini (tidak ada Tokio). Gunakan <code>cargo run</code> di lokal.
</div>

<div class="concept-box tip">
<strong>Tips buat pajak.io:</strong> Async berguna untuk: kirim batch e-Faktur ke DJP concurrent, call multiple API sekaligus, handle banyak request tanpa blocking.
</div>
        `,
        defaultCode: `// Kode ini membutuhkan Tokio — jalankan dengan cargo run
// Cargo.toml: tokio = { version = "1", features = ["full"] }

// Untuk demo di playground, kita simulasi dengan kode sync
// yang menunjukkan KONSEP yang sama

fn main() {
    // Konsep: sequential vs concurrent
    // Di async Rust, ini akan berjalan concurrent (paralel)

    let faktur_ids = vec!["FKT-001", "FKT-002", "FKT-003", "FKT-004"];

    println!("=== Simulasi Kirim Batch e-Faktur ===");
    println!("(Di production pakai tokio::join! untuk concurrent)\\n");

    // Sequential simulation
    let mut results = Vec::new();
    for id in &faktur_ids {
        let result = proses_faktur(id);
        results.push(result);
    }

    let mut sukses = 0;
    let mut gagal  = 0;
    for (id, result) in faktur_ids.iter().zip(&results) {
        match result {
            Ok(nsfp) => { println!("✅ {id} → {nsfp}"); sukses += 1; }
            Err(e)   => { println!("❌ {id} → {e}"); gagal += 1; }
        }
    }

    println!("\\nSukses: {sukses} | Gagal: {gagal}");
}

fn proses_faktur(id: &str) -> Result<String, String> {
    // Simulasi: FKT-003 selalu gagal
    if id == "FKT-003" {
        return Err(format!("DJP reject {id}: format tidak valid"));
    }
    Ok(format!("NSFP-{id}-2024"))
}`,
        exercise: {
          title: 'Exercise: Async Concepts',
          desc: 'Simulasikan pola async dengan sync code untuk memahami konsepnya.',
          tasks: [
            'Buat fn kirim_batch(ids: Vec<&str>) yang proses semua dan return (sukses, gagal)',
            'Implementasikan retry logic: coba 3x sebelum mark sebagai gagal',
            'Buat fn proses_dengan_timeout(id: &str, max_retry: u32) -> Result<String, String>',
            'Kumpulkan semua hasil dan print summary: total, sukses, gagal, rata-rata attempt',
          ],
          starterCode: `fn proses_faktur(id: &str, attempt: u32) -> Result<String, String> {
    // Simulasi: gagal di attempt 1-2, sukses di attempt 3
    // Kecuali "FKT-999" yang selalu gagal
    if id == "FKT-999" {
        return Err(format!("DJP permanently reject {id}"));
    }
    if attempt < 3 {
        return Err(format!("Timeout (attempt {attempt})"));
    }
    Ok(format!("NSFP-{id}-{attempt}"))
}

// TODO 1: implementasikan
fn kirim_batch(ids: &[&str]) -> (Vec<String>, Vec<String>) {
    // return (sukses, gagal)
    todo!()
}

// TODO 2 & 3: dengan retry
fn proses_dengan_retry(id: &str, max_retry: u32) -> Result<String, String> {
    // coba max_retry kali
    // return Ok jika berhasil, Err jika semua attempt gagal
    todo!()
}

fn main() {
    let ids = vec!["FKT-001", "FKT-002", "FKT-003", "FKT-999", "FKT-004"];

    println!("=== Tanpa Retry ===");
    // TODO: panggil kirim_batch, print hasilnya

    println!("\\n=== Dengan Retry (max 3x) ===");
    // TODO: panggil proses_dengan_retry untuk tiap id, print hasilnya

    // TODO 4: print summary
    // Expected:
    // Total: 5 | Sukses: X | Gagal: X
}`,
          hints: [
            'kirim_batch: for id in ids { match proses_faktur(id, 3) { Ok(r) => sukses.push(r), Err(e) => gagal.push(e) } }',
            'proses_dengan_retry: for attempt in 1..=max_retry { match proses_faktur(id, attempt) { Ok(r) => return Ok(r), Err(e) => if attempt == max_retry { return Err(e) } } }',
            'Collect hasil dari semua ids, filter Ok dan Err, hitung masing-masing',
            'println!("Total: {} | Sukses: {} | Gagal: {}", ids.len(), sukses.len(), gagal.len())',
          ]
        }
      }
    ]
  },

  {
    id: '08_studi_kasus',
    title: '08 · Studi Kasus: API BMKG',
    icon: '🌏',
    lessons: [
      {
        id: 'bmkg_parsing',
        title: 'Ambil & Parse Data Gempa',
        content: `
# Studi Kasus: Data Gempa BMKG

Sekarang kita pakai data **sungguhan**. BMKG menyediakan API publik yang gratis dan tanpa API key:

- \`autogempa.json\` — 1 gempa terbaru, \`gempa\` berbentuk **objek**
- \`gempaterkini.json\` — 15 gempa terkini M ≥ 5.0, \`gempa\` berbentuk **array**
- \`gempadirasakan.json\` — 15 gempa yang dirasakan warga, berbentuk **array**

Semuanya ada di \`https://data.bmkg.go.id/DataMKG/TEWS/\`

## Bentuk JSON-nya

\`\`\`json
{
  "Infogempa": {
    "gempa": {
      "Tanggal": "14 Agu 2026",
      "Jam": "08:14:48 WIB",
      "Coordinates": "5.36,125.34",
      "Magnitude": "5.3",
      "Kedalaman": "10 km",
      "Wilayah": "195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT",
      "Potensi": "Tidak berpotensi tsunami",
      "Dirasakan": "-"
    }
  }
}
\`\`\`

## Jebakan 1 — Semua Nilai Bertipe String

Lihat baik-baik: \`"Magnitude": "5.3"\` itu **String**, bukan angka. Begitu juga \`"Kedalaman": "10 km"\`.
Jadi konversinya kita kerjakan sendiri:

\`\`\`rust
fn magnitudo(&self) -> f64 {
    self.magnitude.trim().parse().unwrap_or(0.0)
}
fn kedalaman_km(&self) -> f64 {
    self.kedalaman
        .split_whitespace()          // "10 km" -> ["10", "km"]
        .next()                      // Option<&str>
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0)
}
\`\`\`

<div class="concept-box">
<strong>Beda dari JS/Python:</strong> di JS <code>JSON.parse</code> memberi kamu objek yang bisa diisi apa saja. Di Rust, bentuk datanya harus dideklarasikan dulu lewat struct — kalau JSON-nya tidak cocok, parsing gagal di situ juga, bukan meledak jauh di kemudian hari.
</div>

## Jebakan 2 — Key PascalCase

BMKG memakai \`"Tanggal"\`, konvensi Rust \`tanggal\`. Jembatannya \`#[serde(rename)]\`:

\`\`\`rust
#[derive(Debug, Deserialize)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
}
\`\`\`

## Jebakan 3 — Field yang Tidak Selalu Ada

\`Potensi\` tidak ada di \`gempadirasakan.json\`, \`Shakemap\` hanya ada di \`autogempa.json\`.
Kalau dideklarasikan sebagai \`String\` biasa, parsing akan **gagal total**. Solusinya \`Option<T>\`:

\`\`\`rust
#[serde(rename = "Potensi")]
potensi: Option<String>,
\`\`\`

## Jebakan 4 — "-" Artinya Kosong

BMKG mengisi \`"Dirasakan": "-"\` kalau tidak ada laporan. Ubah jadi \`None\` supaya rapi:

\`\`\`rust
fn dirasakan_bersih(&self) -> Option<&str> {
    let nilai = self.dirasakan.as_deref()?.trim();
    if nilai.is_empty() || nilai == "-" { None } else { Some(nilai) }
}
\`\`\`

## Klasifikasi Magnitudo

\`match\` dengan guard cocok sekali untuk ini:

\`\`\`rust
fn skala(&self) -> &'static str {
    match self.magnitudo() {
        m if m < 4.0 => "🟢 Kecil",
        m if m < 5.0 => "🟡 Ringan",
        m if m < 6.0 => "🟠 Sedang",
        m if m < 7.0 => "🔴 Kuat",
        _            => "🟣 Besar",
    }
}
\`\`\`

<div class="concept-box warn">
<strong>Kenapa JSON-nya ditempel di kode?</strong> Playground ini tidak punya akses internet, jadi data BMKG kita tempel langsung sebagai konstanta. Di project aslinya, data diambil pakai <code>reqwest</code> — lihat <code>08_mini_projects/gempa_bmkg/</code> di repo.
</div>

<div class="concept-box tip">
<strong>Coba sendiri:</strong> buka <code>https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json</code> di browser. Datanya berubah tiap kali ada gempa baru di Indonesia.
</div>
        `,
        defaultCode: `// Studi Kasus: Parsing data gempa BMKG
// Data asli dari https://data.bmkg.go.id/DataMKG/TEWS/autogempa.json
// Playground tidak punya internet, jadi JSON-nya ditempel langsung.

use serde::Deserialize;

// Perhatikan: SEMUA nilai dari BMKG bertipe String — termasuk angka!
const DATA_BMKG: &str = r#"{
  "Infogempa": {
    "gempa": {
      "Tanggal": "14 Agu 2026",
      "Jam": "08:14:48 WIB",
      "Coordinates": "5.36,125.34",
      "Lintang": "5.36 LU",
      "Bujur": "125.34 BT",
      "Magnitude": "5.3",
      "Kedalaman": "10 km",
      "Wilayah": "195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT",
      "Potensi": "Tidak berpotensi tsunami",
      "Dirasakan": "-"
    }
  }
}"#;

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "Infogempa")]
    infogempa: Wrapper,
}

#[derive(Debug, Deserialize)]
struct Wrapper {
    gempa: Gempa,
}

#[derive(Debug, Deserialize)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "Jam")]
    jam: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Kedalaman")]
    kedalaman: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,
    #[serde(rename = "Lintang")]
    lintang: String,
    #[serde(rename = "Bujur")]
    bujur: String,

    // Field ini tidak ada di semua endpoint BMKG -> harus Option
    #[serde(rename = "Potensi")]
    potensi: Option<String>,
    #[serde(rename = "Dirasakan")]
    dirasakan: Option<String>,
}

impl Gempa {
    // "5.3" -> 5.3
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

    fn skala(&self) -> &'static str {
        match self.magnitudo() {
            m if m < 4.0 => "🟢 Kecil",
            m if m < 5.0 => "🟡 Ringan",
            m if m < 6.0 => "🟠 Sedang",
            m if m < 7.0 => "🔴 Kuat",
            _            => "🟣 Besar",
        }
    }

    fn kategori_kedalaman(&self) -> &'static str {
        match self.kedalaman_km() {
            d if d < 60.0  => "Dangkal",
            d if d < 300.0 => "Menengah",
            _              => "Dalam",
        }
    }

    // BMKG mengisi "-" kalau gempa tidak dirasakan
    fn dirasakan_bersih(&self) -> Option<&str> {
        let nilai = self.dirasakan.as_deref()?.trim();
        if nilai.is_empty() || nilai == "-" {
            None
        } else {
            Some(nilai)
        }
    }
}

fn main() {
    let response: Response = serde_json::from_str(DATA_BMKG).expect("JSON tidak valid");
    let g = response.infogempa.gempa;

    println!("🌏 GEMPA TERBARU · BMKG");
    println!("────────────────────────────────────────────");
    println!("  {}  M {:.1}", g.skala(), g.magnitudo());
    println!("────────────────────────────────────────────");
    println!("  Waktu     : {} {}", g.tanggal, g.jam);
    println!("  Wilayah   : {}", g.wilayah);
    println!("  Koordinat : {} , {}", g.lintang, g.bujur);
    println!("  Kedalaman : {} ({})", g.kedalaman, g.kategori_kedalaman());

    // Option<String> -> pakai if let, jangan unwrap()
    if let Some(potensi) = &g.potensi {
        println!("  Potensi   : {}", potensi);
    }

    match g.dirasakan_bersih() {
        Some(mmi) => println!("  Dirasakan : {}", mmi),
        None      => println!("  Dirasakan : tidak ada laporan"),
    }
    println!("────────────────────────────────────────────");
}`,
        exercise: {
          title: 'Exercise: Koordinat & Potensi Tsunami',
          desc: 'Lengkapi tiga method yang belum jadi. Perhatikan jebakan di TODO 3 — ini bug yang benar-benar sering terjadi.',
          tasks: [
            'koordinat(): ubah "5.36,125.34" jadi Some((5.36, 125.34)). Pakai split_once(",") dan operator ?',
            'google_maps_url(): hasilkan "https://www.google.com/maps?q=LAT,LON", None kalau koordinat gagal di-parse',
            'tsunami(): return true HANYA kalau benar-benar berpotensi tsunami. Hati-hati — teks "Tidak berpotensi tsunami" juga mengandung kata "berpotensi"!',
            'Di main(), tampilkan koordinat, link Google Maps, dan status tsunami',
          ],
          starterCode: `use serde::Deserialize;

const DATA_BMKG: &str = r#"{
  "Infogempa": {
    "gempa": {
      "Coordinates": "5.36,125.34",
      "Magnitude": "5.3",
      "Kedalaman": "10 km",
      "Wilayah": "195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT",
      "Potensi": "Tidak berpotensi tsunami"
    }
  }
}"#;

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "Infogempa")]
    infogempa: Wrapper,
}

#[derive(Debug, Deserialize)]
struct Wrapper {
    gempa: Gempa,
}

#[derive(Debug, Deserialize)]
struct Gempa {
    #[serde(rename = "Coordinates")]
    coordinates: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,
    #[serde(rename = "Potensi")]
    potensi: Option<String>,
}

impl Gempa {
    fn magnitudo(&self) -> f64 {
        self.magnitude.trim().parse().unwrap_or(0.0)
    }

    // TODO 1: "5.36,125.34" -> Some((5.36, 125.34))
    // Hint: self.coordinates.split_once(',') mengembalikan Option<(&str, &str)>
    fn koordinat(&self) -> Option<(f64, f64)> {
        todo!()
    }

    // TODO 2: bikin URL Google Maps dari koordinat
    fn google_maps_url(&self) -> Option<String> {
        todo!()
    }

    // TODO 3: JEBAKAN! "Tidak berpotensi tsunami" mengandung kata "berpotensi".
    // Pastikan fungsi ini TIDAK salah baca.
    fn tsunami(&self) -> bool {
        todo!()
    }
}

fn main() {
    let response: Response = serde_json::from_str(DATA_BMKG).expect("JSON tidak valid");
    let g = response.infogempa.gempa;

    println!("M {:.1} — {}", g.magnitudo(), g.wilayah);

    // TODO 4: tampilkan koordinat, link maps, dan status tsunami
    // Contoh output yang diharapkan:
    // M 5.3 — 195 km BaratLaut TAHUNA-KEP.SANGIHE-SULUT
    // Koordinat : 5.36, 125.34
    // Peta      : https://www.google.com/maps?q=5.36,125.34
    // Tsunami   : aman
}`,
          hints: [
            'koordinat(): let (lat, lon) = self.coordinates.split_once(\',\')?; lalu lat.trim().parse().ok()? untuk masing-masing, terakhir Some((lat, lon))',
            'google_maps_url(): let (lat, lon) = self.koordinat()?; Some(format!("https://www.google.com/maps?q={},{}", lat, lon))',
            'tsunami(): ubah ke lowercase dulu, lalu cek p.contains("berpotensi") && !p.contains("tidak"). Field-nya Option, jadi pakai .as_deref().map(...).unwrap_or(false)',
            'Di main(): if let Some((lat, lon)) = g.koordinat() { println!("Koordinat : {}, {}", lat, lon); } dan untuk tsunami pakai if g.tsunami() { "WASPADA" } else { "aman" }',
          ]
        }
      },

      {
        id: 'bmkg_analisis',
        title: 'Analisis Daftar Gempa',
        content: `
# Analisis Daftar Gempa

Endpoint \`gempaterkini.json\` mengembalikan **array**, bukan objek tunggal.
Bedanya cuma satu baris di struct pembungkus:

\`\`\`rust
struct Wrapper {
    gempa: Vec<Gempa>,   // array — bandingkan dengan: gempa: Gempa
}
\`\`\`

## Jebakan 5 — Data Kembar

BMKG kadang mengirim gempa yang sama dua kali. Kita buang duplikatnya dengan \`HashSet\`,
tapi urutan aslinya harus tetap terjaga (terbaru di atas):

\`\`\`rust
fn dedup(daftar: Vec<Gempa>) -> Vec<Gempa> {
    let mut terlihat = HashSet::new();
    daftar
        .into_iter()
        .filter(|g| terlihat.insert(g.kunci()))
        .collect()
}
\`\`\`

Kuncinya ada di \`HashSet::insert()\` — method itu mengembalikan \`false\` kalau nilainya sudah ada.
Jadi \`filter\` otomatis membuang yang kedua tanpa perlu sorting.

## Statistik dengan Iterator

\`\`\`rust
let jumlah: f64 = daftar.iter().map(|g| g.magnitudo()).sum();
let rata_rata = jumlah / daftar.len() as f64;
\`\`\`

## Jebakan 6 — f64 Tidak Punya Ord

\`daftar.iter().max_by_key(|g| g.magnitudo())\` **tidak akan compile**. Kenapa?
Karena \`f64\` bisa bernilai \`NaN\`, dan \`NaN\` tidak bisa dibandingkan — jadi \`f64\` hanya punya
\`PartialOrd\`, bukan \`Ord\`. Solusinya \`max_by\` + \`partial_cmp\`:

\`\`\`rust
let terkuat = daftar
    .iter()
    .max_by(|a, b| a.magnitudo().partial_cmp(&b.magnitudo()).unwrap())
    .unwrap();
\`\`\`

## Kelompokkan dengan HashMap entry API

\`\`\`rust
let mut per_kategori: HashMap<&str, usize> = HashMap::new();
for g in &daftar {
    *per_kategori.entry(g.kategori_kedalaman()).or_insert(0) += 1;
}
\`\`\`

\`entry().or_insert(0)\` artinya: "ambil nilainya, kalau belum ada isi 0 dulu".
Tanda \`*\` di depan untuk dereference — karena yang dikembalikan adalah \`&mut usize\`.

<div class="concept-box tip">
<strong>Kenapa kedalaman penting?</strong> Gempa dangkal (&lt; 60 km) terasa jauh lebih kuat di permukaan dibanding gempa dalam dengan magnitudo sama. Makanya M 5.0 dangkal bisa lebih merusak daripada M 6.0 di kedalaman 300 km.
</div>

<div class="concept-box">
<strong>Versi lengkapnya</strong> ada di <code>08_mini_projects/gempa_bmkg/</code> — sudah pakai <code>reqwest</code> untuk ambil data betulan, punya mode <code>--watch</code>, dan 12 unit test.
</div>
        `,
        defaultCode: `// Studi Kasus 2: Analisis daftar gempa BMKG
// Data asli dari https://data.bmkg.go.id/DataMKG/TEWS/gempaterkini.json
// Bedanya dengan autogempa.json: "gempa" di sini berupa ARRAY.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

const DATA_BMKG: &str = r#"{
  "Infogempa": { "gempa": [
    { "Tanggal":"14 Agu 2026","DateTime":"2026-08-14T01:14:48+00:00",
      "Coordinates":"5.36,125.34","Magnitude":"5.7","Kedalaman":"48 km",
      "Wilayah":"175 km TimurLaut TAHUNA-KEP.SANGIHE-SULUT" },
    { "Tanggal":"14 Agu 2026","DateTime":"2026-08-14T01:14:48+00:00",
      "Coordinates":"5.36,125.34","Magnitude":"5.7","Kedalaman":"48 km",
      "Wilayah":"175 km TimurLaut TAHUNA-KEP.SANGIHE-SULUT" },
    { "Tanggal":"09 Agu 2026","DateTime":"2026-08-09T04:02:10+00:00",
      "Coordinates":"-5.60,102.80","Magnitude":"5.5","Kedalaman":"40 km",
      "Wilayah":"84 km Tenggara ENGGANO-BENGKULU" },
    { "Tanggal":"05 Agu 2026","DateTime":"2026-08-05T15:41:03+00:00",
      "Coordinates":"4.80,126.50","Magnitude":"6.4","Kedalaman":"10 km",
      "Wilayah":"221 km BaratLaut PULAUKARATUNG-SULUT" },
    { "Tanggal":"05 Agu 2026","DateTime":"2026-08-05T20:12:55+00:00",
      "Coordinates":"1.20,127.40","Magnitude":"5.9","Kedalaman":"110 km",
      "Wilayah":"62 km BaratDaya PULAUDOI-MALUT" },
    { "Tanggal":"01 Agu 2026","DateTime":"2026-08-01T12:30:41+00:00",
      "Coordinates":"-3.10,130.20","Magnitude":"4.8","Kedalaman":"320 km",
      "Wilayah":"44 km BaratLaut SERAMBAGIANTIMUR-MALUKU" }
  ]}
}"#;

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "Infogempa")]
    infogempa: Wrapper,
}

#[derive(Debug, Deserialize)]
struct Wrapper {
    gempa: Vec<Gempa>, // <- ARRAY, ini bedanya
}

#[derive(Debug, Clone, Deserialize)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "DateTime")]
    date_time: String,
    #[serde(rename = "Coordinates")]
    coordinates: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Kedalaman")]
    kedalaman: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,
}

impl Gempa {
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

    fn kategori_kedalaman(&self) -> &'static str {
        match self.kedalaman_km() {
            d if d < 60.0  => "Dangkal",
            d if d < 300.0 => "Menengah",
            _              => "Dalam",
        }
    }

    fn ikon(&self) -> &'static str {
        match self.magnitudo() {
            m if m < 5.0 => "🟡",
            m if m < 6.0 => "🟠",
            m if m < 7.0 => "🔴",
            _            => "🟣",
        }
    }

    // Kunci unik untuk deteksi data kembar
    fn kunci(&self) -> String {
        format!("{}|{}", self.date_time, self.coordinates)
    }
}

// HashSet::insert() mengembalikan false kalau nilainya sudah ada —
// jadi filter ini membuang duplikat TANPA mengubah urutan.
fn dedup(daftar: Vec<Gempa>) -> Vec<Gempa> {
    let mut terlihat = HashSet::new();
    daftar
        .into_iter()
        .filter(|g| terlihat.insert(g.kunci()))
        .collect()
}

fn main() {
    let response: Response = serde_json::from_str(DATA_BMKG).expect("JSON tidak valid");

    let mentah = response.infogempa.gempa;
    println!("Data mentah dari BMKG : {} gempa", mentah.len());

    let daftar = dedup(mentah);
    println!("Setelah buang kembar  : {} gempa\\n", daftar.len());

    // ---- Tabel ----
    println!("────────────────────────────────────────────────────────");
    println!("  {:<14} {:<7} {:<10} {}", "TANGGAL", "MAG", "KEDALAMAN", "WILAYAH");
    println!("────────────────────────────────────────────────────────");
    for g in &daftar {
        println!(
            "  {:<14} {} {:<4.1} {:<10} {}",
            g.tanggal,
            g.ikon(),
            g.magnitudo(),
            g.kedalaman,
            g.wilayah
        );
    }
    println!("────────────────────────────────────────────────────────\\n");

    // ---- Statistik pakai iterator ----
    let total = daftar.len() as f64;
    let jumlah_mag: f64 = daftar.iter().map(|g| g.magnitudo()).sum();
    println!("📊 STATISTIK");
    println!("  Rata-rata magnitudo : M {:.2}", jumlah_mag / total);

    // f64 tidak punya Ord (gara-gara NaN), jadi pakai partial_cmp
    let terkuat = daftar
        .iter()
        .max_by(|a, b| a.magnitudo().partial_cmp(&b.magnitudo()).unwrap())
        .unwrap();
    println!("  Terkuat             : M {:.1} — {}", terkuat.magnitudo(), terkuat.wilayah);

    // ---- Kelompokkan pakai HashMap entry API ----
    let mut per_kedalaman: HashMap<&str, usize> = HashMap::new();
    for g in &daftar {
        *per_kedalaman.entry(g.kategori_kedalaman()).or_insert(0) += 1;
    }

    println!("\\n  Sebaran kedalaman:");
    for kategori in ["Dangkal", "Menengah", "Dalam"] {
        if let Some(n) = per_kedalaman.get(kategori) {
            println!("    {:<10} {} ({})", kategori, "▇".repeat(*n), n);
        }
    }

    // ---- Filter: hanya gempa signifikan ----
    let signifikan: Vec<&Gempa> = daftar.iter().filter(|g| g.magnitudo() >= 5.5).collect();
    println!("\\n  Gempa M >= 5.5      : {} dari {}", signifikan.len(), daftar.len());
}`,
        exercise: {
          title: 'Exercise: Cari Gempa Paling Berbahaya',
          desc: 'Gempa dangkal terasa jauh lebih kuat daripada gempa dalam dengan magnitudo sama. Cari mana yang paling berisiko.',
          tasks: [
            'urutkan_by_magnitudo(): urutkan daftar dari magnitudo terbesar ke terkecil. Ingat f64 tidak punya Ord — pakai sort_by + partial_cmp',
            'berbahaya(): return true kalau magnitudo >= 5.5 DAN kedalaman < 60 km (dangkal)',
            'hitung_per_tanggal(): kelompokkan jumlah gempa per tanggal pakai HashMap<String, usize>',
            'Di main(), tampilkan daftar terurut, daftar gempa berbahaya, dan rekap per tanggal',
          ],
          starterCode: `use serde::Deserialize;
use std::collections::HashMap;

const DATA_BMKG: &str = r#"{
  "Infogempa": { "gempa": [
    { "Tanggal":"14 Agu 2026","Magnitude":"5.7","Kedalaman":"48 km",
      "Wilayah":"175 km TimurLaut TAHUNA-KEP.SANGIHE-SULUT" },
    { "Tanggal":"14 Agu 2026","Magnitude":"5.5","Kedalaman":"40 km",
      "Wilayah":"84 km Tenggara ENGGANO-BENGKULU" },
    { "Tanggal":"05 Agu 2026","Magnitude":"6.4","Kedalaman":"10 km",
      "Wilayah":"221 km BaratLaut PULAUKARATUNG-SULUT" },
    { "Tanggal":"05 Agu 2026","Magnitude":"5.9","Kedalaman":"110 km",
      "Wilayah":"62 km BaratDaya PULAUDOI-MALUT" },
    { "Tanggal":"01 Agu 2026","Magnitude":"4.8","Kedalaman":"320 km",
      "Wilayah":"44 km BaratLaut SERAMBAGIANTIMUR-MALUKU" }
  ]}
}"#;

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "Infogempa")]
    infogempa: Wrapper,
}

#[derive(Debug, Deserialize)]
struct Wrapper {
    gempa: Vec<Gempa>,
}

#[derive(Debug, Clone, Deserialize)]
struct Gempa {
    #[serde(rename = "Tanggal")]
    tanggal: String,
    #[serde(rename = "Magnitude")]
    magnitude: String,
    #[serde(rename = "Kedalaman")]
    kedalaman: String,
    #[serde(rename = "Wilayah")]
    wilayah: String,
}

impl Gempa {
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

    // TODO 2: berbahaya = M >= 5.5 DAN kedalaman < 60 km
    fn berbahaya(&self) -> bool {
        todo!()
    }
}

// TODO 1: urutkan dari magnitudo terbesar ke terkecil
fn urutkan_by_magnitudo(daftar: &mut Vec<Gempa>) {
    todo!()
}

// TODO 3: hitung berapa gempa per tanggal
fn hitung_per_tanggal(daftar: &[Gempa]) -> HashMap<String, usize> {
    todo!()
}

fn main() {
    let response: Response = serde_json::from_str(DATA_BMKG).expect("JSON tidak valid");
    let mut daftar = response.infogempa.gempa;

    urutkan_by_magnitudo(&mut daftar);

    println!("=== Urut dari Terkuat ===");
    // TODO 4: print tiap gempa: M x.x - wilayah

    println!("\\n=== Gempa Berbahaya (dangkal & kuat) ===");
    // TODO 4: print hanya yang berbahaya() == true

    println!("\\n=== Rekap per Tanggal ===");
    // TODO 4: print hasil hitung_per_tanggal
}`,
          hints: [
            'urutkan_by_magnitudo: daftar.sort_by(|a, b| b.magnitudo().partial_cmp(&a.magnitudo()).unwrap()) — perhatikan b duluan supaya urutannya menurun',
            'berbahaya: self.magnitudo() >= 5.5 && self.kedalaman_km() < 60.0',
            'hitung_per_tanggal: bikin HashMap kosong, lalu for g in daftar { *map.entry(g.tanggal.clone()).or_insert(0) += 1; } dan return map-nya',
            'Untuk filter berbahaya: for g in daftar.iter().filter(|g| g.berbahaya()) { ... }',
          ]
        }
      }
    ]
  }
];
