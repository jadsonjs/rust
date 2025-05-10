fn main() {

    // A closure is an anonymous function you can store in a variable, pass to functions, or return from functions. 
    // Closures capture variables from the environment
    // Are anonymous and often inline.  
    // Rust uses the |a, b| syntax for closure parameters — and that’s different from regular functions (fn foo(a: i32, b: i32)) — on purpose.
    // Closures in Rust were designed to look and feel like math notation or lambda calculus (
    //
    // Is similar to:
    // fn add(a: i32, b: i32) -> i32 {
    //   a + b
    // }

    let add = |a, b| a + b;

    println!("{}", add(2, 3));

    // interators in rust 
    let numbers = vec![1, 2, 3];
    let sum: i32 = numbers.iter().sum(); // 6

    let even_squared: Vec<_> = numbers
        .iter()
            .filter(|&&x| x % 2 == 0) // filter even numbers
                .map(|x| x * x) // square them
                    .collect(); //  Converts the iterator into a collection. 

    println!("{}", even_squared[0]); // prints 4

}
