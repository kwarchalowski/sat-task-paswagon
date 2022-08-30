extern crate rand;

use self::rand::{thread_rng, Rng};

pub fn public_function() {
    println!("Hello, imma public fn.");
}

pub fn random_percentage() -> u8 {
    //format!("{}", rand::random::<u8>() );
    let mut rng = thread_rng();

    //let my_generated_number = rng.gen_range(0..100) as u8;
    //my_generated_number

    rng.gen_range(0..=100) as u8
}