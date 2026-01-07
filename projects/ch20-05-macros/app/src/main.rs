use my_derive::HelloMacro;

#[derive(HelloMacro)]
struct Counter;

fn main() {
    Counter::hello();
}
