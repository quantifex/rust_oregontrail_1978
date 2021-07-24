use std::io::*;
use crate::banner::*;
use crate::ask::*;
use crate::supplies::*;
use crate::trip::*;

mod banner;
mod ask;
mod marksman;
mod supplies;
mod trip;

const ASK_OXEN_SPEND: &str = "How much do you want to spend on your oxen team? ";
const ASK_FOOD_SPEND: &str = "How much do you want to spend on food? ";
const ASK_AMMO_SPEND: &str = "How much do you want to spend on ammunition? ";
const ASK_CLOTHES_SPEND: &str = "How much do you want to spend on clothing? ";
const ASK_MISC_SPEND: &str = "How much do you want to spend on miscellaneous supplies? ";

fn main() {
    let mut stdout = stdout();
    let stdin = stdin();
    print_banner(&mut stdout);

    let _marksman = ask_marksman(&mut stdout, &mut stdin.lock());

    let mut supplies = Supplies::new();
    ask_ok!(supplies.buy_oxen(ask!(ASK_OXEN_SPEND, &mut stdout, &mut stdin.lock())));
    supplies.buy(&mut stdout, &mut stdin.lock());
    supplies.set_premium(0.333);

    let mut trip = Trip::new();
    let mut fort_available = false;
    loop {
        println!("\n=================================================================");
        if trip.miles_traveled() >= 2040 {
            complete_trip(&mut stdout, &mut supplies);
            std::process::exit(0);
        }

        if supplies.food_left() <= 12 {
            println!("You'd better do some hunting or buy some food, and soon!!!!");
        }
        println!("Total mileage traveled: {}\nIt is now {}\nSupplies remaining:\n{}", 
            trip.miles_traveled(), trip.current_date().format("%A %d-%b-%Y"), supplies);

        // Prompt for an action
        let action;
        if fort_available { action = ask_fort_hunt_continue(&mut stdout, &mut stdin.lock()) }
        else { action = ask_hunt_continue(&mut stdout, &mut stdin.lock()); }
        match action {
            TurnAction::Fort => {
                supplies.buy(&mut stdout, &mut stdin.lock());
                fort_available = false;
                trip.reverse(45);
            },
            TurnAction::Hunt => hunt(&mut supplies, &mut stdout, &mut stdin.lock()),
            TurnAction::Continue => fort_available = true,
        }

        if supplies.ammo_left() > 39 { fort_available = true; }
        trip.turn(200, supplies.oxen_left());
    }

}

fn hunt<W: Write, R: BufRead>(supplies: &mut Supplies, out: &mut W, input: &mut R) {

}