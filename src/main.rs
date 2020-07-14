fn main() { 
    use std::env; 
    use std::fs; 

    let filename = "../quotes.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("{}", contents); 
}
