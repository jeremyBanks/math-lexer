pub fn main() {
    let mut chars = "test".chars();
    match chars.next() {
        Some(c) => {
            println!("breakpoint here and look at `c`");
            if c == 'c' {
                println!("c");
            }
        }
        None => {}
    }
}
