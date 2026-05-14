// BLM0238 — Rust: Zero-cost Abstraction + unsafe
// C performansı, GC yok, bellek hatası yok

/// Zero-cost: derleyici bunu el-yazımı C döngüsüne eşdeğer ASM'e çevirir
fn toplam_iterator(v: &[i32]) -> i32 { v.iter().sum() }
fn toplam_dongu(v: &[i32]) -> i32 {
    let mut t = 0;
    for &x in v { t += x; }
    t
}

fn zero_cost_ornegi() {
    let sayilar: Vec<i32> = (1..=100).collect();
    assert_eq!(toplam_iterator(&sayilar), toplam_dongu(&sayilar));
    println!("İteratör ve döngü aynı sonucu verir: {}", toplam_iterator(&sayilar));
}

fn unsafe_ornegi() {
    // unsafe blok: ham pointer erişimi — bilinçli alınan risk
    let mut deger = 42_i32;
    let ptr: *mut i32 = &mut deger;

    unsafe {
        *ptr += 1;
        println!("Ham pointer ile değer: {}", *ptr); // 43
    }
}

fn main() {
    println!("=== Zero-cost Abstraction ===");
    zero_cost_ornegi();
    println!("\n=== unsafe Blok ===");
    unsafe_ornegi();
}
