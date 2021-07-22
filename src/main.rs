use std::io::*;
use chrono::{NaiveDate, Duration};
use crate::banner::*;
use crate::marksman::*;
use crate::supplies::*;
use crate::mileage::*;

mod banner;
mod ask;
mod marksman;
mod supplies;
mod mileage;

const ASK_OXEN_SPEND: &str = "How much do you want to spend on your oxen team? ";
const ASK_FOOD_SPEND: &str = "How much do you want to spend on food? ";
const ASK_AMMO_SPEND: &str = "How much do you want to spend on ammunition? ";
const ASK_CLOTHES_SPEND: &str = "How much do you want to spend on clothing? ";
const ASK_MISC_SPEND: &str = "How much do you want to spend on miscellaneous supplies? ";

fn main() {
    let mut stdout = stdout();
    let stdin = stdin();
    print_banner(&mut stdout);

    let mut marksman;
    loop {
        marksman = MarksmanQuality::from_u32(ask!(include_str!("../strings/ask_marksman.txt"), stdout, stdin));
        if marksman != MarksmanQuality::Unknown { break; }
    }

    let mut supplies = Supplies::new();
    ask_ok!(supplies.buy_oxen(ask!(ASK_OXEN_SPEND, &mut stdout, &mut stdin.lock())));
    ask_ok!(supplies.buy_food(ask!(ASK_FOOD_SPEND, &mut stdout, &mut stdin.lock())));
    ask_ok!(supplies.buy_ammo(ask!(ASK_AMMO_SPEND, &mut stdout, &mut stdin.lock())));
    ask_ok!(supplies.buy_clothes(ask!(ASK_CLOTHES_SPEND, &mut stdout, &mut stdin.lock())));
    ask_ok!(supplies.buy_misc(ask!(ASK_MISC_SPEND, &mut stdout, &mut stdin.lock())));

    let mut mileage = Mileage::new();
    let mut trip_date: NaiveDate = NaiveDate::from_ymd(1847, 03, 29);
    println!("After all your purchases, you now have ${} left\n", supplies.money_left());

    loop {
        println!("\n=================================================================");
        if mileage.traveled() >= 2040 {
            complete_trip(&mut stdout, &mut supplies);
            std::process::exit(0);
        }

        if supplies.food_left() <= 12 {
            println!("You'd better do some hunting or buy some food, and soon!!!!");
        }
        println!("Total mileage traveled: {}", mileage.traveled());
        println!("It is now {}\n", trip_date.format("%A %d-%b-%Y"));
        println!("Supplies remaining:\n{}", supplies);

        'turn_input: loop {
            let turn_action = ask!("Do you want to?\n  1) Continue\n", &mut stdout, &mut stdin.lock());
            match turn_action {
                1 => break 'turn_input,
                _ => continue 'turn_input,
            }
        }

        trip_date += Duration::days(14);
        mileage.turn(200, supplies.oxen_left());
    }

}
