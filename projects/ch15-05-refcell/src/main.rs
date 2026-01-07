use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    count: i32,
}

fn main() {
    let counter = RefCell::new(Counter { count: 0 });

    println!("== 第一次只读借用 ==");
    {
        let c1 = counter.borrow(); // 不可变借用
        println!("c1 = {:?}", c1);

        // ⚠️ 这里如果再 borrow_mut 会直接 panic（运行期检查）
        // let c2 = counter.borrow_mut();
    } // c1 在这里被 drop，借用结束

    println!("== 可变借用修改值 ==");
    {
        let mut c2 = counter.borrow_mut(); // 可变借用
        c2.count += 10;
        println!("c2 = {:?}", c2);
    } // c2 drop

    println!("== 再次只读借用 ==");
    {
        let c3 = counter.borrow();
        println!("c3 = {:?}", c3);
    }

    println!("== 连续两个只读借用是允许的 ==");
    {
        let c4 = counter.borrow();
        let c5 = counter.borrow();
        println!("c4 = {:?}", c4);
        println!("c5 = {:?}", c5);
    }

    println!("== 同时可变 + 不可变借用会 panic（我给你演示） ==");
    {
        let _c6 = counter.borrow();
        println!("already borrowed immutably");

        // 运行到这句会 panic
        let _c7 = counter.borrow_mut();
        println!("never printed");
    }
}
