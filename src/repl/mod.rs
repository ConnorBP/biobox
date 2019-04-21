use std;
use std::io;
use std::io::Write;
use crate::vm::VM;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Core structure for the REPL for the Assembler
#[derive(Default)]
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {//01 0C 03 E8
    /// Creates and returns a new assembly REPL
    pub fn new()->REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn print_help() {
        println!(
            r#"
======BIOBOX Usage======
    Commands:

        .help | .usage : "shows this message"
        .program : "prints the contents of the VM program instructions"
        .registers : "prints the contents of the VM Registers"
        .history : "prints out history of inputted commands"
        .quit : "closes the shell process"

========================
            "#
        );
    }

    pub fn run(&mut self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        //writeln!(&mut stdout, "green text!");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
        print!("==========");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        print!("BIOBOX Terminal");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
        println!("==========");
        println!("===================================");
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
        // this allocates a new String that we store what the user types each loop itteration
        let mut buffer = String::new();
        loop {
            //reset buffer at beginning of new itteration
            buffer = "".to_string();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            //our fancy terminal prompt :^) TODO: Replace print and flush with the manual version
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            print!("B~> ");
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
            io::stdout().flush().expect("Unable to flush stdout");

            //read the line for the user input string
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();

            //store the history of command inputs
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".help" | ".usage" => {
                    REPL::print_help();
                },
                ".program" => {
                    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                    println!("Listing VM program instructions contents:");
                    for instruction in &self.vm.get_program() {
                        println!("[{}]", instruction);
                    }
                    println!("~~~~~~~End of Program Instructions~~~~~~~");
                    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.get_registers());
                    println!("End of registers listing.");
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("> {}", command);
                    }
                }
                ".quit" => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                    println!("Goodbye!~");
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    println!("~~~~~~~~~");
                    std::process::exit(0);//ends the process right away
                    //break;//break out of the execution loop to reach the natural end of the process
                },
                _ => {
                    println!("Invalid input!");
                    REPL::print_help();
                }
            }
        }
    }
}