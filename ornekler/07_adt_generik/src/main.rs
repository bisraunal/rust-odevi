// BLM0238 — Rust: ADT (Algebraic Data Types) ve Generiklik
// enum (sum type), struct (product type), Option, Result, generik fonksiyon

use std::f64::consts::PI;

// Enum: sum type — varyantlar farklı veri taşır
#[derive(Debug)]
enum Şekil {
    Nokta,
    Daire { yarıçap: f64 },
    Dikdörtgen(f64, f64),
    Üçgen { taban: f64, yükseklik: f64 },
}

impl Şekil {
    fn alan(&self) -> f64 {
        match self {
            Şekil::Nokta                           => 0.0,
            Şekil::Daire { yarıçap }               => PI * yarıçap * yarıçap,
            Şekil::Dikdörtgen(a, b)                => a * b,
            Şekil::Üçgen { taban, yükseklik }      => 0.5 * taban * yükseklik,
        }
    }
}

// Generic fonksiyon + where clause (trait bound)
fn en_büyük<T>(liste: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut max = &liste[0];
    for öge in liste {
        if öge > max { max = öge; }
    }
    max
}

// Generic struct
#[derive(Debug)]
struct Nokta<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Nokta<T> {
    fn göster(&self) {
        println!("Nokta({}, {})", self.x, self.y);
    }
}

fn main() {
    // Enum ADT
    let şekiller = vec![
        Şekil::Nokta,
        Şekil::Daire { yarıçap: 3.0 },
        Şekil::Dikdörtgen(4.0, 5.0),
        Şekil::Üçgen { taban: 6.0, yükseklik: 4.0 },
    ];
    for s in &şekiller {
        println!("{:?} → alan = {:.2}", s, s.alan());
    }

    // Generik fonksiyon: monomorphization ile i32 ve f64 için ayrı kod
    let tam_sayılar = vec![34, 50, 25, 100, 65];
    println!("\nEn büyük tam sayı: {}", en_büyük(&tam_sayılar));

    let ondalıklar = vec![3.14, 2.71, 1.61];
    println!("En büyük ondalık: {}", en_büyük(&ondalıklar));

    // Generic struct
    let p_int   = Nokta { x: 5_i32, y: 10 };
    let p_float = Nokta { x: 1.5_f64, y: 4.2 };
    p_int.göster();
    p_float.göster();

    // Option<T> ve Result<T,E>
    let değerler: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let toplam: i32 = değerler.iter().flatten().sum();
    println!("\nOption toplamı: {toplam}"); // 4

    let sonuçlar: Vec<Result<i32, _>> = vec!["1","hata","3"]
        .iter().map(|s| s.parse::<i32>()).collect();
    for (i, r) in sonuçlar.iter().enumerate() {
        match r {
            Ok(n)  => println!("sonuç[{i}] = {n}"),
            Err(e) => println!("sonuç[{i}] hata: {e}"),
        }
    }
}
