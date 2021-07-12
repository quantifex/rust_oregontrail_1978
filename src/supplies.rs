use std::fmt;
use std::error::Error;

pub struct Supplies {
    money: u32,
    spent_on_oxen: u32,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum BuyErrorType {
    TooLittle,
    TooMuch,
    InsufficientFunds,
}

#[derive(Debug)]
pub struct BuyError {
    requested: u32,
    available: u32,
    min_required: u32,
    max_allowed: u32,
    reason: BuyErrorType,
}

impl Error for BuyError {}

impl fmt::Display for BuyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.reason {
            BuyErrorType::TooLittle =>
                write!(f, "\tYou must spend at least ${}", self.min_required),
            BuyErrorType::TooMuch =>
                write!(f, "\tYou may not spend more than ${}", self.max_allowed),
            BuyErrorType::InsufficientFunds =>
                write!(f, "\tUnable to spend ${}, you only have ${} available.", self.requested, self.available),
        }
    }
}

impl Supplies {
    pub fn new() -> Supplies {
        Supplies {
            money: 700,
            spent_on_oxen: 0
        }
    }

    pub fn buy_oxen(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 200, max_allowed: 300, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        } else if spend < 200 {
            return Err(BuyError{ min_required: 200, max_allowed: 300, requested: spend, available: self.money, reason: BuyErrorType::TooLittle });
        } else if spend > 300 {
            return Err(BuyError{ min_required: 200, max_allowed: 300, requested: spend, available: self.money, reason: BuyErrorType::TooMuch });
        }
        self.spent_on_oxen = spend;
        self.money -= spend;
        Ok(())    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supplies_constructor() {
        let supplies = Supplies::new();
    
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_success() {
        let mut supplies = Supplies::new();
        match supplies.buy_oxen(200) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        }
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_too_little() {
        let mut supplies = Supplies::new();
        match supplies.buy_oxen(0) {
            Ok(_) => panic!("failure expected due to too little spent"),
            Err(e) => {
                match e.reason {
                    BuyErrorType::TooLittle => {},
                    _ => panic!("expected BuyErrorType::TooLittle"),
                }
            },
        }
    
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_too_much() {
        let mut supplies = Supplies::new();
        match supplies.buy_oxen(301) {
            Ok(_) => panic!("failure expected due to too much spent"),
            Err(e) => {
                match e.reason {
                    BuyErrorType::TooMuch => {},
                    _ => panic!("expected BuyErrorType::TooMuch"),
                }
            },
        }
    
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_insufficient() {
        let mut supplies = Supplies::new();
        match supplies.buy_oxen(1000) {
            Ok(_) => panic!("failure expected due to insufficient"),
            Err(e) => {
                match e.reason {
                    BuyErrorType::InsufficientFunds => {},
                    _ => panic!("expected BuyErrorType::InsufficientFunds"),
                }
            },
        }
    
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }
}