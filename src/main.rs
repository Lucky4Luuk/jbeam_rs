//! Test file for the jbeam_rs library
//! Reads from the test_mod directory

fn main() {
    let vehicle = jbeam_rs::Vehicle::from_folder("test_mod/vehicles/test_car").expect("Failed to load test_mod/vehicles/test_car!");
    println!("{:#?}", vehicle);
}
