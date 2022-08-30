#[macro_use] extern crate rocket;

use mycargo::myrandom;

#[get("/probabilityOfUnitInjectorFail/<vin>")]
fn probability_of_unit_injector_fail(vin: String) -> String {
    let random_probability_u8 = myrandom::random_percentage();
    let random_probability_f32 = random_probability_u8 as f32 / 100.0 ;
    println!("\n----------------\n[... beep beep prrrt ...]\n----------");
    println!("\tModel: PeopleCar PasWagon C6,\n\tVIN:\t{}", vin.as_str());
    println!("\n\tProbability of failure: \x1b[93m{}%\x1b[0m ({:.2})\n--------------\n", random_probability_u8, random_probability_f32);

    random_probability_f32.to_string().replace(".", ",")
}

#[get("/calculate/<distance>/<prod_year>/<fuel>")]
fn calculate(distance: u32, prod_year: u16, fuel: f32) -> String {

    let fuel_consumed = ((distance as f32)/100.0) * fuel;

    println!("\n----------------\n[... beep beep prrrt ...]\n----------");

    mycargo::myrandom::public_function();
    
    println!("Distance: \x1b[92m{} km\x1b[0m", distance);
    println!("Car production year: \x1b[1m{}\x1b[0m", prod_year);
    println!("Fuel consumption (per 100 km): \x1b[93m{} l\x1b[0m\n----------", fuel);
    println!("\t Fuel used:\n\t\t\x1b[4m{:.2} l\x1b[0m\n--------------\n", fuel_consumed);
    
    fuel_consumed.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![probability_of_unit_injector_fail, calculate])
}


// tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}