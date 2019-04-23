use crate::assembler::program_parsers::program;
use crate::instructions::Opcode;
use crate::vm::VM;

use std;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::result::Result::{Err, Ok};

use nom::types::CompleteStr;

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

    pub fn print_help(stdout: &mut StandardStream) {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(170, 96, 48))))
            .expect("set color failed");
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
        .loadfile         : "loads a program file into the program bank"
        .run              : "starts the vm loop with the current program"
        .clear_program    : "clears out the program bank"
        .clear_registers  : "resets all registers to 0"
        .history          : "prints out history of inputted commands"
        .quit             : "closes the shell process"

========================
            "#
        );
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .expect("set color failed");
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
            let mut stdin = io::stdin();

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
                    REPL::print_help(&mut stdout);
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
                ".loadfile" => match REPL::get_file_path_prompt(&mut stdin) {
                    Ok(filepath) => {
                        if self.load_program_filepath(filepath) {
                            println!("Loaded program successfully!");
                        }
                    }
                    Err(_e) => {
                        println!("Error getting input: {}", _e);
                    }
                },
                ".run" => {
                    self.vm.run();
                }
                ".clear_program" | ".clpro" => {
                    self.vm.clear_program();
                    println!("Cleared contents of the program bank!");
                }
                ".clear_registers" | ".clreg" => {
                    self.vm.clear_registers();
                    println!("All registers re-initialized to 0!");
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
                                    REPL::print_help(&mut stdout);
                                }
                            }
                        }
                        _ => {
                            //assume assembly input mode
                            match program(buffer.into()) {
                                Ok((_, program)) => {
                                    //check first if the opcodes are valid before running on system
                                    if program.is_valid() {
                                        self.vm.append_program_bytes(program.to_bytes());
                                        self.vm.run_once();
                                    } else {
                                        println!("Invalid opcode or operands!");
                                        REPL::print_help(&mut stdout)
                                    }
                                }
                                Err(_) => {
                                    println!("Unable to parse input. Please check your syntax.");
                                    REPL::print_help(&mut stdout);
                                }
                            };
                        }
                    }
                }
            }
        }
    }

    /// Helper functions
    ///

    /// File loading prompt
    ///
    fn get_file_path_prompt(stdin: &mut io::Stdin) -> Result<PathBuf, &str> {
        println!("==Please Enter a file path to load==");
        print!("FilePath: ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut tmp = String::new();
        stdin
            .read_line(&mut tmp)
            .expect("Unable to read line from user");
        let tmp = tmp.trim();
        if tmp.is_empty() {
            return Err("There was no input!");
        }
        Ok(Path::new(&tmp).to_path_buf())
    }

    fn load_program_filepath(&mut self, filename: PathBuf) -> bool {
        let mut f = File::open(Path::new(&filename)).expect("File not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("There was an error reading from the file");
        let program = match program(CompleteStr(&contents)) {
            Ok((_, program)) => program,
            Err(e) => {
                println!("Unable to parse input: {:?}", e);
                return false;
            }
        };
        self.vm.append_program_bytes(program.to_bytes());
        true
    }

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
