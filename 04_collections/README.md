# 04 · Collections

> Vec, String, dan HashMap — tiga koleksi yang paling sering dipakai di Rust.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_vectors.rs](./01_vectors.rs) | Vec\<T\> | push, pop, iter, map, filter, sort |
| [03_hashmaps.rs](./03_hashmaps.rs) | HashMap\<K,V\> | insert, get, entry API, group by |

---

## Vec\<T\> — Dynamic Array

```rust
// PHP:    $arr = [1, 2, 3];   array_push($arr, 4);
// Python: arr = [1, 2, 3];    arr.append(4)
// Rust:
let mut v = vec![1, 2, 3];
v.push(4);

// Akses aman pakai get() → return Option
match v.get(10) {
    Some(val) => println!("{val}"),
    None      => println!("index tidak ada"),  // tidak panic!
}
```

### Iterator — Cara Idiomatis di Rust
```rust
let angka = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Filter + map + collect (mirip array_filter + array_map PHP)
let hasil: Vec<i32> = angka.iter()
    .filter(|&&x| x % 2 == 0)   // ambil yang genap
    .map(|&x| x * 10)            // kali 10
    .collect();                   // jadikan Vec

// [20, 40, 60, 80, 100]
```

---

## HashMap\<K, V\> — Key-Value Store

```rust
// PHP:    $map = ["key" => "val"];
// Python: d = {"key": "val"}
// Rust:
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("PPN", 0.11);
map.insert("PPh21", 0.05);

// Akses → return Option<&V>
match map.get("PPN") {
    Some(tarif) => println!("PPN: {tarif}"),
    None        => println!("Tidak ada"),
}
```

### Entry API — Insert Kalau Belum Ada
```rust
// Pola yang sangat umum: counter / group by
let mut counter: HashMap<&str, u32> = HashMap::new();

for kata in teks.split_whitespace() {
    let count = counter.entry(kata).or_insert(0);
    *count += 1;
}
```

---

## String vs &str

Ini yang sering bikin bingung di awal:

```rust
let s1: &str    = "halo";          // string literal, fixed, di stack
let s2: String  = String::from("halo");  // owned, bisa diubah, di heap
let s3: String  = "halo".to_string();   // sama dengan s2

// Function parameter: pakai &str (lebih fleksibel)
fn tampilkan(s: &str) { println!("{s}"); }

tampilkan("literal");    // ✅
tampilkan(&s2);          // ✅ String → &str otomatis
```

---

## Exercise

| File | Soal | Topik |
|------|------|-------|
| [ex01_collections.rs](./ex01_collections.rs) | 7 soal | Vec, HashMap, iterator, group by |

```bash
rustc ex01_collections.rs && ./ex01_collections
```

---

## Setelah Selesai Modul Ini

Lanjut ke **[05_error_handling](../05_error_handling)** — cara handle error yang benar di Rust! 🦀
