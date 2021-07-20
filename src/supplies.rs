use std::fmt;
use std::error::Error;

pub struct Supplies {
    money: u32,
    spent_on_oxen: u32,
    spent_on_food: u32,
    spent_on_ammo: u32,
    spent_on_clothes: u32,
    spent_on_misc: u32
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
            spent_on_oxen: 0,
            spent_on_food: 0,
            spent_on_ammo: 0,
            spent_on_clothes: 0,
            spent_on_misc: 0
        }
    }

    pub fn money_left(&mut self) -> u32 {
        self.money
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

    pub fn buy_food(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.spent_on_food = spend;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_ammo(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.spent_on_ammo = spend;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_clothes(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.spent_on_clothes = spend;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_misc(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.spent_on_misc = spend;
        self.money -= spend;
        Ok(())    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buyerror_toolittle() {
        let buy_error = BuyError{ min_required: 0, max_allowed: 0, requested: 0, available: 0, reason: BuyErrorType::TooLittle };
        println!("{}", buy_error)
    }

    #[test]
    fn test_buyerror_toomuch() {
        let buy_error = BuyError{ min_required: 0, max_allowed: 0, requested: 0, available: 0, reason: BuyErrorType::TooMuch };
        println!("{}", buy_error)
    }

    #[test]
    fn test_buyerror_insufficient() {
        let buy_error = BuyError{ min_required: 0, max_allowed: 0, requested: 0, available: 0, reason: BuyErrorType::InsufficientFunds };
        println!("{}", buy_error)
    }

    #[test]
    fn test_supplies_constructor() {
        let supplies = Supplies::new();
    
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_money_left() {
        let mut supplies = Supplies::new();
        supplies.buy_oxen(200).unwrap();
        assert_eq!(500, supplies.money_left());
    }

    #[test]
    fn test_supplies_buy_oxen_success() {
        let mut supplies = Supplies::new();
        supplies.buy_oxen(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_too_little() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_oxen(0).unwrap_err().reason;

        assert_eq!(BuyErrorType::TooLittle, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_too_much() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_oxen(301).unwrap_err().reason;

        assert_eq!(BuyErrorType::TooMuch, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_oxen(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_oxen);
    }

    #[test]
    fn test_supplies_buy_food_success() {
        let mut supplies = Supplies::new();
        supplies.buy_food(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.spent_on_food);
    }

    #[test]
    fn test_supplies_buy_food_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_food(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_food);
    }

    #[test]
    fn test_supplies_buy_ammo_success() {
        let mut supplies = Supplies::new();
        supplies.buy_ammo(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.spent_on_ammo);
    }

    #[test]
    fn test_supplies_buy_ammo_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_ammo(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_ammo);
    }

    #[test]
    fn test_supplies_buy_clothes_success() {
        let mut supplies = Supplies::new();
        supplies.buy_clothes(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.spent_on_clothes);
    }

    #[test]
    fn test_supplies_buy_clothes_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_clothes(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_clothes);
    }

    #[test]
    fn test_supplies_buy_misc_success() {
        let mut supplies = Supplies::new();
        supplies.buy_misc(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.spent_on_misc);
    }

    #[test]
    fn test_supplies_buy_misc_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_misc(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.spent_on_misc);
    }
}