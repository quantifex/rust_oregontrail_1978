use rand::Rng;
use rand::rngs::ThreadRng;
use chrono::{NaiveDate, Duration};
use std::result::Result;
use crate::*;

const HEALTH_ILLNESS: &str = "illness";

#[derive(PartialEq)]
#[derive(Debug)]
pub enum MealChoice {
    Poorly,
    Moderately,
    Well,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Illness {
    Mild,
    Bad,
    Serious,
}

pub struct Trip {
    miles_traveled: u32,
    current_date: NaiveDate,
    need_doctor: bool,
    health_issue: String,
    last_meal: MealChoice,
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
            need_doctor: false,
            health_issue: String::new(),
            last_meal: MealChoice::Well,
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

    /// Pick an illness randomly, using the last meal as a factor in the calculation
    ///     The severity of the illness is based on a random calculation and the last meal
    ///     Mild Illness: 100*RND() < 10 + 35*(last_meal - 1)
    ///     Bad Illness:  100*RND() < 100 - (40 / 4^(last_meal - 1))
    ///     Otherwise Serious Illness
    pub fn get_illness(&mut self, last_meal: MealChoice) -> Illness {
        let eat: u32 = match self.last_meal {
            MealChoice::Poorly => 1,
            MealChoice::Moderately => 2,
            MealChoice::Well => 3,
        };

        if self.rng.gen_range(0..100) < (10 + 35 * (eat - 1)) { // Mild illness
            return Illness::Mild;
        } else if self.rng.gen_range(0..100) < (100 - (40 / i32::pow(4, eat - 1))) { // Bad illness
            return Illness::Bad;
        } else {    // Serious illness
            return Illness::Serious;
        }
    }

    /// Update the health indicators to record an illness
    ///     Mild Illness:    Subtract 5 miles and use 2 miscellaneous supplies
    ///     Bad Illness:     Subtract 5 miles and use 5 miscellaneous supplies
    ///     Serious Illness: Subtract 10 miles and mark need_doctor
    pub fn got_sick(&mut self, illness: Illness, supplies: &mut Supplies) -> Result<(), UseError> {
        self.health_issue.clear();
        self.health_issue.push_str(HEALTH_ILLNESS);

        match illness {
            Illness::Mild => {
                self.miles_traveled -= 5;
                supplies.use_misc(2)?;
            }, Illness:: Bad => {
                self.miles_traveled -= 5;
                supplies.use_misc(5)?;
            }, Illness::Serious => {
                self.miles_traveled -= 10;
                self.need_doctor = true;
            },
        }
        Ok(())
    }

    /// Retrieve the current need_doctor flag
    pub fn need_doctor(&mut self) -> bool {
        self.need_doctor
    }

    /// Retrieve the string which describes your current health issue
    pub fn health_issue(&mut self) -> String {
        if self.health_issue.is_empty() {
            String::new()
        } else {
            format!("your {}", self.health_issue.as_str())
        }
    }

    /// Visit the doctor to heal any injuries or illness, costs $20
    pub fn visit_doctor(&mut self, supplies: &mut Supplies) -> Result<(), BuyError> {
        supplies.spend(20)?;
        self.need_doctor = false;
        Ok(())
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
        assert_eq!(false, trip.need_doctor());
        assert_eq!("", trip.health_issue());
        assert_eq!(MealChoice::Well, trip.last_meal);
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
        assert!(trip.miles_traveled() > 190);
        assert!(trip.miles_traveled() < 220);
    }

    #[test]
    fn test_illness_eat_poor() {
        let mut trip = Trip::new();
        let mut mild = 0;
        let mut bad = 0;
        let mut serious = 0;
        for _ in 0..1000 {
            match trip.get_illness(MealChoice::Poorly) {
                Illness::Mild => mild += 1,
                Illness::Bad => bad += 1,
                Illness::Serious => serious += 1,
            }
        }
        assert_eq!(1000, mild + bad + serious);
        assert!(mild > 50);
        assert!(bad > 10);
        assert!(serious >= 1);
    }

    #[test]
    fn test_illness_eat_moderately() {
        let mut trip = Trip::new();
        let mut mild = 0;
        let mut bad = 0;
        let mut serious = 0;
        for _ in 0..1000 {
            match trip.get_illness(MealChoice::Moderately) {
                Illness::Mild => mild += 1,
                Illness::Bad => bad += 1,
                Illness::Serious => serious += 1,
            }
        }
        assert_eq!(1000, mild + bad + serious);
        assert!(mild > 50);
        assert!(bad > 10);
    }

    #[test]
    fn test_illness_eat_well() {
        let mut trip = Trip::new();
        let mut mild = 0;
        let mut bad = 0;
        let mut serious = 0;
        for _ in 0..1000 {
            match trip.get_illness(MealChoice::Well) {
                Illness::Mild => mild += 1,
                Illness::Bad => bad += 1,
                Illness::Serious => serious += 1,
            }
        }
        assert_eq!(1000, mild + bad + serious);
        assert!(mild > 50);
        assert!(bad > 10);
    }

    #[test]
    fn test_got_sick_mild() {
        let mut trip = Trip::new();
        trip.travel(200);
        let mut supplies = Supplies::new();
        supplies.buy_misc(200);
        trip.got_sick(Illness::Mild, &mut supplies);
        assert_eq!("your illness", trip.health_issue());
        assert_eq!(false, trip.need_doctor());
        assert_eq!(198, supplies.misc_left());
        assert_eq!(195, trip.miles_traveled());
    }

    #[test]
    fn test_got_sick_bad() {
        let mut trip = Trip::new();
        trip.travel(200);
        let mut supplies = Supplies::new();
        supplies.buy_misc(200);
        trip.got_sick(Illness::Bad, &mut supplies);
        assert_eq!("your illness", trip.health_issue());
        assert_eq!(false, trip.need_doctor());
        assert_eq!(195, supplies.misc_left());
        assert_eq!(195, trip.miles_traveled());
    }

    #[test]
    fn test_got_sick_serious() {
        let mut trip = Trip::new();
        trip.travel(200);
        let mut supplies = Supplies::new();
        trip.got_sick(Illness::Serious, &mut supplies);
        assert_eq!(true, trip.need_doctor());
        assert_eq!("your illness", trip.health_issue());
        assert_eq!(190, trip.miles_traveled());
    }

    #[test]
    fn test_visit_doctor() {
        let mut trip = Trip::new();
        trip.travel(200);
        let mut supplies = Supplies::new();
        trip.got_sick(Illness::Serious, &mut supplies);
        assert_eq!(true, trip.need_doctor());
        trip.visit_doctor(&mut supplies);
        assert_eq!(680, supplies.money_left());
        assert_eq!(false, trip.need_doctor());
    }
}