use rand::Rng;
use rand::rngs::ThreadRng;
use chrono::{NaiveDate, Duration};
use crate::*;

const HEALTH_ILLNESS: &str = "illness";

pub struct Trip {
    miles_traveled: u32,
    current_date: NaiveDate,
    need_healing: bool,
    health_issue: String,
    rng: ThreadRng,
}

impl Trip {
    /// Constructor
    ///     Miles Traveled will be initialized to 0
    ///     Current Date will be initialized to March 29, 1847
    pub fn new() -> Trip {
        Trip {
            miles_traveled: 0,
            current_date: NaiveDate::from_ymd(1847, 03, 29),
            need_healing: false,
            health_issue: String::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Retrieve the # of miles traveled so far
    pub fn miles_traveled(&mut self) -> u32 {
        self.miles_traveled
    }

    /// Retrieve the current date of travel
    pub fn current_date(&mut self) -> NaiveDate {
        self.current_date
    }

    /// Travel the Oregon Trail by a specific number of miles
    pub fn travel(&mut self, miles: u32) {
        self.miles_traveled += miles;
    }

    pub fn got_sick(&mut self) {
        self.need_healing = true;
        self.health_issue.clear();
        self.health_issue.push_str(HEALTH_ILLNESS);
    }

    /// Retrieve the current need_healing flag
    pub fn need_healing(&mut self) -> bool {
        self.need_healing
    }

    /// Retrieve the string which describes your current health issue
    pub fn health_issue(&mut self) -> String {
        if self.health_issue.is_empty() {
            String::new()
        } else {
            format!("your {}", self.health_issue.as_str())
        }
    }

    pub fn visit_doctor(&mut self, supplies: &mut supplies::Supplies) {
        supplies.spend(20);
        self.need_healing = false;
    }

    /// Completes a portion of the Oregon Trail Trip, a "turn" of the game
    ///     This includes traveling X miles and the passing of 2 weeks
    ///     Where X miles equals a distance calculated as (BASIC code):
    ///         200+((oxen)-220)/5+10*RND()
    ///     Based on this calculation, the amount spent on oxen has the biggest impact:
    ///         If Oxen = 200, Mileage: 197 <==> 199
    ///         If Oxen = 300, Mileage: 213 <==> 206
    pub fn turn(&mut self, oxen: u32) {
        let turn_miles = 200 + ((oxen - 220) / (5 + self.rng.gen_range(0..10)));
        self.miles_traveled += turn_miles;
        self.current_date += Duration::days(14);
    }

    /// Something bad happened, we need to go backwards or delay the trip by a specific number of miles
    pub fn reverse(&mut self, miles: u32) {
        self.miles_traveled -= miles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trip_constructor() {
        let mut trip = Trip::new();
        assert_eq!(0, trip.miles_traveled());
        assert_eq!(NaiveDate::from_ymd(1847, 03, 29), trip.current_date());
        assert_eq!(false, trip.need_healing());
        assert_eq!("", trip.health_issue());
    }

    #[test]
    fn test_trip_travel() {
        let mut trip = Trip::new();
        trip.travel(200);
        assert_eq!(200, trip.miles_traveled());
    }

    #[test]
    fn test_trip_reverse() {
        let mut trip = Trip::new();
        trip.travel(200);
        trip.reverse(100);
        assert_eq!(100, trip.miles_traveled());
    }

    #[test]
    fn test_trip_turn() {
        let mut trip = Trip::new();
        trip.turn(300);
        assert!(trip.miles_traveled() > 200);
        assert!(trip.miles_traveled() < 300);
    }

    #[test]
    fn test_visit_doctor() {
        let mut trip = Trip::new();
        trip.got_sick();
        let mut supplies = Supplies::new();
        trip.visit_doctor(&mut supplies);
        assert_eq!(680, supplies.money_left());
        assert_eq!(false, trip.need_healing());
    }
}