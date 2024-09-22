fn main() {
    println!("Hello, world!");
    let name = String::from("pandit");
    let length = get_str_len(name);

    println!("The length of String is, {},", length);
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
}
