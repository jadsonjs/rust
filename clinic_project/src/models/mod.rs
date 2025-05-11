// Then models/mod.rs is the entry point for the models module. It collects and organizes the submodules inside the models folder
// Declare that models contains three submodules: patient, doctor, and appointment.
pub mod patient;
pub mod doctor;
pub mod appointment;

// Re-exports the Patient and Doctor types from their submodules.
// Makes them accessible as if they were declared directly in models.
//
// use crate::models::Patient; // ✅ works because of `pub use`
// Instead of needing to write:   use crate::models::patient::Patient; // ✅ but more verbose
pub use patient::Patient;
pub use doctor::Doctor;
pub use appointment::Appointment;