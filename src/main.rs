use std::env;
use std::fs;

pub fn main() {
    let input: Rc<String> = Rc::new("test".to_string());
    let mut chars = input.chars();

    loop {
        match chars.next() {
            Some(c) => {
                println!("What is the value of c, here?");
                if c == '\n' {
                    println!("yes");
                }
            }
            None => {}
        }
    }
}
