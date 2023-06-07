use std::io::stdin;

struct Visitor{
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("failed to read line");
    your_name.trim().to_lowercase()
}
fn main() {
    let visitor_list = ["bert", "steve", "fred"];

    println!("Hello, what's your name?");
    let name = what_is_your_name();
    
    let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
            
        }
    }

    if allow_them_in {
        println!("Welcome to the treehouse, {}", name);
    } else {
        println!("Sorry, you're not on the list");
    }
}
