// ============================================
// VECTORS — Dynamic array di Rust
// ============================================
// Jalankan: rustc 01_vectors.rs && ./01_vectors
// Vec<T> = array yang bisa grow/shrink, mirip Array PHP / list Python

fn main() {
    // ==================
    // BUAT VECTOR
    // ==================
    let v1: Vec<i32> = Vec::new();            // kosong
    let v2 = vec![1, 2, 3, 4, 5];            // dengan macro vec!
    let v3: Vec<i32> = (1..=10).collect();    // dari range
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);

    // ==================
    // PUSH & POP
    // ==================
    let mut angka = Vec::new();
    angka.push(10);
    angka.push(20);
    angka.push(30);
    println!("Setelah push: {:?}", angka);

    let terakhir = angka.pop(); // return Option<T>
    println!("Pop: {:?}", terakhir); // Some(30)
    println!("Setelah pop: {:?}", angka);

    // ==================
    // AKSES ELEMENT
    // ==================
    let buah = vec!["apel", "mangga", "jeruk", "durian"];

    // Index langsung — PANIC kalau out of bounds
    println!("Index 0: {}", buah[0]);

    // get() — return Option, lebih aman
    match buah.get(2) {
        Some(b) => println!("get(2): {b}"),
        None    => println!("Tidak ada"),
    }

    // get yang out of bounds
    if let Some(b) = buah.get(99) {
        println!("{b}");
    } else {
        println!("Index 99 tidak ada"); // ✅ tidak panic
    }

    // ==================
    // ITERASI
    // ==================
    let nilais = vec![80, 90, 75, 95, 85];

    // for loop biasa
    for n in &nilais {
        print!("{n} ");
    }
    println!();

    // enumerate
    for (i, n) in nilais.iter().enumerate() {
        println!("Siswa {}: {}", i+1, n);
    }

    // map + collect
    let nilai_scaled: Vec<i32> = nilais.iter()
        .map(|&n| (n as f64 * 1.1) as i32) // scale up 10%
        .collect();
    println!("Scaled: {:?}", nilai_scaled);

    // filter
    let lulus: Vec<&i32> = nilais.iter()
        .filter(|&&n| n >= 80)
        .collect();
    println!("Lulus: {:?}", lulus);

    // fold (reduce)
    let total: i32 = nilais.iter().sum();
    let rata: f64 = total as f64 / nilais.len() as f64;
    println!("Total: {total}, Rata-rata: {rata:.1}");

    // ==================
    // SORT
    // ==================
    let mut data = vec![5, 2, 8, 1, 9, 3];
    data.sort();
    println!("Sort asc: {:?}", data);

    data.sort_by(|a, b| b.cmp(a));
    println!("Sort desc: {:?}", data);

    // ==================
    // CONTOH: Proses data faktur
    // ==================
    let totals = vec![
        1_500_000.0_f64,
        2_300_000.0,
        850_000.0,
        5_000_000.0,
        300_000.0,
    ];

    let total_semua: f64 = totals.iter().sum();
    let ppn_semua: f64 = totals.iter().map(|t| t * 0.11).sum();
    let faktur_besar: Vec<&f64> = totals.iter().filter(|&&t| t > 1_000_000.0).collect();

    println!("\n=== Summary Faktur ===");
    println!("Total transaksi : Rp {:.0}", total_semua);
    println!("Total PPN       : Rp {:.0}", ppn_semua);
    println!("Faktur > 1jt    : {} faktur", faktur_besar.len());
}

// ============================================
// Perbandingan:
//
// PHP:    $arr = []; array_push($arr, 1);
//         foreach ($arr as $val) { ... }
//         array_map(fn, $arr), array_filter(fn, $arr)
//
// Python: arr = []; arr.append(1)
//         for val in arr: ...
//         list(map(fn, arr)), list(filter(fn, arr))
//
// Rust:   let mut v = Vec::new(); v.push(1);
//         for val in &v { ... }
//         v.iter().map(fn).collect()
// ============================================
