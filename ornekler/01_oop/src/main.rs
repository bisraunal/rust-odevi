// BLM0238 — Rust: Nesneye Yönelik Programlama
// struct + impl + trait ile OOP; dyn Trait ile çalışma-zamanı polimorfizm

trait Ses {
    fn cikar(&self) -> &str;
    /// Varsayılan implementasyon — alt türler isteğe bağlı yeniden tanımlar
    fn tanitim(&self) {
        println!("Ben {}. Sesim: {}", self.isim(), self.cikar());
    }
    fn isim(&self) -> &str;
}

struct Kedi {
    ad: String,
}

struct Köpek {
    ad: String,
}

impl Ses for Kedi {
    fn cikar(&self) -> &str { "Miyav" }
    fn isim(&self) -> &str  { &self.ad }
}

impl Ses for Köpek {
    fn cikar(&self) -> &str { "Hav" }
    fn isim(&self) -> &str  { &self.ad }
}

/// dyn Ses → vtable üzerinden çalışma-zamanı dispatch (dinamik polimorfizm)
fn hepsini_tanit(hayvanlar: &[&dyn Ses]) {
    for h in hayvanlar {
        h.tanitim();
    }
}

fn main() {
    let kedi  = Kedi  { ad: String::from("Pamuk") };
    let kopek = Köpek { ad: String::from("Karabaş") };

    // Trait nesnesi: farklı tipler aynı slice'ta toplanabilir
    let hayvanlar: Vec<&dyn Ses> = vec![&kedi, &kopek];
    hepsini_tanit(&hayvanlar);
}
