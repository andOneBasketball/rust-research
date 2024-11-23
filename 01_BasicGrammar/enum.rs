fn main() {
    enum Book {
        Papery {index: u32},
        Electronic {url: String},
    }
    
    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("url...")};
    
    match ebook {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }

    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {}, // 必须有 _ 匹配所有其他情况
    }
}