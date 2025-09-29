pub mod hosting;
pub mod serving;

// 在父模块中重新导出 hosting 的函数，让外部直接访问
pub use hosting::add_to_waitlist;
