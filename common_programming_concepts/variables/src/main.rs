fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const cannot be mutated, must always be type annoted, cannot be set as a return value of a function
    const COUNT: u32 = 100_000;

    // This way mutablity is preserved
    let y = 2;
    println!("The value of x is: {}", y);
    let y = 3;
    println!("The value of x is: {}", y);
}
