use crate::advanced_traits::{Meters, Millimeters};
use hello_macro::HelloMacro;

mod advanced_traits;
mod macros;

#[derive(HelloMacro)]
struct Hello {}

fn main() {
    // unsafe_rust::snippets();

    let meters = Meters(10);
    let milli_meters = Millimeters(10);

    let milli_meters = milli_meters + meters.clone();
    println!("millimeters: {milli_meters:#?}");

    let meters = meters + milli_meters;
    println!("meters: {meters:#?}");

    let tem_vec = my_vec!([1, 3, 4]);
    println!("temp vec: {tem_vec:?}");

    Hello::hello_macro();
}
