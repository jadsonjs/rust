

mod models; // There’s a module named models, and you’ll find its code in a file called models/mod.rs
mod clinic; // “There’s a module named clinic, defined in clinic.rs.”

use clinic::Clinic;  // This brings the Clinic struct into scope from the clinic module 

use models::{Doctor, Patient, Appointment};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};


fn main() {
    let mut clinic = Clinic::new();

    clinic.register_doctor("Dr. Alice");
    clinic.register_patient("John Doe");
    clinic.register_patient("Jane Roe");

    clinic.assign_patient_to_doctor("Dr. Alice", "John Doe");
    clinic.assign_patient_to_doctor("Dr. Alice", "Jane Roe");

    clinic.print_summary();


    let patient = Patient::new("Alice");
    let doctor = Doctor::new("Dr. Bob");

    let date_time: NaiveDateTime = NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(2025, 5, 12).unwrap(), // monday
        chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
    );

    // Appointment::new(...) is a constructor method that returns an Option<Appointment>.
    //
    // Option is an enum that has two variants:
    //
    // Some(value) → contains a valid result.
    // None → means something failed (e.g., invalid date/time).
    //
    match Appointment::new(patient, doctor, date_time) {
        Some(appointment) => println!("✅ Appointment scheduled: {:?}", appointment),
        None => println!("❌ Invalid appointment time or day!"),
    }
    
}
