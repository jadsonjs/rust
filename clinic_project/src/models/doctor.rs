//  is a Rust attribute that tells the compiler to automatically generate code to support the Debug trait for your struct
// println!("{:?}", doctor); // ✅ only works if `#[derive(Debug)]` is present
#[derive(Debug)]  
                 
pub struct Doctor {
    pub name: String,
    pub patients: Vec<String>,
}

impl Doctor {
    pub fn new(name: &str) -> Self {
        Doctor {
            name: name.to_string(),
            patients: vec![],
        }
    }

    pub fn attend_patient(&mut self, patient_name: String) {
        self.patients.push(patient_name);
    }
}