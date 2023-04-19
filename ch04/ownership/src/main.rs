fn main() {
    string();
    move_test1();
    move_test2();
    string_clone();
    ownership_and_function();
    return_and_scope1();
    return_and_scope2();
}

fn string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s)
}

fn move_test1() {
    let x = 5;
    let y = x;
    println!("[move_test1] x: {}, y: {}", x, y);
}

fn move_test2() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("[move_test2] s2: {}", s2);
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("[string_clone] s1: {}, s2: {}", s1, s2);
}

fn ownership_and_function() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_and_scope1() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_and_scope2() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
