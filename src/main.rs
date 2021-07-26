use std::io::*;
use crate::banner::*;
use crate::ask::*;
use crate::supplies::*;
use crate::trip::*;
use crate::finish::*;

mod banner;
mod ask;
mod marksman;
mod supplies;
mod trip;
mod finish;

const ASK_OXEN_SPEND: &str = "How much do you want to spend on your \x1B[31mOxen team\x1B[0m? ";

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
        if trip.need_healing() {
            if supplies.money_left() < 20 {
                println!("You can't afford a doctor and died from your injuries.");
                handle_death(&mut stdout, &mut stdin.lock());
                std::process::exit(0);
            } else {
                println!("Doctor's bill is $20 to heal your injuries.");
                supplies.spend(20);
                trip.visit_doctor(&mut supplies);
            }
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
            TurnAction::Hunt => {
                hunt(&mut supplies, &mut stdout, &mut stdin.lock())
            },
            TurnAction::Continue => {
                fort_available = true
            },
        }

        // After turn actions are complete, see if we can survive
        if supplies.food_left() <= 14 {
            println!("\n=================================================================");
            println!("You ran out of food and starved to death.");
            handle_death(&mut stdout, &mut stdin.lock());
            std::process::exit(0);
        }

        // Determine if a fort will be available
        if supplies.ammo_left() > 39 { fort_available = true; }

        // Travel along the Oregon Trail
        trip.turn(supplies.oxen_left());
    }

}

fn hunt<W: Write, R: BufRead>(supplies: &mut Supplies, out: &mut W, input: &mut R) {

}