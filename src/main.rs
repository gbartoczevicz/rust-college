mod data_types;
mod operator_overload;
mod operators;
mod symbol_definitions;
mod todo_list;

fn main() {
    println!("===== Data Types =====\n");

    data_types::main();

    println!("===== Symbol Definitions =====\n");

    symbol_definitions::main();

    println!("===== Operators =====\n");

    operators::main();

    println!("===== Operator Overload =====\n");

    operator_overload::main();

    println!("\n============================================================\n");

    todo_list::main();
}
