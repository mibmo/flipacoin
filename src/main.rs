fn main() {
    let result = match rand::random() {
        false => "Heads",
        true => "Tails",
    };

    println!("{}", result);
}
