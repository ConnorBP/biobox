use crate::assembler::program_parsers::program;
use crate::instructions::Opcode;
use crate::vm::VM;

use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use std::result::Result::Err;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Core structure for the REPL for the Assembler
#[derive(Default)]
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn print_help() {
        println!(
            r#"
======BIOBOX Usage======
    Execute VM Opcode:

        Enter an instruction to execute directly on the vm
        Example for a LOAD instruction: load $0 #100
        Example for add (add register 3 to register 1 and store in register 4): add $1 $3 $4

        Optionally:
        Enter in a Hex String (prefixed by 0x) to execute opcodes directly on the vm
        Example for a LOAD command: 0x01 01 03 E8
        Example for add (add register 3 to register 1 and store in register 4): 0x02 01 03 04

    Commands:

        .help | .usage    : "shows this message"
        .codes | .asm     : "shows a list of opcodes/instructions available"
        .program          : "prints the contents of the VM program instructions"
        .registers        : "prints the contents of the VM Registers"
        .history          : "prints out history of inputted commands"
        .quit             : "closes the shell process"

========================
            "#
        );
    }

    pub fn run(&mut self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        //writeln!(&mut stdout, "green text!");
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
            .expect("set color failed");
        print!("==========");
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .expect("set color failed");
        print!("BIOBOX Terminal");
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
            .expect("set color failed");
        println!("==========");
        println!("===================================");
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .expect("set color failed");
        // this allocates a new String that we store what the user types each loop itteration
        let mut buffer;
        loop {
            //reset buffer at beginning of new itteration
            buffer = "".to_string();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            //our fancy terminal prompt :^) TODO: Replace print and flush with the manual version
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                .expect("set color failed");
            print!("B~> ");
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                .expect("set color failed");
            io::stdout().flush().expect("Unable to flush stdout");

            //read the line for the user input string
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            //store the history of command inputs
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".help" | ".usage" => {
                    REPL::print_help();
                }
                ".codes" | ".instructions" | ".asm" => {
                    Opcode::get_list();
                }
                ".program" => {
                    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                    println!("Listing VM program instructions contents:");
                    for instruction in &self.vm.get_program() {
                        println!("[{}]", instruction);
                    }
                    println!("~~~~~~~End of Program Instructions~~~~~~~");
                    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.get_registers());
                    println!("End of registers listing.");
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("> {}", command);
                    }
                }
                ".quit" => {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                        .expect("set color failed");
                    println!("Goodbye!~");
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                        .expect("set color failed");
                    println!("~~~~~~~~~");
                    std::process::exit(0); //ends the process right away
                                           //break;//break out of the execution loop to reach the natural end of the process
                }
                line => {
                    match &line[..2] {
                        "0x" => {
                            //hex input mode
                            let results = match REPL::remove_first2(buffer) {
                                Some(value) => Some(self.parse_hex(value)),
                                None => None,
                            };
                            match results {
                                Some(Ok(bytes)) => {
                                    for byte in bytes {
                                        self.vm.add_byte(byte);
                                    }
                                    self.vm.run_once();
                                }
                                Some(Err(_)) | None => {
                                    println!("Unable to decode hex string. Please enter 4 groups (separated by spaces) of 2 hex characters each.");
                                    REPL::print_help();
                                }
                            }
                        }
                        _ => {
                            //assume assembly input mode
                            match program(buffer.into()) {
                                Ok((_, program)) => {
                                    //check first if the opcodes are valid before running on system
                                    if program.is_valid() {
                                        self.vm.append_bytes(program.to_bytes());
                                        self.vm.run_once();
                                    } else {
                                        println!("Invalid opcode or operands!");
                                        REPL::print_help()
                                    }
                                }
                                Err(_) => {
                                    println!("Unable to parse input. Please check your syntax.");
                                    REPL::print_help();
                                }
                            };
                        }
                    }
                }
            }
        }
    }

    /// Helper functions

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 01 01 03 E8 or 01 0C 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    fn remove_first2(s: &str) -> Option<&str> {
        s.chars().next().map(|c| &s[c.len_utf8() * 2..])
    }
}
