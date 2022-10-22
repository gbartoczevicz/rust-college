const THIS_ONE_IS_GLOBAL: &str = "A global symbol";

pub fn main() {
    let a_number: i32 = 1;

    let or_like_this = 1;

    println!("Local symbols {} {}", a_number, or_like_this);

    println!("Global symbol {}", THIS_ONE_IS_GLOBAL);

    let mut to_mutate = "I have this value";

    println!("This value will change -> {}", to_mutate);

    to_mutate = "Now I have another value";

    println!("Now the value changed -> {}", to_mutate);
}
