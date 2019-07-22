use std::io;

fn main() {
    let mut inputTemp = String::new();

    io::stdin()
        .read_line(&mut inputTemp)
        .expect("Failed to read line");
    
    let inputTemp : f64 = match inputTemp.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("Invalid input!")
    };

    println!("Input {:?}", inputTemp);
}
