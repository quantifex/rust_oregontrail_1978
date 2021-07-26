use std::io::{Cursor, Write, BufRead, Seek, SeekFrom};
use std::fmt;
use std::error::Error;
use std::result::Result;
use crate::*;

const ASK_FOOD_SPEND: &str = "How much do you want to spend on \x1B[36mFood\x1B[0m? ";
const ASK_AMMO_SPEND: &str = "How much do you want to spend on \x1B[35mAmmunition\x1B[0m? ";
const ASK_CLOTHES_SPEND: &str = "How much do you want to spend on \x1B[34mClothing\x1B[0m? ";
const ASK_MISC_SPEND: &str = "How much do you want to spend on \x1B[33mMiscellaneous supplies\x1B[0m? ";
macro_rules! SUPPLIES_BUY_LEFT { () => { "After all your purchases, you now have $\x1B[32m{}\x1B[0m left\n" }; }

pub struct Supplies {
    money: u32,
    oxen: u32,
    food: u32,
    ammo: u32,
    clothes: u32,
    misc: u32,
    cost_premium: f32,
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

impl fmt::Display for Supplies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t{}\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\t{}\n",
            "\x1B[36mFood\x1B[0m", "\x1B[35mAmmo\x1B[0m", "\x1B[34mClothes\x1B[0m",
            "\x1B[33mMisc\x1B[0m", "\x1B[32mMoney\x1B[0m",
            self.food, self.ammo, self.clothes, self.misc, self.money)
    }
}

impl Supplies {
    pub fn new() -> Supplies {
        Supplies {
            money: 700,
            oxen: 0,
            food: 0,
            ammo: 0,
            clothes: 0,
            misc: 0,
            cost_premium: 0.0,
        }
    }

    pub fn set_premium(&mut self, premium: f32) {
        self.cost_premium = premium;
    }

    pub fn money_left(&mut self) -> u32 {
        self.money
    }

    pub fn spend(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 20, max_allowed: 20, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.money -= spend;
        Ok(())
    }

    pub fn oxen_left(&mut self) -> u32 {
        self.oxen
    }

    pub fn food_left(&mut self) -> u32 {
        self.food
    }

    pub fn ammo_left(&mut self) -> u32 {
        self.ammo
    }

    pub fn clothes_left(&mut self) -> u32 {
        self.clothes
    }

    pub fn misc_left(&mut self) -> u32 {
        self.misc
    }

    pub fn buy_oxen(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 200, max_allowed: 300, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        } else if spend < 200 {
            return Err(BuyError{ min_required: 200, max_allowed: 300, requested: spend, available: self.money, reason: BuyErrorType::TooLittle });
        } else if spend > 300 {
            return Err(BuyError{ min_required: 200, max_allowed: 300, requested: spend, available: self.money, reason: BuyErrorType::TooMuch });
        }
        self.oxen += spend;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_food(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.food += (spend as f32 * (1.0 - self.cost_premium)) as u32;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_ammo(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.ammo += (spend as f32 * (1.0 - self.cost_premium)) as u32;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_clothes(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.clothes += (spend as f32 * (1.0 - self.cost_premium)) as u32;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy_misc(&mut self, spend: u32) -> Result<(), BuyError> {
        if spend > self.money {
            return Err(BuyError{ min_required: 0, max_allowed: self.money, requested: spend, available: self.money, reason: BuyErrorType::InsufficientFunds });
        }
        self.misc += (spend as f32 * (1.0 - self.cost_premium)) as u32;
        self.money -= spend;
        Ok(())    
    }

    pub fn buy<W: Write, R: BufRead>(&mut self, out: &mut W, input: &mut R) {
        ask_ok!(self.buy_food(ask!(ASK_FOOD_SPEND, out, input)));
        ask_ok!(self.buy_ammo(ask!(ASK_AMMO_SPEND, out, input)));
        ask_ok!(self.buy_clothes(ask!(ASK_CLOTHES_SPEND, out, input)));
        ask_ok!(self.buy_misc(ask!(ASK_MISC_SPEND, out, input)));
        out.write(format!(SUPPLIES_BUY_LEFT!(), self.money_left()).as_bytes()).unwrap();
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
        assert_eq!(0, supplies.oxen);
    }

    #[test]
    fn test_supplies_money_left() {
        let mut supplies = Supplies::new();
        supplies.buy_oxen(200).unwrap();
        assert_eq!(500, supplies.money_left());
    }

    #[test]
    fn test_supplies_oxen_left() {
        let mut supplies = Supplies::new();
        supplies.buy_oxen(250).unwrap();
        assert_eq!(250, supplies.oxen_left());
    }

    #[test]
    fn test_supplies_food_left() {
        let mut supplies = Supplies::new();
        supplies.buy_food(200).unwrap();
        assert_eq!(200, supplies.food_left());
    }

    #[test]
    fn test_supplies_ammo_left() {
        let mut supplies = Supplies::new();
        supplies.buy_ammo(150).unwrap();
        assert_eq!(150, supplies.ammo_left());
    }

    #[test]
    fn test_supplies_clothes_left() {
        let mut supplies = Supplies::new();
        supplies.buy_clothes(200).unwrap();
        assert_eq!(200, supplies.clothes_left());
    }

    #[test]
    fn test_supplies_misc_left() {
        let mut supplies = Supplies::new();
        supplies.buy_misc(200).unwrap();
        assert_eq!(200, supplies.misc_left());
    }

    #[test]
    fn test_supplies_buy_oxen_success() {
        let mut supplies = Supplies::new();
        supplies.buy_oxen(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_too_little() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_oxen(0).unwrap_err().reason;

        assert_eq!(BuyErrorType::TooLittle, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_too_much() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_oxen(301).unwrap_err().reason;

        assert_eq!(BuyErrorType::TooMuch, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.oxen);
    }

    #[test]
    fn test_supplies_buy_oxen_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_oxen(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.oxen);
    }

    #[test]
    fn test_supplies_buy_food_success() {
        let mut supplies = Supplies::new();
        supplies.buy_food(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.food);
    }

    #[test]
    fn test_supplies_buy_food_twice() {
        let mut supplies = Supplies::new();
        supplies.buy_food(200).unwrap();
        supplies.buy_food(200).unwrap();
    
        assert_eq!(300, supplies.money);
        assert_eq!(400, supplies.food);
    }

    #[test]
    fn test_supplies_buy_food_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_food(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.food);
    }

    #[test]
    fn test_supplies_buy_ammo_success() {
        let mut supplies = Supplies::new();
        supplies.buy_ammo(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.ammo);
    }

    #[test]
    fn test_supplies_buy_ammo_twice() {
        let mut supplies = Supplies::new();
        supplies.buy_ammo(200).unwrap();
        supplies.buy_ammo(200).unwrap();
    
        assert_eq!(300, supplies.money);
        assert_eq!(400, supplies.ammo);
    }

    #[test]
    fn test_supplies_buy_ammo_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_ammo(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.ammo);
    }

    #[test]
    fn test_supplies_buy_clothes_success() {
        let mut supplies = Supplies::new();
        supplies.buy_clothes(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.clothes);
    }

    #[test]
    fn test_supplies_buy_clothes_twice() {
        let mut supplies = Supplies::new();
        supplies.buy_clothes(200).unwrap();
        supplies.buy_clothes(200).unwrap();
    
        assert_eq!(300, supplies.money);
        assert_eq!(400, supplies.clothes);
    }

    #[test]
    fn test_supplies_buy_clothes_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_clothes(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.clothes);
    }

    #[test]
    fn test_supplies_buy_misc_success() {
        let mut supplies = Supplies::new();
        supplies.buy_misc(200).unwrap();
    
        assert_eq!(500, supplies.money);
        assert_eq!(200, supplies.misc);
    }

    #[test]
    fn test_supplies_buy_misc_twice() {
        let mut supplies = Supplies::new();
        supplies.buy_misc(200).unwrap();
        supplies.buy_misc(200).unwrap();
    
        assert_eq!(300, supplies.money);
        assert_eq!(400, supplies.misc);
    }

    #[test]
    fn test_supplies_buy_misc_insufficient() {
        let mut supplies = Supplies::new();
        let reason = supplies.buy_misc(1000).unwrap_err().reason;

        assert_eq!(BuyErrorType::InsufficientFunds, reason);
        assert_eq!(700, supplies.money);
        assert_eq!(0, supplies.misc);
    }   

    #[test]
    fn test_supplies_set_premium_food() {
        let mut supplies = Supplies::new();
        supplies.set_premium(0.3);
        supplies.buy_food(200).unwrap();
        assert_eq!(140, supplies.food_left());
        assert_eq!(500, supplies.money_left());
    }

    #[test]
    fn test_supplies_set_premium_ammo() {
        let mut supplies = Supplies::new();
        supplies.set_premium(0.3);
        supplies.buy_ammo(200).unwrap();
        assert_eq!(140, supplies.ammo_left());
        assert_eq!(500, supplies.money_left());
    }

    #[test]
    fn test_supplies_set_premium_clothes() {
        let mut supplies = Supplies::new();
        supplies.set_premium(0.3);
        supplies.buy_clothes(200).unwrap();
        assert_eq!(140, supplies.clothes_left());
        assert_eq!(500, supplies.money_left());
    }

    #[test]
    fn test_supplies_set_premium_misc() {
        let mut supplies = Supplies::new();
        supplies.set_premium(0.3);
        supplies.buy_misc(200).unwrap();
        assert_eq!(140, supplies.misc_left());
        assert_eq!(500, supplies.money_left());
    }

    #[test]
    fn test_supplies_display() {
        let mut supplies = Supplies::new();
        supplies.buy_oxen(200).unwrap();
        supplies.buy_food(10).unwrap();
        supplies.buy_ammo(20).unwrap();
        supplies.buy_clothes(30).unwrap();
        supplies.buy_misc(40).unwrap();

        let supplies_display = format!("{}", &mut supplies);
        assert_eq!("\t\x1B[36mFood\x1B[0m\t\x1B[35mAmmo\x1B[0m\t\x1B[34mClothes\x1B[0m\t\x1B[33mMisc\x1B[0m\t\x1B[32mMoney\x1B[0m\n\t10\t20\t30\t40\t400\n", supplies_display);
    }

    #[test]
    fn test_buy() {
        let mut supplies = Supplies::new();
        let mut cout = Cursor::new(Vec::new());
        let mut cin = Cursor::new(Vec::new());
        cin.write(b"50\r\n75\r\n100\r\n150\r\n").unwrap();
        cin.seek(SeekFrom::Start(0)).unwrap();
        supplies.buy(&mut cout, &mut cin);
        assert_eq!(50, supplies.food_left());
        assert_eq!(75, supplies.ammo_left());
        assert_eq!(100, supplies.clothes_left());
        assert_eq!(150, supplies.misc_left());
    }
}