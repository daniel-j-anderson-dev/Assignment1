mod driver;
mod order;
mod order_list;

use crate::driver::Driver;

fn main() {
    if let Err(error) = Driver::main() {
        eprintln!("Error in Driver::main: {error}");
    }
}
