use borsh::{BorshDeserialize, BorshSerialize};
use std::cell::RefCell;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

fn main() -> std::io::Result<()> {
    println!("=== Cursor 高频用法 Demo ===");

    // 模拟 Solana account.data
    let account_data = RefCell::new(vec![0u8; 10]);

    {
        let mut borrow_mut = account_data.borrow_mut();
        let slice: &mut [u8] = &mut borrow_mut[..];

        // ===============================
        // 写入 slice
        // ===============================
        {
            // ❌ slice 不会被移动
            let mut cursor = Cursor::new(&mut slice[..]);
            cursor.write_all(&[1, 2, 3])?;
            cursor.seek(SeekFrom::Start(5))?;
            cursor.write_all(&[9, 9])?;
        } // Cursor 离开作用域，可变借用结束

        println!("写入 + Seek 后 slice: {:?}", slice);
    }

    println!("account_data 最终: {:?}", account_data.borrow());

    // ===============================
    // Borsh serialize 写入 slice
    // ===============================
    {
        let mut borrow_mut = account_data.borrow_mut();
        let slice: &mut [u8] = &mut borrow_mut[..];

        let counter = Counter { count: 42 };
        {
            let mut cursor = Cursor::new(&mut slice[..]);
            counter.serialize(&mut cursor)?;
        }

        println!("Borsh serialize 写入 slice: {:?}", slice);
    }

    // ===============================
    // 读取 slice
    // ===============================
    {
        let borrow = account_data.borrow();
        let slice: &[u8] = &borrow[..];

        let mut cursor = Cursor::new(slice);
        let mut tmp = [0u8; 4];
        cursor.read_exact(&mut tmp)?;
        println!("读取前 4 字节: {:?}", tmp);
    }

    // ===============================
    // Borsh 反序列化
    // ===============================
    {
        let borrow = account_data.borrow();
        let slice: &[u8] = &borrow[..];

        let counter: Counter = Counter::try_from_slice(&slice[..4]).unwrap();
        println!("Borsh 反序列化 Counter: {:?}", counter);
    }

    Ok(())
}
