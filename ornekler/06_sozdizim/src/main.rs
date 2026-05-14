// BLM0238 — Rust: Sözdizim ve Anlam Bilgisi
// Expression-based, shadowing, exhaustive pattern matching

fn expression_based() {
    // if bir değer döndürür
    let yas: u32 = 20;
    let mesaj = if yas >= 18 { "yetişkin" } else { "çocuk" };
    println!("{mesaj}");

    // Blok da bir ifadedir
    let kare = {
        let x = 5;
        x * x   // noktalı virgül yok → değer döndürür
    };
    println!("Kare: {kare}");
}

fn shadowing_ornegi() {
    let deger = "25";
    let deger: u32 = deger.parse().unwrap(); // aynı isim, farklı tip
    let deger = deger * 2;
    println!("Gölgelenmiş deger: {deger}"); // 50
}

fn pattern_matching() {
    let koordinat = (3_i32, -5_i32);
    match koordinat {
        (0, 0)          => println!("Orijin"),
        (x, 0) | (0, x) => println!("Eksen üzerinde: {x}"),
        (x, y) if y < 0 => println!("Alt yarı düzlem: ({x}, {y})"),
        (x, y)          => println!("Genel konum: ({x}, {y})"),
    }

    // if let: tek dal için kısa yol
    let belki: Option<i32> = Some(7);
    if let Some(n) = belki {
        println!("Değer var: {n}");
    }

    // while let: lazy akış
    let mut yığın = vec![1, 2, 3];
    while let Some(üst) = yığın.pop() {
        println!("Pop: {üst}");
    }
}

fn main() {
    println!("=== Expression-based ===");
    expression_based();
    println!("\n=== Shadowing ===");
    shadowing_ornegi();
    println!("\n=== Pattern Matching ===");
    pattern_matching();
}
