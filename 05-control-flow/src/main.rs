use std::io;

fn main() {
//    loop_and_if();
    for_loop();
    match_statement();
}

fn loop_and_if() {
    loop {
        let mut input_temp = String::new();

        println!("Enter temperature ");
        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        let temp: f64 = match input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue;
            }
        };

        if temp > 25.0 {
            println!("Too cold");
        } else {
            println!("Too hot");
        }
    }
}

fn for_loop() {
    for x in 1..11{
        if x == 3 { continue; }
//        if x == 8 { break; }
        println!("x is {}", x);
    }
}

fn match_statement(){
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        91 => "IN",
        1...99 => "Unknown", // 1 and 99 are included in the range
        _ => "Invalid"
    };

    println!("Country with code {} is {}", country_code, country);
}

