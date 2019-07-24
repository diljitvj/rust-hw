
struct Point {
    x: f64,
    y: f64
}

struct  Line {
    start: Point,
    end: Point
}

fn structures(){
    let p1 = Point{ x: 1.0, y:4.0};
    let p2 = Point{x: 5.0, y: 10.0};

    let l1 = Line{start: p1, end: p2};

    println!("Line l1 starts from {},{} to {},{}", l1.start.x, l1.start.y, l1.end.x, l1.end.y);
}

enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8), // tuple
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8} // struct
}

fn enums(){
//    let b: Color = Color::RgbColor(0,0,0);
    let c: Color = Color::CmykColor {cyan: 0, magenta: 128, yellow: 24, black: 0};

    match c {
        Color::Blue => {
            println!("Blue")
        },
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::RgbColor(0,0,0) => println!("Black"),
        Color::CmykColor {cyan: c, magenta: m, yellow: y, black: b} => println!("CMYK color c{} m{} y{} k{}", c, m, y, b),
        _ => println!("Some color")
    }

}

fn main(){
    structures();
    enums();
}