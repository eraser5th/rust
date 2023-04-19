fn main() {
    ref_test1();
    mut_ref_test1();
}

fn ref_test1() {
    println!("ref_test1");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mut_ref_test1() {
    println!("ref_test1");
    let mut s = String::from("hello");
    change(&mut s);
    println!("changed s is '{}'", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn hoge() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}
*/

/*
fn hoge() {
    let mut s = String::from("hello");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！
}
*/
