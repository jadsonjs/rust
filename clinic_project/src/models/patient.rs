//  is a Rust attribute that tells the compiler to automatically generate code to support the Debug trait for your struct
// println!("{:?}", doctor); // ✅ only works if `#[derive(Debug)]` is present
#[derive(Debug, Clone)]
pub struct Patient {
    pub name: String,
}

impl Patient {
    pub fn new(name: &str) -> Self {
        Patient {
            name: name.to_string(),
        }
    }
}