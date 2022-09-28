fn main() {
    {
        let str = String::from("hello");
        let len = calculate_length(&str); // &str 创建一个对于str 的引用，创建一个引用的行为称为 借用（borrowing）。
        println!("str's length is {}", len);
    }

    {
        // 可变引用
        let mut str = String::from("hello");
        change_string(&mut str);
        println!("[change_string] str: {}", str);
    }
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", rust");
}
