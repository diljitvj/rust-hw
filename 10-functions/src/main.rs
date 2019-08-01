
// Function with a return value
fn product(x: i32, y: i32) -> i32 {
   x * y
}

// Adding methods to a struct
struct Point {
   x: f64,
   y: f64
}

struct Line {
   start : Point,
   end : Point
}

// Add a method to the struct Line
impl Line {
   fn len(&self) -> f64{
      let dx = self.end.x - self.start.x;
      let dy = self.end.y - self.start.y;

      (dy*dy + dx*dx).sqrt()
   }
}

fn main() {
   let a = product(3, 5);

   let p1 = Point{ x: 5.0, y: 2.0};
   let p2 = Point{ x: 10.0, y: 15.0};
   let l1 = Line { start: p1, end: p2};

   println!("The length of line is {}", l1.len())
}
