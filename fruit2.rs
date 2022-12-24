struct Fruit {
    apples: i32,
    bananas: i32,
}

fn count_fruit(fruit: Fruit){
    println!("I've got {} apples and {} bananas", fruit.apples, fruit.bananas);
}

fn price_fruit(fruit: Fruit) -> i32{
    fruit.apples * 8 + fruit.bananas * 12
}

fn main(){
    let fruit = Fruit{
        apples: 10,
        bananas: 5,
    }

    count_fruit(fruit);
    let price = price_fruit(fruit);
    println!("I can make {} cents", price);
}