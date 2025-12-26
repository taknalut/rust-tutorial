fn main() {
    println!("Hello ch02!");

    /*
        Statically Typed
    */
    let a: i32 = 100;

    let b = -200;

    println!("Integer with explicit type: {}", a);
    println!("Integer with inferred type: {}", b);
    println!("-----------------------------------");

    /*
        Scala Types: Floating-Points
    */
    let temp: f64 = 3.1344555;
    let pressure: f32 = 10.3454545465656775;

    println!("Floating-point number: {}", temp);
    println!("Floating-point number: {}", pressure);
    println!("-----------------------------------");

    /*
        Signed and Unsigned Integers
    */
    let age: u8 = 10;

    println!("Unsigned integer: {}", age);
    println!("-----------------------------------");

    /*
        Mutability and Immutability
    */
    let immutable_var = 10;
    println!("Immutable varible value: {}", immutable_var);

    let mut mutable_var = 20;
    mutable_var = 200;
    println!("Mutable varible value: {}", mutable_var);
    println!("-----------------------------------");

    /*
        Constants
    */
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
    println!("-----------------------------------");
}
