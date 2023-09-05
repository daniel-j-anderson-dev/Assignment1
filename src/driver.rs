use std::io::Cursor;

use crate::order::Order;
use crate::order_list::OrderList;

pub struct Driver {}
impl Driver {
    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        println!("FastFood World Order Tracker\n");
        let menu: &str = "Please select a menu option:\n1- Create order\n2- Delete order\n3- Ready order\n4- Print order\n5- Print all orders\n6- Exit\n";
        let mut restraunt_orders: OrderList = OrderList::new();
        loop {
            match Driver::get_input(menu)?.parse::<usize>() {
                Ok(menu_input) => match menu_input {
                    1 => {
                        println!();
                        let mut items: [String; 3] = [String::new(), String::new(), String::new()];
                        for (i, item) in items.iter_mut().enumerate() {
                            let prompt: String = format!("Enter item {}: ", i + 1);
                            *item = Driver::get_input(&prompt)?;
                        }
                        restraunt_orders.add_order(Order::new(&items));
                        println!("Order has been added\n", );
                    }
                    2 => {
                        println!();
                        let id: usize = Driver::get_id()?;
                        match restraunt_orders.remove_order(id) {
                            Some(order) => println!("Order {} has been removed.\n", order.get_id()),
                            None => println!("No order with such id\n"),
                        }
                    }
                    3 => {
                        println!();
                        let id: usize = Driver::get_id()?;
                        if restraunt_orders.ready_order(id) {
                            println!("Order {} has been set to \"Ready\".\n", id);
                        } else {
                            println!("No order with such id\n");
                        }
                    }
                    4 => {
                        println!();
                        let id: usize = Driver::get_id()?;
                        match restraunt_orders.print_order(id) {
                            Some(order_string) => println!("{order_string}"),
                            None => println!("No order with such id\n"),
                        }
                    }
                    5 => println!("\n{}", restraunt_orders.print_orders()),
                    6 => {
                        println!("\nShutting off");
                        break
                    },
                    _ => eprintln!("\nInvalid input: Please enter one of the numbers displayed\n"),
                },
                Err(parse_int_error) => eprintln!("\nInvalid input: {parse_int_error}\n"),
            }
        }

        return Ok(());
    }

    fn menu() {
        println!();
    }

    fn get_id() -> Result<usize, std::io::Error> {
        loop {
            match Driver::get_input("Please enter order id: ")?.parse() {
                Ok(id) => return Ok(id),
                Err(parse_int_error) => {
                    eprintln!("Invalid input: {parse_int_error}\n");
                    continue;
                },
            }
        }
    }

    fn get_input(prompt: &str) -> Result<String, std::io::Error> {
        use std::io::{self, stdin, stdout, Write};
    
        if stdout().write(prompt.as_bytes())? == 0 {
            return Err(io::Error::new(io::ErrorKind::WriteZero, "nothing written to stdout"));
        };
        stdout().flush()?;
    
        let mut input: String = String::new();
        if stdin().read_line(&mut input)? == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "nothing read from stdin"));
        }
        input = input.trim().to_owned();

        return Ok(input);
    }
}