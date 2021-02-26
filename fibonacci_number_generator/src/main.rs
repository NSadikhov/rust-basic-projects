#![allow(dead_code)]

use dialoguer::{theme::ColorfulTheme, Select};

use console::style;
use std::io;
use std::io::Write;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GOLDEN_RATIO: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
}

fn main() {
    loop {
        println!(
            "<<< Find the {} term of the Fibonacci sequence >>>",
            style("n-th").cyan().bold()
        );
        print!("Enter n: ");
        io::stdout().flush().expect("Failed to write");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let n: i32 = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("<<< {} >>>", style("Please enter number").red().bold());
                continue;
            }
        };

        let fibonacci_number =
            ((*GOLDEN_RATIO).powi(n) - (1.0 - *GOLDEN_RATIO).powi(n)) / 5.0_f64.sqrt();

        println!("Result: {:.0}", fibonacci_number);

        match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Another one?")
            .item("Yes")
            .item("No")
            .default(0)
            .interact()
            .unwrap()
        {
            0 => (),
            1 => {
                println!("<<< Application ended >>>");
                break;
            }
            _ => break,
        }
    }
}
