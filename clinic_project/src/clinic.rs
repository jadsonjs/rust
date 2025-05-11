
// This line brings the Doctor and Patient structs into scope from the models module 
// so they can be used directly in the clinic module without writing the full path every time.
use crate::models::{Doctor, Patient};

pub struct Clinic {
    doctors: Vec<Doctor>,
    patients: Vec<Patient>,
}

impl Clinic {
    pub fn new() -> Self {
        Clinic {
            doctors: vec![],
            patients: vec![],
        }
    }


    /// &mut self means: “This method needs mutable access to the instance (self) of the struct it's being called on.”
    /// Because we’re modifying the struct (adding a new doctor to its doctors vector), Rust requires a mutable reference to self.
    /// Rust just hides the &mut self parameter for you in method syntax
    /// 
    /// What you write	What Rust sees
    /// clinic.register_doctor("Dr. Alice")	  Clinic::register_doctor(&mut clinic, "Dr. Alice")
    /// 
    /// The & means you're borrowing data — you're not taking ownership of it.
    /// You're *** not *** making a new copy of the string.
    /// Instead, you're just saying, "I want to use this string, but I don't need to own it."
    /// pub fn register_doctor(&mut self, name: String) {
    ///    self.doctors.push(Doctor::new(name));
    ///}
    ///
    pub fn register_doctor(&mut self, name: &str) {
        self.doctors.push(Doctor::new(name));
    }

    pub fn register_patient(&mut self, name: &str) {
        self.patients.push(Patient::new(name));
    }

    pub fn assign_patient_to_doctor(&mut self, doctor_name: &str, patient_name: &str) {
        if let Some(doctor) = self.doctors.iter_mut().find(|d| d.name == doctor_name) {
            doctor.attend_patient(patient_name.to_string());
        }
    }

    pub fn print_summary(&self) {
        for doctor in &self.doctors {
            println!("{} attended {} patients:", doctor.name, doctor.patients.len());
            for p in &doctor.patients {
                println!("- {}", p);
            }
        }
    }
}