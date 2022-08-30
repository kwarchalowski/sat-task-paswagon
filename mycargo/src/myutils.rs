extern crate rand;
extern crate chrono;

use self::rand::{thread_rng, Rng};
use self::chrono::Datelike;

pub fn random_percentage() -> u8 {
    thread_rng().gen_range(0..=100) as u8
}

pub fn is_car_production_year_valid(year: &u16) -> bool {

    let &production_year = year;
    if production_year < 1885 { return false }

    let current_year = chrono::Utc::now().year() as u16;
    if production_year > current_year { return false }

    true
}