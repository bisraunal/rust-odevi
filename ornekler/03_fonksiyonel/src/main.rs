// BLM0238 — Rust: Fonksiyonel Programlama
// Closure, lazy iterator zinciri, Option/Result monadik zincir

fn closure_ornegi() {
    // Fn: salt okur | FnMut: değiştirir | FnOnce: sahipliği taşır
    let çarpan = 3;
    let uc_ile_carp = |x: i32| x * çarpan; // Fn — çarpan'ı yakalar
    println!("5 × 3 = {}", uc_ile_carp(5));

    let mut toplam = 0;
    let mut birikimli_topla = |x: i32| { toplam += x; toplam }; // FnMut
    println!("1+2+3 = {}", [1, 2, 3].iter().map(|&x| birikimli_topla(x)).last().unwrap());
}

fn iterator_zinciri() {
    let kelimeler = vec!["rust", "go", "python", "haskell", "ruby"];

    // Lazy: collect() çağrılana dek hiçbir iş yapılmaz
    let sonuc: Vec<String> = kelimeler.iter()
        .filter(|&&k| k.len() > 2)
        .filter(|&&k| k.contains('r'))
        .map(|&k| k.to_uppercase())
        .collect();

    println!("Filtrelenmiş: {:?}", sonuc); // ["RUST", "PYTHON", "RUBY"]

    let toplam: i32 = (1..=10).filter(|x| x % 2 == 0).sum();
    println!("1–10 çiftlerin toplamı: {toplam}"); // 30
}

fn option_result_zinciri() {
    // Option monadı: and_then ile güvenli zincir
    let belki_sayi: Option<i32> = Some("42".parse().ok()?);
    let sonuc = belki_sayi
        .filter(|&n| n > 0)
        .map(|n| n * 2);
    println!("Option sonucu: {:?}", sonuc); // Some(84)

    // Result zinciri
    let r: Result<i32, _> = "10"
        .parse::<i32>()
        .map(|n| n + 5)
        .map_err(|e| format!("Hata: {e}"));
    println!("Result sonucu: {:?}", r); // Ok(15)
}

fn main() {
    println!("=== Closure ===");
    closure_ornegi();
    println!("\n=== Iterator Zinciri ===");
    iterator_zinciri();
    println!("\n=== Option / Result ===");
    option_result_zinciri();
}
