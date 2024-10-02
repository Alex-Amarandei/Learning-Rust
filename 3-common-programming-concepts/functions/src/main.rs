fn main_1() {
    println!("Hello, world!");

    another_function_1();
}

fn another_function_1() {
    println!("Another function.");
}

fn main_2() {
    another_function_2(5);
}

fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

fn main_3() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn main_4() {
    let x = five();

    println!("The value of x is: {x}");
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
