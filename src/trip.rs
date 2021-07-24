use rand::Rng;
use rand::rngs::ThreadRng;
use chrono::{NaiveDate, Duration};

pub struct Trip {
    miles_traveled: u32,
    current_date: NaiveDate,
    rng: ThreadRng,
}

impl Trip {
    pub fn new() -> Trip {
        Trip {
            miles_traveled: 0,
            current_date: NaiveDate::from_ymd(1847, 03, 29),
            rng: rand::thread_rng(),
        }
    }

    pub fn miles_traveled(&mut self) -> u32 {
        self.miles_traveled
    }

    pub fn current_date(&mut self) -> NaiveDate {
        self.current_date
    }

    pub fn travel(&mut self, miles: u32) {
        self.miles_traveled += miles;
    }

    pub fn turn(&mut self, miles: u32, oxen: u32) {
        let turn_miles = miles + ((oxen - 220) / (5 + self.rng.gen_range(0..10)));
        self.miles_traveled += turn_miles;
        self.current_date += Duration::days(14);
    }

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
        trip.turn(200, 300);
        assert!(trip.miles_traveled() > 200);
        assert!(trip.miles_traveled() < 300);
    }
}