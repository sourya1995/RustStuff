struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn new() -> Fruit {
        Fruit {
            apples: 10,
            bananas: 5,
        }
    }

    fn increase(mut self) -> Self {
        self.apples *= 2;
        self.bananas *= 3;
        self
    }

    fn print(&self) {
        println!("You have {} apples and {} bananas", self.apples, self.bananas);
    }
}