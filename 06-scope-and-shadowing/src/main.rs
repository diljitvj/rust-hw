const GLOBAL_VAR: &str = "I am a global variable";
static mut MUTABLE_GLOBAL_VARIABLE : i32 = 5;

fn main(){
    let b: i32 = 6;
    another_scope(b);
    println!("b in main {}",b);
    println!("{}", GLOBAL_VAR);



    {
        // this is a new scope;

        let b: i32 = 8;
        println!("b inside scope {}",b);
    }

    unsafe {
        // To mutate a global variable the block of function needs to be declared unsafe.

        MUTABLE_GLOBAL_VARIABLE = 6;

    }


}

fn another_scope(mut b: i32){
    let a : i32 = 5;
    // a can be accessed only inside this scope
    println!("b in a function scope {}", b);
    b = 7;
    println!("b after mutation in a function scope {}", b);
    println!("a in a function scope {}", a);

}