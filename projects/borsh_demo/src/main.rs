use borsh::{BorshDeserialize, BorshSerialize};
use std::cell::RefCell;
use std::io::Cursor;

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
struct Counter {
    count: u32,
}

// 这里接收 slice &mut [u8]，返回 std::io::Result，保留 ? 用法
fn serialize_example(data: &mut [u8]) -> std::io::Result<()> {
    let counter = Counter { count: 99 };

    // ❗ 将 slice 包装成 Cursor，Cursor 实现了 std::io::Write
    let mut writer = Cursor::new(data);

    // ❗ 使用 ? 自动传播错误
    counter.serialize(&mut writer)?;

    Ok(())
}

fn main() {
    let counter = Counter { count: 42 };

    println!("Original struct: {:?}", counter);

    // =========================
    // 方式一：使用 serialize 写入 Vec<u8>
    // =========================
    let mut buf = Vec::new();
    counter.serialize(&mut buf).unwrap();

    println!("\n== serialize() ==");
    println!("bytes: {:?}", buf);
    println!("len: {}", buf.len());

    let decoded1 = Counter::try_from_slice(&buf).unwrap();
    println!("decoded from serialize: {:?}", decoded1);

    // =========================
    // 方式二：使用 borsh::to_vec
    // =========================
    let bytes2 = borsh::to_vec(&counter).unwrap();

    println!("\n== borsh::to_vec() ==");
    println!("bytes: {:?}", bytes2);
    println!("len: {}", bytes2.len());

    let decoded2 = Counter::try_from_slice(&bytes2).unwrap();
    println!("decoded from to_vec: {:?}", decoded2);

    // =========================
    // 对比验证
    // =========================
    println!("\n== Compare ==");
    println!("serialize bytes == to_vec bytes ? {}", buf == bytes2);
    println!("decoded1 == decoded2 ? {}", decoded1 == decoded2);

    // 模拟账户数据
    let account_data = RefCell::new(vec![0u8; 10]);

    // 借 mutable
    {
        let mut borrow_mut = account_data.borrow_mut(); // RefMut<Vec<u8>>

        // &mut &mut slice
        let slice: &mut [u8] = &mut borrow_mut[..]; // slice
        let result = serialize_example(&mut slice[..]); // &mut &mut [u8]
        match result {
            Ok(_) => println!("serialize ok! {:?}", slice),
            Err(e) => println!("serialize error: {:?}", e),
        }
    }

    println!("account_data after serialize: {:?}", account_data.borrow());
}
