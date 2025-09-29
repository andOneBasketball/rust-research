#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Circle {
    radius: u32,
    color: String,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let circle1 = Circle {
        radius: 20,
        color: "red".to_string(),
    };

    println!("circle1 is {circle1:?}");
    println!("rect1 is {rect1:?}");
}
