use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Mileage {
    traveled: u32,
    rng: ThreadRng,
}

impl Mileage {
    pub fn new() -> Mileage {
        Mileage {
            traveled: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn traveled(&mut self) -> u32 {
        self.traveled
    }

    pub fn travel(&mut self, miles: u32) {
        self.traveled += miles;
    }

    pub fn turn(&mut self, miles: u32, oxen: u32) {
        let turn_miles = miles + ((oxen - 220) / (5 + self.rng.gen_range(0..10)));
        self.traveled += turn_miles;
    }

    pub fn reverse(&mut self, miles: u32) {
        self.traveled -= miles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mileage_constructor() {
        let mut mileage = Mileage::new();
        assert_eq!(0, mileage.traveled());
    }

    #[test]
    fn test_mileage_travel() {
        let mut mileage = Mileage::new();
        mileage.travel(200);
        assert_eq!(200, mileage.traveled());
    }

    #[test]
    fn test_mileage_reverse() {
        let mut mileage = Mileage::new();
        mileage.travel(200);
        mileage.reverse(100);
        assert_eq!(100, mileage.traveled());
    }

    #[test]
    fn test_mileage_turn() {
        let mut mileage = Mileage::new();
        mileage.turn(200, 300);
        assert!(mileage.traveled() > 200);
        assert!(mileage.traveled() < 300);
    }
}