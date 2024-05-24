use crate::basic_operations::{ask_terminal, str_to_u16};
use crate::calculation_entropy::markow_source;

extern crate nalgebra as na;

mod basic_operations;
mod calculation_entropy;

fn main() {
    println!(r#"This is a entropy calculator!
    Whats the desired field?
    1) Quelle unabhängiger Ereignisse\
    2) MARKOW_Quelle\
    3) Verbundquelle\
    4) OR TYPE 6969 to quit.
    "#);

    let input = str_to_u16(ask_terminal());

    match input {
        1 => calculation_entropy::of_static_source(),
        2 => calculation_entropy::markow_source(),
        6969 => println!("Quitting..."),
        _ => {println!("Please enter a number in the specified range!\n\n");
                main();}
    }
}