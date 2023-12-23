mod character;
mod dice;

fn main() {
    let character = character::Character::generate("Foo Bar".to_string());
    println!("{:?}", character);
}
