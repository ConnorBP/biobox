// An input output "blackbox" I haved dubbed the BIOBOX!!!!!!!!!!!!!! :^) I think it's a pretty cool name anyways :D
// BIOBox is a virtual machine that you pass inputs, and it returns an output.
// Everything inbetween is abstracted by a VM so you can write your own blackbox module,
// compile it down to "machine code" and plop it in as a module either encrypted or not
// modules should be able to run in Rust or C++ code bases and maybe others down the road.
//
// Language: Syntax should be C-like and focused primarily on mathematical and binary operations
// language features should include GLSL like features and syntax and easy binary and mathematical primitives and operations
// focus is on input types and the final output/s
// features for obfuscating the binary with either built in or provided xor or similar functions should be worked in somehow (for protecting proprietary tech)
// optomizations on the engine to make sure primative math and binary functions run as close to the metal as they can would also be nice

//import the modules
pub mod instructions;
//vm after instructions because it uses instructions in the vm :)
pub mod vm;

fn main() {
    println!("Hello, world!");
}
