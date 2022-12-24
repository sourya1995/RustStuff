fn count_fruit(apples: i32, bananas: i32) {
    println!("I've got {} apples and {} bananas", apples, bananas);
}

fn price_fruit(apples: i32, bananas: i32) -> i32 {
    apples * 8 + bananas * 12
}

fn main() {
    count_fruit(10, 5);
    let price = price_fruit(10, 5);
    println!("I can make {} cents", price);
}