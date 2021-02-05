use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!!");

    let input: u32 = input.trim().parse().expect("Please type a number!");
    
    let mut counter: u32 = 1;

    while counter <= input {
        println!("{} Abracadabra", counter);

        counter += 1;
    }
}
