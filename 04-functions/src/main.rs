fn main() {
    println!("Hello, world!");
    let x = return_x(5, 6);
    let y = return_y(5, 6);

    println!("{}", x);
    println!("{}", y);
}

fn return_x(x: i32, y: i32) -> i32 {
    println!("Another function");
    println!("x is {} , y is {}", x, y);
    // A value can be returned with the following statement
    return x;
}

fn return_y(x: i32, y: i32) -> i32 {
    // The following statement returns the value of y.
    //To make it an expression (A set of instructions that returns a value) the semi-colon should be omitted
    x + y
}
