fn main() {
    println!("Hello, world!");

    another_function();
    another_function_2(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");  // 別の関数
}

fn another_function_2(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
