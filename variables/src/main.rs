fn main() {
    let x = 5;
    println!("the value of x is: {}", x);
    let x = x + 1;
    println!("shadowed value of x is: {}", x);
    {
        let x = x * 4;
        println!("inner scope x is: {}", x);
    }
    println!("outer scope x is: {}", x);
}
