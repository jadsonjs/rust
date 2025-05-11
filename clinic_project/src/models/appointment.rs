use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Weekday};


// we need to bring the Datelike trait into scope from the chrono crate. That trait provides the .weekday() method for NaiveDate.
//
// A trait in Rust is a collection of methods defined for types that can be implemented by those types. It is similar to interfaces in other languages like Java or C#, 
// but with some important differences. 
// Traits allow you to define shared behavior across different types and ensure that types conform to certain functionality
//
// Traits, like Datelike, are collections of methods that are implemented for types, and they need to be imported separately.
use chrono::Datelike; // for the weekday method
use crate::models::{Doctor, Patient};

#[derive(Debug)]
pub struct Appointment {
    patient: Patient,
    doctor: Doctor,
    date_time: NaiveDateTime,
}

impl Appointment {

    // Constructor method to create a new appointment
    // Option is an enum that has two variants:
    //
    // Some(value) → contains a valid result.
    // None → means something failed (e.g., invalid date/time).
    //
    pub fn new(patient: Patient, doctor: Doctor, date_time: NaiveDateTime) -> Option<Self> {
        // Check if the appointment is during working hours and on a weekday
        if Self::is_valid(&date_time) {
            Some(Self { patient, doctor, date_time } )
        } else {
            None
        }
    }

    // Validate the appointment time (Monday-Friday, 8:00 AM - 5:00 PM)
    pub fn is_valid(date_time: &NaiveDateTime) -> bool {
        
        let weekday = date_time.weekday();
        let time = date_time.time();

        let start_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap();  // Start of working hours
        let end_time = NaiveTime::from_hms_opt(17, 0, 0).unwrap();   // End of working hours

        // returns true if the appointment is on a weekday and within working hours
        weekday != Weekday::Sat
            && weekday != Weekday::Sun
            && time >= start_time
            && time <= end_time
    }
}