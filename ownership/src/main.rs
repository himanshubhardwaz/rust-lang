fn main() {
    /*
        Ownership rules:
        1. Each value in rust has a variable that's called its owner
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.
    */

    let x = 5;
    let y = x; // Here its copied, not moved

    let s1 = String::from("hello");
    let s2 = s1; // cannot use s1 after this, as it has been moved to s2

    // println!("{}", s1); This line will throw error

    let s3 = String::from("world");
    let s4 = s3.clone(); // Cam use s3 ahead

    /*
        References Rules:
        1. At any given time, we can habe either one mutable reference or
        any number of immutable references.
        2. References must always be valid.
    */
}
