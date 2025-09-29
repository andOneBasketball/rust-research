pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Added to waitlist");
    }

    pub fn seat_at_table() {
        println!("Seated at table");
        // 使用 super 访问上一级（front_of_house 模块）
        super::serving::take_order();
    }
}

pub mod serving {
    pub fn take_order() {
        println!("Order is taken");
    }
}
