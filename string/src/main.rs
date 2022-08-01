use std::fmt::format;

fn main() {
    // let string_append = String::from("hello");
    // let string_rust = String::from(" rust");
    // let result = string_append + &string_rust;
    // let mut result = result + "!!!";
    // result += "。";
    // println!("连接字符串-> {}", result);

    // let s1 = "hello";
    // let s2 = String::from("rust");
    // let s = format!("{} {}", s1, s2);
    // println!("{}", s);
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
