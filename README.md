#  Rust Programlama Dili — BLM0238 Dönem Ödevi

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.78+-B7410E?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/Lisans-MIT-1C2D6B?style=for-the-badge)
![BTU](https://img.shields.io/badge/BTÜ-BLM0238-2B3A8C?style=for-the-badge)

**Bursa Teknik Üniversitesi · Bilgisayar Mühendisliği**  
** Programlama Dilleri · 2025–2026 Bahar**

*Büşra Ünal — 22360859084*

</div>

---

##  İçindekiler

| # | Konu | Klasör |
|---|------|--------|
| 1 | [Giriş](#1-giriş) | — |
| 2 | [Nesneye Yönelik Programlama](#2-nesneye-yönelik-programlama) | [`ornekler/01_oop`](ornekler/01_oop/src/main.rs) |
| 3 | [Paralellik](#3-paralellik-fearless-concurrency) | [`ornekler/02_paralellik`](ornekler/02_paralellik/src/main.rs) |
| 4 | [Fonksiyonel Programlama](#4-fonksiyonel-programlama) | [`ornekler/03_fonksiyonel`](ornekler/03_fonksiyonel/src/main.rs) |
| 5 | [Binding ve Bellek Yönetimi](#5-binding-ve-bellek-yönetimi) | [`ornekler/04_bellek`](ornekler/04_bellek/src/main.rs) |
| 6 | [Diğer Dillerden Ayırt Edici Özellik](#6-diğer-dillerden-ayırt-edici-özellik) | [`ornekler/05_ayirt_edici`](ornekler/05_ayirt_edici/src/main.rs) |
| 7 | [Sözdizim ve Anlam](#7-sözdizim-syntax-ve-anlam-semantics) | [`ornekler/06_sozdizim`](ornekler/06_sozdizim/src/main.rs) |
| 8 | [ADT Yapısı ve Generiklik](#8-adt-yapısı-tipler-ve-generiklik) | [`ornekler/07_adt_generik`](ornekler/07_adt_generik/src/main.rs) |

---

## 1. Giriş

Rust, Mozilla Araştırma tarafından 2010 yılında tasarlanmaya başlanan ve 2015 yılında 1.0 kararlı sürümüyle yayımlanan sistem programlama dilidir. Dilin temel tasarım hedefi, C ve C++ ile eşit düşük seviyeli performansı sağlamak; ancak bunu **bellek güvenliği**, **tip güvenliği** ve **veri yarışması güvenliği** garantileriyle birlikte sunmaktır.

Geleneksel yaklaşımlar ya Garbage Collector (GC) kullanır ya da programcının elle bellek yönetmesini gerektirir. Rust bu ikilemden tamamen farklı bir yol izler: derleme zamanında **sahiplik (ownership)** ve **ödünç alma (borrowing)** kurallarını uygulayarak tüm bellek hatalarını önceden tespit eder. Bu sayede GC yükü olmaksızın, **sıfır maliyetli soyutlama (zero-cost abstraction)** prensibi korunarak güvenli kod yazılabilmektedir.

> 🏆 2022–2024 yılları arasında Stack Overflow Geliştirici Anketi'nde üst üste **"En Çok Sevilen Dil"** seçilmiştir.

---

## 2. Nesneye Yönelik Programlama

Rust geleneksel anlamda bir OOP dili değildir; `class` anahtar kelimesi yoktur ve kalıtım (inheritance) zinciri desteklenmez. Bununla birlikte nesneye yönelik programlamanın temel ilkelerini **`struct` + `impl` + `trait`** üçlüsüyle sağlar.

| Kavram | Java / C++ | Rust |
|--------|-----------|------|
| Veri yapısı | `class` | `struct` |
| Metot tanımlama | sınıf içinde | `impl YapıAdı` bloğu |
| Arayüz / sözleşme | `interface` / saf sanal | `trait` |
| Kalıtım | `extends` | ❌ yok → kompozisyon |
| Polimorfizm | sanal fonksiyon | `dyn Trait` (vtable) |

```rust
trait Ses {
    fn cikar(&self) -> &str;
    fn tanitim(&self) {           // varsayılan implementasyon
        println!("Ben: {}", self.cikar());
    }
}

struct Kedi { ad: String }
struct Köpek { ad: String }

impl Ses for Kedi  { fn cikar(&self) -> &str { "Miyav" } }
impl Ses for Köpek { fn cikar(&self) -> &str { "Hav"   } }

fn hayvan_sesi(h: &dyn Ses) { h.tanitim(); }
// dyn Ses → vtable aracılığıyla çalışma-zamanı dispatch
```

>  Tam örnek: [`ornekler/01_oop/src/main.rs`](ornekler/01_oop/src/main.rs)

---

## 3. Paralellik (Fearless Concurrency)

Rust'ta eşzamanlılık **"Fearless Concurrency"** (korkusuz eşzamanlılık) olarak adlandırılır; çünkü ownership ve borrowing kuralları **veri yarışmalarını (data race) derleme zamanında tamamen olanaksız** kılar.

- **`Arc<T>`** — Atomik Referans Sayımı; thread'ler arası paylaşımlı sahiplik
- **`Mutex<T>` / `RwLock<T>`** — Thread-safe mutasyon senkronizasyonu  
- **`mpsc::channel`** — Mesaj tabanlı iletişim (çok üretici – tek tüketici)
- **`async/await`** — Non-blocking asenkron programlama (Tokio, async-std)
- **`Send` / `Sync`** — Hangi tiplerin thread'ler arası güvenli olduğunu derleyiciye bildirir

```rust
use std::{thread, sync::{Arc, Mutex}};

let sayac = Arc::new(Mutex::new(0_i32));

let handles: Vec<_> = (0..5).map(|_| {
    let s = Arc::clone(&sayac);
    thread::spawn(move || {
        *s.lock().unwrap() += 1;
    })
}).collect();

handles.into_iter().for_each(|h| h.join().unwrap());
println!("Sonuç: {}", *sayac.lock().unwrap()); // 5

// Kanal ile mesaj geçişi
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
thread::spawn(move || { tx.send(42).unwrap(); });
println!("{}", rx.recv().unwrap()); // 42
```

>  Tam örnek: [`ornekler/02_paralellik/src/main.rs`](ornekler/02_paralellik/src/main.rs)

---

## 4. Fonksiyonel Programlama

Rust, fonksiyonel programlamanın temel kavramlarını birinci sınıf vatandaş olarak destekler.

| Özellik | Açıklama |
|---------|----------|
| **Closure** | Çevreyi yakalayan anonim fonksiyon (`Fn`, `FnMut`, `FnOnce`) |
| **Iterator zinciri** | `map`, `filter`, `fold`, `flat_map` vb. — lazy değerlendirme |
| **Değişmezlik** | `let` varsayılan olarak immutable; `mut` ile açıkça değiştirilebilir |
| **Option / Result** | Null-free hata yönetimi; monadik `and_then`, `map_err` zincirleri |

```rust
let kelimeler = vec!["rust", "go", "python", "haskell"];

// Lazy iterator zinciri
let sonuc: Vec<String> = kelimeler.iter()
    .filter(|&&k| k.starts_with('r'))
    .map(|&k| k.to_uppercase())
    .collect();          // ["RUST"]

// Closure çevreden değişken yakalar
let esik = 4;
let uzunlar: Vec<_> = kelimeler.iter()
    .filter(|&&k| k.len() > esik).collect();

// Result zinciri: monadik and_then
fn parse(s: &str) -> Result<i32, _> { s.parse() }
let r = parse("42").map(|n| n * 2).unwrap_or(0); // 84
```

>  Tam örnek: [`ornekler/03_fonksiyonel/src/main.rs`](ornekler/03_fonksiyonel/src/main.rs)

---

## 5. Binding ve Bellek Yönetimi

Rust'ın bellek yönetimi **ownership (sahiplik)** sistemi üzerine kuruludur; herhangi bir garbage collector gerektirmez.

### Üç Temel Kural

```
1. Her değerin tam olarak bir sahibi (owner) vardır.
2. Sahip kapsam dışına çıktığında drop() çağrılır → bellek anında serbest bırakılır (RAII).
3. Aynı anda ya birden fazla &T (immutable borrow)
         YA DA tek bir &mut T (mutable borrow) aktif olabilir.
```

```rust
// 1) Move semantiği: sahiplik s2'ye geçer
let s1 = String::from("merhaba");
let s2 = s1;
// println!("{}", s1); //  DERLEME HATASI — s1 artık geçersiz

// 2) Immutable borrow: birden fazla olabilir
let s3 = String::from("dünya");
let r1 = &s3;
let r2 = &s3;
println!("{} {}", r1, r2); //  OK
// let r3 = &mut s3; //  r1/r2 aktifken mutable borrow HATASI

// 3) Lifetime anotasyonu
fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
// 'a: x ve y en az 'a kadar yaşamalı
```

> Tam örnek: [`ornekler/04_bellek/src/main.rs`](ornekler/04_bellek/src/main.rs)

---

## 6. Diğer Dillerden Ayırt Edici Özellik

Rust'ı rakiplerinden benzersiz kılan özellik **GC olmaksızın bellek güvenliğini derleme zamanında garanti etmesidir.**

```
┌──────────────┬──────────────────┬────────────────────┐
│ Dil          │ Bellek Yönetimi  │ Güvenlik Garantisi │
├──────────────┼──────────────────┼────────────────────┤
│ C / C++      │ Elle (manual)    │  Yok              │
│ Java / Go    │ GC (otomatik)    │ Var (runtime)    │
│ Rust         │ Ownership        │ Var (derleme)    │
└──────────────┴──────────────────┴────────────────────┘
```

```rust
// Zero-cost abstraction: C döngüsüyle özdeş ASM üretir
fn toplam(v: &[i32]) -> i32 { v.iter().sum() }

// unsafe: ham pointer erişimi — bilinçli alınan risk
let mut deger = 42_i32;
let ptr: *mut i32 = &mut deger;
unsafe {
    *ptr += 1;
    println!("{}", *ptr); // 43
}

// C FFI: dış kütüphaneleri çağırma
extern "C" { fn abs(x: i32) -> i32; }
let s = unsafe { abs(-7) }; // 7
```

> `unsafe` ≠ güvensiz; doğruluğun sorumluluğu derleyiciden programcıya geçer.  
>  Tam örnek: [`ornekler/05_ayirt_edici/src/main.rs`](ornekler/05_ayirt_edici/src/main.rs)

---

## 7. Sözdizim (Syntax) ve Anlam (Semantics)

Rust **expression-based** (ifade tabanlı) bir dildir: `if`, `match`, bloklar ve döngüler birer ifadedir; son satırda noktalı virgül olmadan değer döndürürler.

| Özellik | Açıklama |
|---------|----------|
| **Tip çıkarımı** | `let x = vec![1,2,3]` → derleyici tipi otomatik çıkarır |
| **Shadowing** | `let` ile aynı ismin farklı tipte yeniden bağlanması |
| **Pattern matching** | `match`, `if let`, `while let`, destructuring |
| **Exhaustive match** | Tüm dallar kapsanmak zorundadır; eksik dal derleme hatası |
| **Makro sistemi** | `macro_rules!` ve `proc-macro` ile derleme zamanı kod üretimi |

```rust
// Shadowing: aynı isim, farklı tip
let yas = "25";
let yas: u32 = yas.parse().unwrap();

// Expression-based if
let mesaj = if yas >= 18 { "yetişkin" } else { "çocuk" };

// Kapsamlı pattern matching + guard + destructuring
let koordinat = (3, -5);
match koordinat {
    (0, 0)          => println!("Orijin"),
    (x, 0) | (0, x) => println!("Eksen: {x}"),
    (x, y) if y < 0 => println!("Alt: ({x},{y})"),
    (x, y)          => println!("Diğer: ({x},{y})"),
}

// while let: lazy akış kontrolü
let mut yığın = vec![1, 2, 3];
while let Some(üst) = yığın.pop() {
    println!("{üst}");
}
```

>  Tam örnek: [`ornekler/06_sozdizim/src/main.rs`](ornekler/06_sozdizim/src/main.rs)

---

## 8. ADT Yapısı, Tipler ve Generiklik

Rust, **algebraic data type (ADT)** sistemini iki temel yapı üzerinden destekler:

- **`struct`** → product type (tüm alanlar birlikte var olur)
- **`enum`** → sum type (varyantlardan biri var olur)

```rust
// Enum: sum type (ADT) — varyantlar farklı veri taşır
enum Şekil {
    Nokta,
    Daire { yarıçap: f64 },
    Dikdörtgen(f64, f64),
}

fn alan(s: &Şekil) -> f64 {
    match s {
        Şekil::Nokta              => 0.0,
        Şekil::Daire { yarıçap } => std::f64::consts::PI * yarıçap * yarıçap,
        Şekil::Dikdörtgen(a, b)  => a * b,
    }
}

// Generic fonksiyon + where clause
fn en_büyük<T>(liste: &[T]) -> &T
where T: PartialOrd {
    let mut max = &liste[0];
    for öge in liste {
        if öge > max { max = öge; }
    }
    max
}
// Monomorphization: i32, f64 vb. için ayrı makine kodu üretilir
```

| Tip | Açıklama |
|-----|----------|
| `Option<T>` | `Some(T)` veya `None` — null-free yokluk yönetimi |
| `Result<T, E>` | `Ok(T)` veya `Err(E)` — hata yönetimi |
| `Vec<T>` | Dinamik dizi |
| `HashMap<K, V>` | Anahtar-değer eşlemi |
| `Box<T>` | Heap'te yaşayan akıllı işaretçi |

>  Tam örnek: [`ornekler/07_adt_generik/src/main.rs`](ornekler/07_adt_generik/src/main.rs)

---

##  Örnekleri Çalıştırma

Rust kurulu değilse önce yükleyin:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Herhangi bir örneği çalıştırmak için:

```bash
cd ornekler/01_oop
cargo run
```

Tüm örnekleri workspace üzerinden çalıştırmak için:

```bash
cargo run -p oop
cargo run -p paralellik
cargo run -p fonksiyonel
cargo run -p bellek
cargo run -p ayirt-edici
cargo run -p sozdizim
cargo run -p adt-generik
```

---

##  Kaynaklar

-  [The Rust Programming Language](https://doc.rust-lang.org/book/) — Resmi kitap
-  [Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe Rust
-  [Rust Reference](https://doc.rust-lang.org/reference/) — Dil referansı
-  Programming Rust — Jim Blandy, O'Reilly (2021)
-  [Rust RFC Book](https://rust-lang.github.io/rfcs/)

---

<div align="center">
<sub>Bursa Teknik Üniversitesi · Programlama Dilleri · 2025–2026 Bahar · Büşra Ünal</sub>
</div>
