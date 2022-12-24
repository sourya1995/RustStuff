struct Fruit {
    apples: i32,
    bananas: i32
}

impl Fruit {
    fn price(self) -> i32 {
        self.apples * 8 + self.bananas * 12
    }
}

fn new_fruit() -> Fruit {
    Fruit { apples: 10, bananas: 5}
}

fn main() {
    let fruit = new_fruit();
    let price = fruit.price();
    println!("Price is {}", price);
}