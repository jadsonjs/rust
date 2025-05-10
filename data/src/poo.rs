

struct User {
    name: String,
    age: u8,
}

impl User {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}


fn main(){
    let user = User { name: String::from("Alice"), age: 30 };
    user.greet();
}

fn make_animal_speak(animal: &dyn Animal) {
    animal.speak();
}