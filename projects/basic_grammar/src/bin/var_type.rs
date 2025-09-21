use std::any::type_name;

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    // u32 f64 i32 i64 isize
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {guess}, type is {}", type_of(&guess));

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("x + y = {}", x + y as f64);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum = {}, difference = {} {}, product = {}, quotient = {} {}, truncated = {}, remainder = {}",
        sum,
        difference,
        type_of(&difference),
        product,
        quotient,
        type_of(&quotient),
        truncated,
        remainder
    );

    // bool char
    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!(
        "five_hundred = {}, six_point_four = {}, one = {}, x = {}",
        five_hundred, six_point_four, one, x.1
    );
}
