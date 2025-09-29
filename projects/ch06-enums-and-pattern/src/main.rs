use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));

    assert_eq!(localhost_v4.is_ipv6(), false);
    assert_eq!(localhost_v4.is_ipv4(), true);
    println!("{}", localhost_v4);
    println!("{}", localhost_v6);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
            print!("Message is ");
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to x={}, y={}", x, y),
                //Message::Write(s) => println!("Write message: {}", s),
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to red={}, green={}, blue={}", r, g, b)
                }
                // 必须穷尽所有可能性，或者使用 _
                _ => println!("Other"),
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
