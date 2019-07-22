#![allow(unused_variables)]


fn main() {

    let boolean : bool = false;
    let string_val : &str = "123";

    println!("boolean = {} size = {} byte", boolean, std::mem::size_of_val(&boolean));
    println!("string_val = {} size = {} byte", string_val, std::mem::size_of_val(&string_val));

    // mutable variable. Only the value can be mutated and not the type
    // integer types i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    let mut x: i32 = 5;
    println!("The value of x is {} , size = {} bytes ({} bits)", x, std::mem::size_of_val(&x),  std::mem::size_of_val(&x) * 8);
    x = 6;
    println!("x is now {}", x);

    x = i32::pow(x, 3);
    println!("x cubed = {}", x);

    let a = b'A';
    println!("a is {}", a);

    let sizevar: isize = 1;
    println!("sizevar = {} , size of sizevar = {} bytes, {}-bit OS", sizevar, std::mem::size_of_val(&sizevar),std::mem::size_of_val(&sizevar) * 8 );

    // Constant declaration. Constants are always immutable
    // Integer values can have a visual seperator _ for easy readability. The seperator does not affect the value
    const MAX_POINTS: u32 = 100_000;

    println!("max points is {}", MAX_POINTS);

    // Shadowing is used to change the value of a variable as well as its type. The variable need not mutable to utilize shadowing.

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

    let months: [&str; 12] = [
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

    let first: &str = months[0];

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
