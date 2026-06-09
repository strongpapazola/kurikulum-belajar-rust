# 06 · Traits & Generics

> Traits = interface versi Rust, tapi lebih powerful.
> Generics = kode yang bisa bekerja dengan berbagai tipe.

---

## Coming Soon 🚧

Modul ini sedang dalam pengembangan. Yang akan dibahas:

| Topik | Konsep |
|-------|--------|
| **Traits** | Definisi trait, `impl Trait`, default method |
| **Generics** | Generic function, generic struct, `where` clause |
| **Trait Bounds** | `T: Display`, `T: Clone + Debug` |
| **Lifetimes** | `'a`, lifetime annotation, lifetime elision |
| **Common Traits** | `Display`, `Debug`, `Clone`, `Iterator`, `From/Into` |

---

## Preview

```rust
// Trait — mirip interface di PHP/Java
trait HitungPajak {
    fn tarif(&self) -> f64;
    fn hitung(&self, dpp: f64) -> f64 {   // default implementation
        dpp * self.tarif()
    }
}

struct PPN;
struct PPh21 { penghasilan: f64 }

impl HitungPajak for PPN {
    fn tarif(&self) -> f64 { 0.11 }
}

impl HitungPajak for PPh21 {
    fn tarif(&self) -> f64 { 0.05 }
}

// Generic function — bekerja untuk semua yang impl HitungPajak
fn cetak_pajak<T: HitungPajak>(pajak: &T, dpp: f64) {
    println!("Pajak: {}", pajak.hitung(dpp));
}
```

---

Sambil menunggu, lanjut ke **[07_async_rust](../07_async_rust)** 🦀
