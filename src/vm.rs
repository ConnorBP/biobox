use crate::instructions::Opcode;

/// this is the definition of our vm
#[derive(Default)]
pub struct VM {
    // the vm has 32bit wide registers
    registers: [i32; 32],
    // program counter
    pc: usize,
    // program bytecode stored as a vector of bytes
    program: Vec<u8>,
    // the remainder attribute left over from division ops
    remainder: u32,
    // Dedicated flag register for the result of the last comparison operation
    equal_flag: bool,
}

/// implementation of the vm
impl VM {
    /// Default values for a new VM
    pub fn new() -> VM {
        VM {
            //fill the default values for the registers, program bytecode, and program counter
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    /// Loops as long as instructions can be executed
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }

        println!("\n\nReached end of execution.");
    }

    /// Runs only one instruction at current program counter (usually 0) then exits
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// this is run every time we need to execute the next instruction
    fn execute_instruction(&mut self) -> bool {
        // if program counter has exceeded length of the program itself, something is wrong
        if self.pc >= self.program.len() {
            return false;
        }
        //decode_opcode is the first 8 bits (pc +1)
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("\n\nHLT Encountered\n");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // cast to usize to use as index in the array
                let number = u32::from(self.next_16_bits());
                self.registers[register] = number as i32; // the registers are i32s
            }
            Opcode::ADD => {
                //addition opcode. stores result in the register //TODO: Maybe an overflow attribute could be stored if an overflow is detected
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                //subtraction opcode. stores result in the register
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                //multiply opcode. stores result in the register
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                //divide opcode. Special Type of OPCODE. Leaves result in provided register and the remainder in the VM remainder attribute
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                // litteral jump opcode. Jumps to the exact instruction program counter location
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                //relative jump opcodes (from current position) jump forward
                let target = self.registers[self.next_8_bits() as usize];
                let result = self.pc.overflowing_add(target as usize);
                if result.1 {
                    //panic!("PROGRAM COUNTER OVERFLOWED! (JMPF went above usize::MAX)");
                    //panic if program counter overflows. (It should never overflow) and print debug info
                    println!("\n\nPROGRAM COUNTER OVERFLOWED! (JMPF went above usize::MAX) at index: {} args: {}\n", self.pc, target);
                    return false;
                }
                self.pc = result.0;
            }
            Opcode::JMPB => {
                //relative jump opcodes (from current position) jump back
                let target = self.registers[self.next_8_bits() as usize];
                let result = self.pc.overflowing_sub(target as usize);
                if result.1 {
                    //panic!("PROGRAM COUNTER OVERFLOWED! (JMPB went below 0)");
                    //panic if program counter overflows. (It should never overflow) and print debug info
                    println!("\n\nPROGRAM COUNTER OVERFLOWED! (JMPB went below 0) at index: {} args: {}\n", self.pc, target);
                    return false;
                }
                self.pc = result.0;
            }
            Opcode::EQ => {
                //equal comparison operator
                //get the contents of the first two registers (16 bits total)
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                //set the equal flag to the result of comparison
                self.equal_flag = register1 == register2;
                //advance the last 8 bits of the instruction row
                self.next_8_bits();
            }
            Opcode::NEQ => {
                //not equal comparison operator
                //get the contents of the first two registers (16 bits total)
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                //set the equal flag to the result of comparison
                self.equal_flag = register1 != register2;
                //advance the last 8 bits of the instruction row
                self.next_8_bits();
            }
            Opcode::GT => {
                //greater than comparison operator
                //get the contents of the first two registers (16 bits total)
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                //set the equal flag to the result of comparison
                self.equal_flag = register1 > register2;
                //advance the last 8 bits of the instruction row
                self.next_8_bits();
            }
            Opcode::LT => {
                //less than comparison operator
                //get the contents of the first two registers (16 bits total)
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                //set the equal flag to the result of comparison
                self.equal_flag = register1 < register2;
                //advance the last 8 bits of the instruction row
                self.next_8_bits();
            }
            Opcode::GTQ => {
                //greater than or equal comparison operator
                //get the contents of the first two registers (16 bits total)
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                //set the equal flag to the result of comparison
                self.equal_flag = register1 >= register2;
                //advance the last 8 bits of the instruction row
                self.next_8_bits();
            }
            Opcode::LTQ => {
                //less than or equal comparison operator
                //get the contents of the first two registers (16 bits total)
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                //set the equal flag to the result of comparison
                self.equal_flag = register1 <= register2;
                //advance the last 8 bits of the instruction row
                self.next_8_bits();
            }
            Opcode::JEQ => {
                //jump if equal. Jumps to provided PC index if the previous comparison resulted in true
                let register = usize::from(self.next_8_bits());
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            _ => {
                println!("\n\nUnrecognized opcode found! Terminating!\n");
                return false;
            }
        }
        true // continue to the next itteration of the loop by default. The next 8 bits waiting to be read should be an opcode.
    }

    //
    // Helpers
    //

    //opcode decoder helper

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    //bit helpers

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = (u16::from(self.program[self.pc]) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
        result
    }

    //
    // Setters and Getters
    //

    pub fn get_program(&mut self) -> Vec<u8> {
        //return a copy of the program contents vector
        self.program.to_vec()
    }

    pub fn get_registers(&mut self) -> [i32; 32] {
        //return the data in the VM registers
        self.registers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        //load opcode = 1
        test_vm.program = vec![1, 0, 1, 244]; //this is how we represent 500 using two u8s in little endian format
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        //registers 0 and 1 are values 500 and 24
        test_vm.registers[0] = 500;
        test_vm.registers[1] = 24;
        //add(2) the values of register 0 and 1 then store the result into register 3
        test_vm.program = vec![2, 0, 1, 3];
        test_vm.run_once();
        assert_eq!(test_vm.registers[3], 524);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        //jump to pc 1
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jumpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        //7, 0 is jump forward amount in reg0 (which is 2) which skips the last 2 zeros of line1 (the remaining 16 bits on the jmpf instruction line)
        // into line2 which is a normal jmp at index 4bytes (32 bits, the second instruction row)
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jumpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        //goes forward 2 bytes to read instruction and register 0, register 0 is 2 which means go back 2
        test_vm.program = vec![8, 0, 0, 0];
        test_vm.run_once();
        //going back to from pc 2 is 0
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        //eq opcode(9) testing against registers 0 and 1 should result in true
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 on a different value it should now result in false
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 12;
        //neq opcode(10) testing against registers 0 and 1 should result in true
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 on the same value now it should now result in false
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 9;
        //gt opcode(11) testing against registers 0 and 1 should result in true
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 on a different value it should now result in false
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;
        //lt opcode(12) testing against registers 0 and 1 should result in true
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 on a different value it should now result in false
        test_vm.registers[1] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 9;
        //gtq opcode(13) testing against registers 0 and 1 should result in true
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 as same value it should still result in true
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 as higher value it should now result in false
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 9;
        test_vm.registers[1] = 10;
        //ltq opcode(14) testing against registers 0 and 1 should result in true
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 as same value it should still result in true
        test_vm.registers[1] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        //with register 1 as lower value it should now result in false
        test_vm.registers[1] = 8;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        //JEQ opcode 15 to the location in register 0 (7) if equal_flag is true (it is)
        test_vm.program = vec![15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}
