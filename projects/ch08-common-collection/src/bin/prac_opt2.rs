/*
将字符串转换为 pig latin。也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 ay，所以 first 会变成 irst-fay。元音字母开头的单词则在结尾增加 hay（apple 会变成 apple-hay）。请注意 UTF-8 编码的细节！
 */

fn pig_latin(s: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut result = String::new();
    for word in s.split_whitespace() {
        if let Some(first) = word.chars().next() {
            if vowels.contains(first) {
                result += &format!("{}-hay", word);
            } else {
                let rest: String = word.chars().skip(1).collect();
                result += &(rest + "-");
                result += &first.to_string();
                result += "ay";
            }
        }
        result += " ";
    }
    return result.trim().to_string();
}

fn main() {
    let mut s = "hello world";
    println!("{} -> {}", s, pig_latin(s));
    s = "apple banana cherry";
    println!("{} -> {}", s, pig_latin(s))
}
