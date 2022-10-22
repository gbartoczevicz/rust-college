use rand::{self, Rng};

pub fn main() {
    println!("{} + {} = {}", 3, 6, 3 + 6);
    println!("{} - {} = {}", 5.5, 1.25, 5.5 - 1.25);
    println!("{} * {} = {}", -5, 14, -5 * 14);
    println!("{} / {} = {}", 14, 3, 14 / 3);
    println!("{} % {} = {}", 100, 7, 100 % 7);
    println!("{} & {} = {}", 0b1010, 0b1100, 0b1010 & 0b1100);
    println!("{} | {} = {}", 0b1010, 0b1100, 0b1010 | 0b1100);
    println!("{} ^ {} = {}", 0b1010, 0b1100, 0b1010 ^ 0b1100);
    println!("{} << {} = {}", 13, 3, 13 << 3);
    println!("{} >> {} = {}", -10, 2, -10 >> 2);

    let mut rng = rand::thread_rng();

    let x_and_y: (i8, i8) = rng.gen();

    if x_and_y.0 > x_and_y.1 {
        println!("{} is greater than {}", x_and_y.0, x_and_y.1);
    } else {
        println!("{} is not greater than {}", x_and_y.0, x_and_y.1);
    }

    let between_0_and_9: i8 = rng.gen_range(0..=9);

    match between_0_and_9 {
        0..=5 => println!("{} is between 0 and 5", between_0_and_9),
        _ => println!("{} is not between 0 and 5", between_0_and_9),
    }

    let is_true_or_false = rng.gen_bool(0.5);

    let binary = match is_true_or_false {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", is_true_or_false, binary);

    if !is_true_or_false {
        println!("{} is false", is_true_or_false)
    } else {
        println!("{} is actually true", is_true_or_false)
    }

    if x_and_y.0 > x_and_y.1 && binary == 1 {
        println!(
            "{} is greater than {} and {} is equal 1",
            x_and_y.0, x_and_y.1, binary
        );
    } else {
        println!(
            "or {} is not greater than {} or {} is not equal 1",
            x_and_y.0, x_and_y.1, binary
        );
    }

    if x_and_y.0 > x_and_y.1 || binary == 1 {
        println!(
            "or {} is greater than {} or {} is equal 1",
            x_and_y.0, x_and_y.1, binary
        );
    } else {
        println!(
            "{} is not greater than {} and {} is not equal 1",
            x_and_y.0, x_and_y.1, binary
        );
    }
}
