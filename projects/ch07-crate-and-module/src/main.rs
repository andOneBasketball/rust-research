mod front_of_house;

fn main() {
    // 直接调用 pub 函数
    crate::front_of_house::hosting::add_to_waitlist();

    // 通过一个高层函数间接调用
    eat_at_restaurant();
}

fn eat_at_restaurant() {
    // 使用 pub 暴露的路径
    front_of_house::hosting::seat_at_table();

    // 使用 super 从当前模块（main.rs）往上找
    order_food();
}

fn order_food() {
    println!("Ordering some food!");
}
