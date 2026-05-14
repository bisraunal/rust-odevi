// BLM0238 — Rust: Ownership, Borrowing ve Lifetime
// Move semantiği, borrow checker, lifetime anotasyonu

fn move_semantigi() {
    let s1 = String::from("merhaba");
    let s2 = s1;               // sahiplik s2'ye taşındı (move)
    // println!("{s1}");        // ❌ derleme hatası — s1 artık geçersiz
    println!("s2 = {s2}");     // ✅

    // i32 Copy trait'ini uygular → move değil kopya
    let x = 5;
    let y = x;
    println!("x={x}, y={y}"); // ✅ ikisi de geçerli
}

fn borrowing() {
    let s = String::from("dünya");

    // Birden fazla immutable borrow → OK
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    // Mutable borrow — o anda başka borrow olamaz
    let mut veri = vec![1, 2, 3];
    let son = veri.last().unwrap(); // immutable borrow
    println!("Son eleman: {son}");
    // Borrow burada bitti; şimdi mutable borrow güvenli
    veri.push(4);
    println!("Güncel: {:?}", veri);
}

// 'a lifetime: dönüş referansı, en kısa yaşayan parametreden daha uzun yaşayamaz
fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn lifetime_ornegi() {
    let s1 = String::from("uzun string");
    let sonuc;
    {
        let s2 = String::from("kısa");
        sonuc = en_uzun(s1.as_str(), s2.as_str());
        println!("En uzun: {sonuc}"); // ✅ s2 hâlâ yaşıyor
    }
    // println!("{sonuc}"); // ❌ s2 kapsam dışı — derleyici engeller
}

fn main() {
    println!("=== Move Semantiği ===");
    move_semantigi();
    println!("\n=== Borrowing ===");
    borrowing();
    println!("\n=== Lifetime ===");
    lifetime_ornegi();
}
