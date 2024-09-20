use text_io::read;
use capitalize::Capitalize;

fn main() {
   
}

fn get_name(){
    println!("Hello, what is your name?");
    let name: String = read!();
    println!("Great! Nice to meet you {}", name.capitalize());
}
