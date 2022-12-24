fn main() {
    let x: i32 = 5; //immutable, direct value
    let mut y: i32 = 6; //mutable, direct value
    let z : &mut i32 = &mut y; //mutable, holds mutable ref
    *z -= 1; //reduce by 1
    assert_eq!(x, y);
    println!("Success");
}