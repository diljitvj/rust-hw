
fn how_many(x: i32) -> &'static str{
    match x {
        1 => "one",
        _ if x% 2 == 0 => "even number",
        3 | 5 => "some",
        7...10 => "many",
        _ => "too many"
    }
}

fn which_quadrant(t : (i32,i32)) -> &'static str{
    match t {
        (x,y) if x > 0 && y > 0  => "1st Quad",
        (x,y) if x < 0 && y > 0  => "2st Quad",
        (x,y) if x < 0 && y < 0  => "3st Quad",
        (x,y) if x > 0 && y < 0  => "4st Quad",
        (0,0)=> "origin",
        (0,y) => "y-axis",
        (x,0)=> "x-axis",
        _ => "Don't know"
    }
}

pub fn pattern_matching(){
    for x in 1..12 {
        println!("{} : I have {} oranges", x, how_many(x));
    }

    println!("{}", which_quadrant((0,0)));
    println!("{}", which_quadrant((1,0)));
    println!("{}", which_quadrant((0,1)));
    println!("{}", which_quadrant((1,1)));
    println!("{}", which_quadrant((-1,1)));
    println!("{}", which_quadrant((-1,-1)));
    println!("{}", which_quadrant((1,-1)));
}