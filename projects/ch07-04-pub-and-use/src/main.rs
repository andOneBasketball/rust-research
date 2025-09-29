mod front_of_house;

// 直接 use 模块里的函数
use crate::front_of_house::hosting::seat_at_table;

// 使用 as 给函数重命名
use crate::front_of_house::serving::take_order as take_order_from_serving;

fn main() {
    // 1. 通过 re-export 的 pub use 使用函数
    front_of_house::add_to_waitlist();

    // 2. 直接用 use 导入的函数
    seat_at_table();

    // 3. 用 as 重命名后的函数
    take_order_from_serving();
}
