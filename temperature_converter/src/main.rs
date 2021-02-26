#![allow(dead_code)]

use std::io;
use std::io::Write;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq)]
enum Degree {
    Celsius,
    Fahrenheit,
}

impl FromStr for Degree {
    type Err = ();
    fn from_str(input: &str) -> Result<Degree, Self::Err> {
        match input.to_lowercase().as_str() {
            "celsius" => Ok(Degree::Celsius),
            "fahrenheit" => Ok(Degree::Fahrenheit),
            _ => Err(()),
        }
    }
}

fn main() {
    loop {
        print!("Please type the degree to start with (");
        for dg in Degree::iter() {
            print!(" < {:?} > ", dg);
        }
        println!(")");

        let type_input = loop {
            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("<< Failed to read line >>");

            let temp: Degree = match temp.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("< Please type correctly >");
                    continue;
                }
            };

            break temp;
        };

        let number_input: f32 = loop {
            print!("Number: ");
            io::stdout().flush().expect("Flush failed");
            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("<< Failed to read line >>");
            let temp: f32 = match temp.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Please enter number");
                    continue;
                }
            };

            break temp;
        };

        println!(
            "Convertion from {:?} to {:?}, number: {} -> {}",
            type_input,
            if type_input == Degree::Celsius {
                Degree::Fahrenheit
            } else {
                Degree::Celsius
            },
            number_input,
            convert_temp_degree(&type_input, number_input)
        );

        let mut next: bool = true;

        loop {
            print!("Want to convert another one? < y > or < n >: ");
            io::stdout().flush().expect("flash failed");
            let mut decision = String::new();
            io::stdin()
                .read_line(&mut decision)
                .expect("<< Failed to read line >>");

            let decision = decision.trim().chars().nth(0).unwrap();

            match decision {
                'y' | 'Y' => {
                    println!("< ------------------------------------------------------- >");
                    break;
                }
                'n' | 'N' => {
                    println!("\t<<< Conversion ended >>>");
                    next = false;
                    break;
                }
                _ => println!("< Please type either y or n >"),
            }
        }

        if !next {
            break;
        }
    }
}

fn convert_temp_degree(dg: &Degree, num: f32) -> f32 {
    if *dg == Degree::Celsius {
        return 9. / 5. * num + 32.;
    } else {
        return 5. / 9. * (num - 32.);
    }
}
