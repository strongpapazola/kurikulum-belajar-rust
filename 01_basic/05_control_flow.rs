// ============================================
// 05 - Control Flow: if, loop, while, for
// ============================================
// Jalankan: rustc 05_control_flow.rs && ./05_control_flow

fn main() {
    // ==================
    // IF / ELSE
    // ==================
    let nilai = 85;

    if nilai >= 90 {
        println!("A");
    } else if nilai >= 80 {
        println!("B");
    } else if nilai >= 70 {
        println!("C");
    } else {
        println!("D");
    }

    // if sebagai ekspresi (assign langsung ke variable)
    // mirip ternary operator di JS/PHP
    let grade = if nilai >= 80 { "Bagus" } else { "Cukup" };
    println!("Grade: {grade}");

    // ==================
    // LOOP — infinite loop, keluar pakai break
    // ==================
    let mut counter = 0;
    let hasil_loop = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // loop bisa return value!
        }
    };
    println!("Loop result: {hasil_loop}"); // 10

    // ==================
    // WHILE
    // ==================
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("n = {n}"); // 128

    // ==================
    // FOR — paling sering dipakai
    // ==================

    // Iterasi range
    for i in 1..=5 {  // 1..=5 = 1,2,3,4,5 (inklusif)
        print!("{i} ");
    }
    println!();

    for i in 0..5 {   // 0..5 = 0,1,2,3,4 (eksklusif akhir)
        print!("{i} ");
    }
    println!();

    // Iterasi array
    let buah = ["apel", "mangga", "jeruk", "durian"];
    for b in &buah {
        println!("Buah: {b}");
    }

    // Iterasi dengan index (enumerate)
    for (i, b) in buah.iter().enumerate() {
        println!("{i}: {b}");
    }

    // Iterasi Vec
    let angka = vec![10, 20, 30, 40, 50];
    for a in &angka {
        print!("{a} ");
    }
    println!();

    // ==================
    // MATCH — switch case versi Rust, lebih powerful
    // ==================
    let status_code = 404;

    match status_code {
        200 => println!("OK"),
        201 => println!("Created"),
        400 => println!("Bad Request"),
        404 => println!("Not Found"),
        500 => println!("Server Error"),
        _   => println!("Unknown: {status_code}"), // default / wildcard
    }

    // Match dengan range
    let skor = 75;
    match skor {
        90..=100 => println!("Excellent"),
        80..=89  => println!("Good"),
        70..=79  => println!("Average"),
        _        => println!("Below Average"),
    }

    // Match dengan multiple pattern
    let hari = 6;
    match hari {
        1 | 7   => println!("Weekend"),
        2..=6   => println!("Weekday"),
        _       => println!("Invalid"),
    }

    // Match return value
    let label = match status_code {
        200..=299 => "Success",
        400..=499 => "Client Error",
        500..=599 => "Server Error",
        _         => "Unknown",
    };
    println!("Label: {label}");

    // ==================
    // LOOP CONTROL
    // ==================
    // continue
    for i in 1..=10 {
        if i % 2 == 0 { continue; }
        print!("{i} "); // hanya ganjil
    }
    println!();

    // break dengan label (nested loop)
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                break 'outer; // keluar dari loop luar
            }
            println!("({i},{j})");
        }
    }
}

// ============================================
// Perbandingan:
//
// PHP:    switch($x) { case 1: ... break; }
// JS:     switch(x) { case 1: ... break; }
// Python: match x: case 1: ...  (Python 3.10+)
// Rust:   match x { 1 => ..., _ => ... }
//
// Rust match WAJIB exhaustive (semua kemungkinan harus di-cover)
// ============================================
