

struct Person {
    name: String,
    age: u8,
}


struct Rectangle {
    width: u32,
    height: u32,
}

// structs themselves only define the data — they do not include methods directly. To add behavior (like methods) to a struct, you must use a separate impl block.
impl Rectangle {

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}


enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

fn main() {
    
    // We create a new Person using let person = Person { ... }.
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);


    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area: {}", rect.area());
    println!("Is square? {}", rect.is_square());

    let rect2 = Rectangle::new(50, 50);

    println!("Area 2: {}", rect2.area());
    println!("Is square 2? {}", rect2.is_square());

    // We can create instances of each of the two variants of IpAddrKind like this
    let four = IpAddrKind::V4;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home2: IpAddr2 = IpAddr2::V4(String::from("127.0.0.1"));

}
