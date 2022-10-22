use rand::{self, Rng};
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Number {
    value: i8,
}

impl ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        println!("Adding {} and {}", self.value, rhs.value);

        Number {
            value: self.value + rhs.value,
        }
    }
}

pub fn main() {
    let mut rng = rand::thread_rng();

    let number_x = Number {
        value: rng.gen_range(0..=9),
    };
    let number_y = Number {
        value: rng.gen_range(0..=9),
    };

    println!("X {:?}", number_x);
    println!("Y {:?}", number_y);

    let number_x_and_y_added = number_x + number_y;

    println!(
        "{:?} + {:?} = {:?}",
        number_x, number_y, number_x_and_y_added
    );
}
