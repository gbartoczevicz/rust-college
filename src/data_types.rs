fn primitive_types() {
    let i8_symbol: i8 = 10;
    let i32_symbol: i32 = 10;
    let i64_symbol: i64 = 10;
    let i128_symbol: i128 = 10;

    let f32_symbol: f32 = 10.0;

    let bool_symbol: bool = true;

    let char_symbol: char = 'e';

    println!(
        "Integer types {} i8, {} i32, {} i64, {} i128",
        i8_symbol, i32_symbol, i64_symbol, i128_symbol
    );
    println!("Float type {}", f32_symbol);
    println!("Bool type {}", bool_symbol);
    println!("Char type {}", char_symbol);
}

mod structures {
    struct Date {
        year: i32,
        month: i32,
        day: i32,
    }

    impl Date {
        fn formatted(&self) -> String {
            format!("{}-{}-{}", self.year, self.month, self.day)
        }
    }

    pub fn main() {
        let date = Date {
            year: 2022,
            month: 5,
            day: 12,
        };

        println!("The date is {}", date.formatted())
    }
}

mod unions {
    #[repr(C)]
    union AnyUnion {
        value_a: i32,
        value_b: i32,
    }

    pub fn main() {
        let mut u = AnyUnion { value_a: 1 };

        unsafe {
            println!("U {} {}", u.value_a, u.value_b);

            let b = &mut u.value_b;

            println!("B {}", b);

            *b = 2;

            println!("U {} {}", u.value_a, u.value_b);
        }
    }
}

fn vectors_and_matrix_and_tuples() {
    let arr = vec![1, 2, 3];

    println!("Arr {:?}", arr);

    let matrix = vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    println!("Matrix {:?}", matrix);
    println!("Matrix [2, 1] {}", matrix[2][1]);

    let tuple = ("VALUE A", "VALUE B");

    println!("Tuple {:?}", tuple);
}

mod pointers_and_recursive_types {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    pub fn main() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        println!("List {:?}", list);
    }
}

mod traits {
    struct Bear {
        name: &'static str,
        eating_honey: bool,
    }

    trait Animal {
        fn new(name: &'static str) -> Self;

        fn name(&self) -> &'static str;

        fn noise(&self) -> &'static str;

        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Bear {
        fn is_eating_honey(&self) -> bool {
            self.eating_honey
        }

        fn eat_honey(&mut self) {
            if self.is_eating_honey() {
                println!("{} is eating honey already", self.name);
            } else {
                self.eating_honey = true;
            }
        }
    }

    impl Animal for Bear {
        fn new(name: &'static str) -> Self {
            Bear {
                name,
                eating_honey: false,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_eating_honey() {
                "*eating honey*"
            } else {
                "AAAUGH"
            }
        }
    }

    pub fn main() {
        let mut yogi_bear: Bear = Animal::new("Zé Colmeia");

        yogi_bear.talk();
        yogi_bear.eat_honey();
        yogi_bear.eat_honey();
        yogi_bear.talk();
    }
}

pub fn main() {
    println!("Primitives\n");
    primitive_types();

    println!("\nStructs\n");
    structures::main();

    println!("\nUnions\n");
    unions::main();

    println!("\nVectors, Matrixes and Tuples\n");
    vectors_and_matrix_and_tuples();

    println!("\nPointers and recursive types\n");
    pointers_and_recursive_types::main();

    println!("\nTraits\n");
    traits::main();
}
