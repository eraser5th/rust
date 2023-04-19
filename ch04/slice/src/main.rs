fn main() {
    let s1 = String::from("Hello world");
    let first_word1_result = first_word1(&s1);
    println!("first_word1: {}", first_word1_result);

    let s2 = String::from("Hello world");
    let first_word2_result = first_word2(&s2);
    println!("first_word2: {}", first_word2_result);

    let s3 = String::from("Hello world");
    let first_word3_result = first_word3(&s3[..]);
    println!("first_word3: {}", first_word3_result);

    let first_word3_result = first_word3("hello world");
    println!("first_word3: {}", first_word3_result);
}

// return end of first word
fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// return slice of first word
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
