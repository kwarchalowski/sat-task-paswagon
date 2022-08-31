// * All printlns left for better console output @ API calls. 'Weird' formatting because of coloring parts of the logs.
// * non_snake_case param allowed to stick with the endpoint/inputs/ names from solution description.

#![allow(non_snake_case)]
#[macro_use] extern crate rocket;

#[get("/probabilityOfUnitInjectorFail/<vin>")]
fn probability_of_unit_injector_fail(vin: String) -> String {
    let tmp_failProbability = mycargo::myutils::random_percentage();
    let failProbability = tmp_failProbability as f32 / 100.0 ;

    println!("\n----------------\n[... beep beep prrrt ...]\n--------");
    println!("\tModel:  PeopleCar PasWagon C6\n\tVIN:\t{}", vin);
    println!("\n\tProbability of failure: \x1b[93m{}%\x1b[0m ({:.2})\n----------------\n", tmp_failProbability, failProbability);

    failProbability.to_string().replace(".", ",")
}

// ? '..Dissel..' because of the name stated in solution description. Should change to '..Diesel..'?
#[get("/calculateDisselUsageForDistance/<distance>/<yearOfProduction>/<fuelUsagePer100KM>")]
fn calculate_dissel_usage_for_distance(distance: u32, yearOfProduction: u16, fuelUsagePer100KM: u16) -> String {

    let fuelUsage = ((distance as f32)/100.0) * fuelUsagePer100KM as f32;
    let yearOfProductionStatus = if mycargo::myutils::is_car_production_year_valid(&yearOfProduction) {"VALID"} else {"INVALID"};

    println!("\n----------------\n[... beep beep prrrt ...]\n----------");
    println!("Distance: \x1b[92m{} km\x1b[0m", distance);
    println!("Car production year: \x1b[1m{}\x1b[0m (\x1b[1m{}\x1b[0m)", yearOfProduction, yearOfProductionStatus);
    println!("Fuel consumption (per 100 km): \x1b[93m{} l\x1b[0m\n----------", fuelUsagePer100KM);
    println!("\t Fuel used:\n\t\t\x1b[4m{:.2} l\x1b[0m\n--------------\n", fuelUsage);

    fuelUsage.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![probability_of_unit_injector_fail, calculate_dissel_usage_for_distance])
}

// * Tests
#[cfg(test)]
#[path = "tests/main_tests.rs"]
mod main_tests;