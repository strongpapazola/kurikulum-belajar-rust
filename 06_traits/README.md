# 06 · Traits & Generics

> Traits = interface versi Rust, tapi lebih powerful.
> Generics = kode yang bisa bekerja dengan berbagai tipe tanpa overhead performa.

---

## Yang Akan Dipelajari

| File | Topik | Konsep Utama |
|------|-------|--------------|
| [01_traits.rs](./01_traits.rs) | Traits | definisi trait, `impl Trait`, default method, trait object |
| [02_generics.rs](./02_generics.rs) | Generics | generic function, generic struct, trait bounds, `where` |
| [03_lifetimes.rs](./03_lifetimes.rs) | Lifetimes | `'a`, struct lifetime, lifetime elision, `'static` |

---

## Traits — Kontrak Behavior

```rust
// Definisi
trait HitungPajak {
    fn tarif(&self) -> f64;           // wajib diimplementasikan
    fn nama(&self) -> &str;

    fn hitung(&self, dpp: f64) -> f64 {  // default method — boleh di-override
        dpp * self.tarif()
    }
}

// Implementasi
struct PPN;
impl HitungPajak for PPN {
    fn tarif(&self) -> f64 { 0.11 }
    fn nama(&self)  -> &str { "PPN" }
}

// Trait sebagai parameter
fn cetak(pajak: &impl HitungPajak, dpp: f64) {
    println!("{}: {}", pajak.nama(), pajak.hitung(dpp));
}

// Trait object — kumpulan tipe berbeda dalam satu Vec
let daftar: Vec<Box<dyn HitungPajak>> = vec![
    Box::new(PPN),
    Box::new(PPh23 { tarif: 0.02 }),
];
```

---

## Generics — Kode untuk Semua Tipe

```rust
// Generic function
fn terbesar<T: PartialOrd>(list: &[T]) -> &T { ... }

// Generic struct
struct Repo<T> {
    data: Vec<T>,
}

impl<T: Clone + Debug> Repo<T> {
    fn simpan(&mut self, item: T) { self.data.push(item); }
}

// Bisa dipakai untuk Faktur, WajibPajak, apapun!
let mut faktur_repo: Repo<Faktur>     = Repo::new();
let mut wp_repo:     Repo<WajibPajak> = Repo::new();
```

---

## Lifetimes — Validitas Reference

```rust
// Compiler butuh tahu reference mana yang dikembalikan
fn paling_panjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct yang simpan reference butuh lifetime
struct Laporan<'a> {
    judul: &'a str,   // reference ke string di luar struct
}
```

> Kebanyakan kasus, lifetime di-**infer otomatis** (lifetime elision). Annotation eksplisit hanya diperlukan kalau compiler tidak bisa tahu sendiri.

---

## Perbandingan

```
PHP:    interface HitungPajak { public function tarif(): float; }
Java:   interface HitungPajak { double tarif(); }
Rust:   trait HitungPajak { fn tarif(&self) -> f64; }

Keunggulan Rust:
✅ Bisa implement trait untuk type dari library lain
✅ Default method implementation
✅ Zero-cost abstraction (monomorphization untuk generics)
✅ Lifetime mencegah dangling reference saat compile time
```

---

## Exercise

| File | Soal | Topik |
|------|------|-------|
| [ex01_traits.rs](./ex01_traits.rs) | 8 soal | trait, impl, generic struct, trait object |

```bash
rustc ex01_traits.rs && ./ex01_traits
```

---

## Setelah Selesai Modul Ini

Lanjut ke **[07_async_rust](../07_async_rust)** — Async/Await & Tokio! 🦀
