#![allow(dead_code)]
pub fn for_loop() {
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{} = {}", pos, y);
    }
}

pub fn match_statement() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "invalid",
    };

    println!("Code {} = Country {}", country_code, country);
}
