# 02 · Ownership

> Konsep paling unik di Rust — tidak ada di bahasa lain.
> Pahami ini, dan kamu sudah menguasai 50% dari Rust.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_ownership.rs](./01_ownership.rs) | Ownership | move, clone, drop, scope |
| [02_borrowing.rs](./02_borrowing.rs) | Borrowing | `&reference`, `&mut reference`, slice |

---

## Kenapa Ownership Ada?

Rust tidak punya **garbage collector** (GC) seperti Java/Go, tapi juga tidak pakai **manual memory management** seperti C/C++.

Solusinya: **Ownership** — compiler yang manage memory secara otomatis saat compile time.

```
PHP/Python/Java/Go → Garbage Collector (GC)
                     → Performa tidak predictable, ada "GC pause"

C/C++             → Manual (malloc/free)
                     → Berbahaya, bisa memory leak / dangling pointer

Rust              → Ownership System
                     → Otomatis + aman + tanpa GC pause ✅
```

---

## 3 Aturan Ownership

```
1. Setiap nilai punya satu owner
2. Hanya boleh ada SATU owner di satu waktu
3. Saat owner keluar scope → nilai otomatis di-drop (memory bebas)
```

---

## Konsep Utama

### Move — Ownership Berpindah
```rust
let s1 = String::from("halo");
let s2 = s1;          // ownership PINDAH ke s2

println!("{s1}");     // ❌ ERROR! s1 sudah tidak valid
println!("{s2}");     // ✅ OK
```

### Clone — Copy Data di Heap
```rust
let s1 = String::from("halo");
let s2 = s1.clone();  // deep copy — keduanya valid

println!("{s1}");     // ✅ OK
println!("{s2}");     // ✅ OK
```

### Borrowing — Pinjam Tanpa Ambil Ownership
```rust
let s = String::from("halo");

fn panjang(s: &String) -> usize {  // & = borrow
    s.len()
}

let p = panjang(&s);  // s dipinjam, bukan di-move
println!("{s}");      // ✅ s masih valid!
```

### Aturan Borrowing
```rust
// ✅ Boleh: banyak immutable reference sekaligus
let r1 = &s;
let r2 = &s;

// ✅ Boleh: satu mutable reference
let r3 = &mut s;

// ❌ TIDAK BOLEH: mutable + immutable reference bersamaan
let r1 = &s;
let r2 = &mut s;  // ERROR!
```

---

## Analogi Sederhana

> **Ownership** seperti buku perpustakaan:
> - Hanya **satu orang** yang bisa pinjam buku di satu waktu (ownership)
> - Kamu bisa **kasih lihat** buku ke teman tanpa lepas kepemilikan (immutable borrow)
> - Kamu bisa **kasih teman untuk edit** — tapi kamu tidak boleh lihat dulu (mutable borrow)
> - Saat kamu **kembalikan buku** (keluar scope) → langsung tersedia lagi (drop)

---

## Exercise

| File | Soal | Topik |
|------|------|-------|
| [ex01_ownership.rs](./ex01_ownership.rs) | 6 soal | move, clone, &ref, &mut ref, slice |

```bash
rustc ex01_ownership.rs && ./ex01_ownership
```

---

## Setelah Selesai Modul Ini

Lanjut ke **[03_enums](../03_enums)** — Enum & pattern matching! 🦀
