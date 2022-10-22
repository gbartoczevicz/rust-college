mod data_types;
mod operators;
mod symbol_definitions;

fn main() {
    println!("===== Data Types =====\n");

    data_types::main();

    println!("===== Symbol Definitions =====\n");

    symbol_definitions::main();

    println!("===== Operators =====\n");

    operators::main();
}
