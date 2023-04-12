fn main() {
    if_expression();
    loop_test();
    while_test();
    for_test();
    for_test_2();

    let n = 5;
    println!("フィボナッチの{}番目は: {}", n, fibonacci(n));
}

fn if_expression() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if false { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn while_test() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn for_test_2() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) * 2 + 1
    }
}
