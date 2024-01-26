fn own_vector(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

fn own_integer(x: i32){
    x + 1;
}

fn own_string(s: String){
    println!("{}", s);
}

fn main(){
    let mut my_vector = vec![1, 2, 3, 4, 5];
    let my_integer = 10;
    let my_string = String::from ("Hello, world!");

    own_integer(my_integer); 
    println!("{}", my_integer); //this runs fine

    own_string(my_string);  //also runs fine
    //println!("{}", my_string); //this does not run because at line 22, my_string is owned by own_string and therefore cannot be owned by two different parts
    //of the code

    own_vector(my_vector); //runs fine
    println!("{:?}", my_vector); //this does not run because at line 26, my_vector is owned by own_vector and therefore cannot be owned by two different parts
    //of the code

}