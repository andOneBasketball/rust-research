use std::fmt::{self, Display};

// 定义一个通用的 Trait
trait Summary {
    fn summarize(&self) -> String;
}

// 给不同的类型实现这个 trait
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("《{}》 by {}", self.title, self.author)
    }
}

// 给 Article 实现 Display，才能用 print_and_notify
impl Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Article(title: {}, author: {})", self.title, self.author)
    }
}

struct Tweet {
    user: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{} says: {}", self.user, self.content)
    }
}

// 一个泛型函数，接收任何实现了 Summary trait 的类型
fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

// 如果需要多个约束（trait bound）
fn print_and_notify<T: Summary + Display>(item: &T) {
    println!("Item = {}", item); // 使用 Display
    println!("Notify = {}", item.summarize()); // 使用 Summary
}

// 通过 `impl Trait` 简化写法（语法糖）
fn notify_simple(item: &impl Summary) {
    println!("(simple) {}", item.summarize());
}

// 甚至可以返回实现了 trait 的类型（impl Trait）
fn get_tweet() -> impl Summary {
    Tweet {
        user: "Alice".to_string(),
        content: "Rust traits are powerful!".to_string(),
    }
}

fn main() {
    let article = Article {
        title: "Rust Trait Guide".to_string(),
        author: "Harold".to_string(),
    };

    let tweet = Tweet {
        user: "Bob".to_string(),
        content: "Learning Rust traits!".to_string(),
    };

    // 多个类型都可以调用同一个 trait 方法
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());

    // 泛型函数 + trait bound
    notify(&article);
    notify_simple(&tweet);

    // ✅ 使用 print_and_notify（因为 Article 同时实现了 Summary + Display）
    print_and_notify(&article);

    // ❌ Tweet 不能用 print_and_notify，因为它没有实现 Display
    // print_and_notify(&tweet);

    // 返回 impl Trait
    let new_tweet = get_tweet();
    println!("Generated: {}", new_tweet.summarize());
}
