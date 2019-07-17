#![allow(unused_variables)]

fn main() {
    // mutable variable. Only the value can be mutated and not the type
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;

    println!("x is now {}", x);

    let a = b'A';
    println!("a is {}", a);

    // Constant declaration. Constants are always immutable
    // Integer values can have a visual sperator _ for easy readability. The seperator doesnot affect the value
    const MAX_POINTS: u32 = 100_000;

    println!("max points is {}", MAX_POINTS);

    // Shadowing is used to change the value of a variable as well as its type.

    let y = 1;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is {}", y);

    // Compound types

    // Tuples
    // Elements in a tuple can be of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value in the tuple tup is {:?}", tup);

    // Elements can be accessed by the . operator
    println!("First element in the tuple is {}", tup.0);

    // Destructuring a tuple
    let (x, y, z) = tup;

    println!("x {}, y {}, z {}", x, y, z);

    // Arrays
    // Elements in an Array should be of the same type.
    // The length is fixed in an array

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Months array : {:?}", months);

    // Accessing array elements

    let first = months[0];

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
}
