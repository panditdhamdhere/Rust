use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println!("Hello Pandit!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your number");

        let mut guess = String::new();
    
        io::stdin().Stdin
        .read_line(buf: &mut guess) Result<usize, Error>
        .expect("Failed to read user input");
    
        let guess: u32 = match guess.trim().parse() {
        Ok(num : u32) => num,
        Err(_) => continue,

        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
            break}
        }
   

    };
}
