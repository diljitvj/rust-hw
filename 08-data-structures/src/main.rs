
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

//
union IntOrFloat {
    i: i32,
    f: f32
}

fn unions(){
    let iof = IntOrFloat { i : 123 };

    unsafe {
        match iof {
            IntOrFloat{ i} => println!("Integer {}", iof.i),
            IntOrFloat{ f} => println!("Float {}", iof.f),
        }
    }
}

fn option(){
    let x:f32 = 1.0;
    let y:f32 = 1.0;

    let result: Option<f32> = if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {}/{}", x, y)
    }

    if let Some(z) = result { println!("Result is {}", z); };
    if let None = result { println!("Cannot divide");};

}

fn arrays(){
    let mut a : [i32;5]; // Declares a mutable array of size 5, which can store i32 values.
    a = [1,2,3,4,5];

    println!("The array a has {} and first element is {} ", a.len(), a[0]);
    println!("{:?}", a); // debug output for the array

    if a == [1,2,3,4,5] {
        println!("Array matches [1,2,3,4,5]");
    }

    let b = [1u16;10]; // fill the array with 10 1s and specify the size as u16

    for i in 0..b.len(){
        println!("{}", b[i]);
    }

    println!("b took {} bytes", std::mem::size_of_val(&b));

    let mtx:[[i32;3];3] = [[1,2,2],[3,2,3], [0,0,3]];

    println!("{:?}", mtx);

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j {
                println!("{}", mtx[i][j]);
            }
        }
    }


}

fn main(){
    structures();
    enums();
    unions();
    option();
    arrays();
}