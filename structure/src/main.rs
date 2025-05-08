



// Ownership Rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
fn main() {
    
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid


    {                                                   // s2 is not valid here, it’s not yet declared
        let mut s2 = String::from("hello");

        s2.push_str(", world!"); // push_str() appends a literal to a String

        println!("{s2}"); // This will print `hello, world!`
    }                                                   // this scope is now over, and s2 is no longer valid

    // When a variable goes out of scope, Rust calls a special function for us. 
    // This function is called drop, and it’s where the author of String can put the code to return the memory. 
    // Rust calls drop automatically at the closing curly bracket.

    // To ensure memory safety, after the line let s2 = s1;, 
    // Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. 
    // Check out what happens when you try to use s1 after s2 is created; it won’t work:
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); // error[E0382]: borrow of moved value: `s1`



    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.

    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here

        // println!("{}", s);   // error[E0382]: borrow of moved value: `s`
        
        let x = 5;                      // x comes into scope

        makes_copy(x);                  // because i32 implements the Copy trait,
                                        // x does NOT move into the function,
        println!("{}", x);              // so it's okay to use x afterward
    }


    // Return Values and Scope : Returning values can also transfer ownership. 
    {
        let s10 = gives_ownership();         // gives_ownership moves its return
        // value into s10

        let s20 = String::from("hello");     // s2 comes into scope

        let s30 = takes_and_gives_back(s20);  // s2 is moved into
    }


    // Here is how you would define and use a calculate_length function that has a reference 
    // to an object as a parameter instead of taking ownership of the value:

    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }



} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.



fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}



fn calculate_length(s: &String) -> usize {
    s.len()
}