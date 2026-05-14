// BLM0238 — Rust: Fearless Concurrency
// Arc + Mutex ile paylaşımlı durum; mpsc::channel ile mesaj geçişi

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn paylasimli_sayac() {
    let sayac = Arc::new(Mutex::new(0_i32));

    let handles: Vec<_> = (0..5).map(|i| {
        let s = Arc::clone(&sayac);
        thread::spawn(move || {
            let mut kilit = s.lock().unwrap();
            *kilit += 1;
            println!("Thread {i}: sayac = {kilit}");
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("Son değer: {}", *sayac.lock().unwrap()); // 5
}

fn kanal_ornegi() {
    let (tx, rx) = mpsc::channel::<String>();

    // Üretici thread
    thread::spawn(move || {
        let mesajlar = vec!["merhaba", "rust", "kanalı"];
        for m in mesajlar {
            tx.send(m.to_string()).unwrap();
        }
    });

    // Ana thread: tüketici
    for alinan in rx {
        println!("Alındı: {alinan}");
    }
}

fn main() {
    println!("=== Paylaşımlı Sayaç ===");
    paylasimli_sayac();
    println!("\n=== Kanal Örneği ===");
    kanal_ornegi();
}
