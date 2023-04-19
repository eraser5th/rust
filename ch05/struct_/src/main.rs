fn main() {
    just_value();
    tupple();
    struct_();
    debug();
}

fn just_value() {
    let width = 30;
    let height = 50;

    println!(
       "The area of the rectangle is {} square pixels.",
        area1(width, height),
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn tupple() {
    let rect = (30, 50);
    println!(
       "The area of the rectangle is {} square pixels.",
        area2(rect),
    )
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_() {
    let rect = Rectangle { width: 30, height: 50 };

    println!(
       "The area of the rectangle is {} square pixels.",
        area3(&rect),
    )
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn debug() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("rect is {:#?}", rect);
}
