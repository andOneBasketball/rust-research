// 结构体中存在引用变量则必须标注生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    //let first_sentence = novel.split('.').next().unwrap();
    let first_sentence = novel.split('.').nth(0).unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence is: {}, novel: {}", i.part, novel)
}
