use lib::calc;
use lib::time;
use lib::time::get_seconds_millis;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, TimeZone, Utc};

fn main() {
    let mut testArray = vec![];
    let mut isRandoming = true;
    println!("Hello Rust!\n");
    println!("1+1 equals {}\n", calc::add(1, 1));
    println!("4-1 equals {}\n", calc::sub(4, 1));
    println!("7*4 equals {}\n", calc::multi(7, 4));
    println!("30/5 equals {}\n", calc::div(30, 5));
    println!("Random number: {}\n", calc::rand(3, 80));
    for x in 1..calc::rand(82, 108) {
        testArray.push(calc::rand(34, 76));
    }

    println!("Random array: {:?}", testArray);
    let neededRandNum = calc::rand(1, 245);
    println!("Number needed: {}", neededRandNum);

    while isRandoming {
        let randnum = calc::rand(1, 245);
        println!("Number generated: {}", randnum);
        if randnum == neededRandNum {
            println!("Number found!");
            isRandoming = false;
        }
    }

    let mut idkarray = vec![];
    let randIdkTime = Instant::now();
    println!("Generating a 95000 random number array...");
    for x in 1..95000 {
        idkarray.push(calc::rand(1, 95000));
    }

    let idkElapsedTime = randIdkTime.elapsed();
    println!("Random generated array: {:?}", idkarray);
    println!("Generated in {} seconds", get_seconds_millis(idkElapsedTime));
    println!("UTC Time: {:?}", Utc::now())

    println!("Thanks for using this!");
    println!("Credits:"); // json parser thing later
    println!("- Rindo (aka AmixelHello)");
    println!("- Aquiles Trindade");
}