mod pm;

fn generics(){
    // Create a struct with generic types
    struct Point<T>{
        x: T,
        y: T
    }

    let a: Point<i32> = Point{ x: 0, y: 0}; // Point with type i32
    let b: Point<f32> = Point{ x: 1.3, y: 4.5}; // Point with type f32
    let c: Point<i32> = Point{ x: 2, y: 2};

    // Compose a struct with another generic struct
    // In the following case both start and end must of the the same type
    struct Line<T>{
        start: Point<T>,
        end: Point<T>
    }

    let l1 = Line{ start: a, end: c};
}

fn main() {
    pm::pattern_matching();
    generics();
}
