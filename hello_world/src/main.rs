
// hello world in rust
fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    conditions();

    loops();
}


fn another_function(value: i32, unit_label: char) {
    println!("The value of x is: {value}{unit_label}");
}

// We don’t name return values, but we must declare their type after an arrow (->).
fn five() -> i32 {
    
    let y = 6;  // Statements are instructions that perform some action and do not return a value.

    // Expressions evaluate to a resultant value. Let’s look at some examples.

    let y = {
        // This expression is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. 
        let x = 3;

        //  Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
        x + 1
    };
    
    // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:
    5 // expression without ";"
}



fn conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}



fn loops() {
    let mut count = 0;

    'counting_up: loop {  // You can optionally specify a loop label on a loop that you can then use with break or continue
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }


    // As a more concise alternative, you can use a for loop and execute some code for each item in a collection.
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }


    // Here’s what the countdown would look like using a for loop and another method we’ve not yet talked about, rev, to reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}