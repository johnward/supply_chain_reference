use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let mut text_input: [char; 2] = ['a', 'b'];
let b: [char; 3] = ['a', 'b', 'c'];
text_input.copy_from_slice(&b);

    
    println!("{:?}", args);
    
}