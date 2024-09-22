struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    let user = User {
        first_name: String::from("pandit"),
        last_name: String::from("Dhamdhere"),
        age: 26,
    };

    println!("{}", user.first_name);
}
