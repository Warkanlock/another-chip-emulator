extern crate num;

use std::{fs::File, io::Read};

use colored::*;
use num::{Integer, Zero};

// helper to allocate memory
pub fn allocate_memory<T>(_len: usize) -> Vec<T>
where
    T: Clone + Integer,
{
    let mut memory = Vec::<T>::with_capacity(_len);
    memory.resize(_len, Zero::zero());
    memory
}

// main fontset memory
static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[allow(dead_code)]
pub struct MainCPU {
    /*
    Memory mapping
    0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    0x200-0xFFF - Program ROM and work RAM
    */
    operation_code: u16,        // 2 bytes
    memory: Vec<u8>,            // [1 byte]
    virtual_registers: Vec<u8>, // byte register,

    // counters
    index: usize,           // 2 bytes,
    program_counter: usize, // 2 bytes, 0x000 - 0xFFF | 0xFFF = 4096
    stack_pointer: usize,   // 2 bytes

    // stack
    stack: Vec<u16>,   // 2 bytes
    base_hex: u16,     // to get value from operation code
    identity_hex: u16, // to get identity from operation code
}

#[allow(dead_code)]
impl MainCPU {
    // here we initialize the main CPU
    pub fn new() -> MainCPU {
        let mut created_cpu = MainCPU {
            operation_code: 0x000,
            memory: allocate_memory::<u8>(4096),
            virtual_registers: allocate_memory::<u8>(16),

            // base operands (from an opcode as 0xABCD we can get A as identity and BCD as base)
            base_hex: 0x0FFF, // to get value from operation code (isolate first-three bits)
            identity_hex: 0xF000, // to get identity from operation code (isolate left-most bit)

            // usize elements, grows as memory allows it (64 bit will be 8 bytes)
            index: 0x000,
            program_counter: 0x200,
            stack_pointer: 0x000,

            stack: allocate_memory::<u16>(16),
        };

        MainCPU::set_font_system(&mut created_cpu);

        created_cpu
    }

    // this method will help us to log only CPU events
    // that's why it's on utils or anything similar
    pub fn log(&mut self, variant: &str, text: String) {
        match variant {
            "info" => println!("{}", format!("[{}] - {}", variant.blue(), text)),
            "warning" => println!("{}", format!("[{}] - {}", variant.yellow(), text)),
            "error" => println!("{}", format!("[{}] - {}", variant.red(), text)),
            "action" => println!("{}", format!("[{}] - {}", variant.green(), text)),
            _ => println!("{}", format!("[{}] - {}", variant.blue(), text)),
        }
    }

    // main loop
    pub fn emulate(&mut self) -> i8 {
        self.log(
            "info",
            format!(
                "stack: {} | program counter: 0x{:X} | previous operation code: 0x{:X}",
                self.stack_pointer, self.program_counter, self.operation_code
            ),
        );

        if self.program_counter == 0x0010 {
            self.log(
                "warning",
                "use stdin input to shutdown the emulator".to_string(),
            );

            return -1; // break program
        };

        // fetch operation
        self.fetch_operation();
        // decode + execute operation
        self.decode_operation();

        // not implemented yet, update timers for graphic purposes
        return 1;
    }

    // fetch
    fn fetch_operation(&mut self) -> u16 {
        // set current operation code as the result of merging two pieces of memory together
        self.operation_code = self.merge_opcode(
            self.memory[self.program_counter],
            self.memory[self.program_counter + 1],
        );

        // increase program counter to match new instruction
        self.program_counter += 1;

        self.log(
            "info",
            format!("fetched operation code -> 0x{:X}", self.operation_code), // this way we print it as hex
        );

        // return resultant operation code
        self.operation_code
    }

    // decode
    fn decode_operation(&mut self) {
        self.log(
            "info",
            format!("decoding operation code -> 0x{:X}", self.operation_code),
        );

        // we read the identifier of the desire action provided by the opcode
        let identifier: u16 = self.operation_code & self.identity_hex;

        // we capture the value
        let value: u16 = self.operation_code & self.base_hex;

        // read more about identifiers: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
        match identifier {
            0x0000 => match value {
                0x0010 => {
                    self.program_counter = 0x0010;
                    self.log("action", "exiting program".to_string())
                }
                0x00E0 => self.clean_display(),
                0x00EE => self.return_from_subroutine(),
                _ => self.log("error", format!("no action implemented yet for this code")),
            },
            0x1000 => self.jp_to_addr(value),
            0x2000 => self.call_subroutine(value),
            _ => self.log("error", format!("no action implemented yet for this code")),
        }

        if self.program_counter != 0x0010 {
            self.program_counter += 1;
        } 
    }

    fn return_from_subroutine(&mut self) {
        self.log("action", "returning from subroutine".to_string());

        // TODO: implement this action

        // we decrement the stack pointer
        // self.stack_pointer -= 1;
        
        // we set the program counter to the top of the stack
        // self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    fn clean_display(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        self.log("action", "cleaning screen".to_string());
    }

    fn jp_to_addr(&mut self, addr: u16) {
        self.log("action", format!("jumping to {}", addr));

        // set program counter to the address
        self.program_counter = addr as usize;
    }

    fn call_subroutine(&mut self, addr: u16) {
        self.log("action", format!("calling subroutine at {}", addr));

        // we increment the stack pointer
        self.stack_pointer += 1;

        // current program counter on top of the stack
        self.stack[self.stack_pointer] = self.program_counter as u16;

        // program counter set to the address
        self.program_counter = addr as usize;
    }

    // memory manipulation
    pub fn load_program(&mut self, memory: &[u8]) {
        self.log("action", "inserting program into memory".to_string());
        self.log("action", format!("program size: {}", memory.len()));

        // load program into memory
        for byte in memory {
            self.load_byte(*byte);
        }
        // reset program counter
        self.program_counter = 0x200;
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // read from file
        let mut file = File::open(path)?;

        self.log("action", "ROM opened".to_string());

        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)?;

        self.log("action", format!("reading ROM from {}", path));

        // load program into memory
        self.load_program(&buffer);

        Ok(())
    }

    fn load_byte(&mut self, value: u8) {
        self.memory[self.program_counter] = value;
        self.program_counter += 1;
    }

    // utils
    fn set_font_system(cpu: &mut MainCPU) {
        for i in (std::ops::Range { start: 0, end: 80 }) {
            // allocate in the memory fonts utilized later
            cpu.memory[i] = FONTSET[i];
        }
    }
    fn merge_opcode(&self, first: u8, second: u8) -> u16 {
        /*
            Merge both u8 and convert it to u16

            first = 00000001
            second = 01010101
            first << 8 = 0000000100000000
            first | second = 0000000100000000 | 0000000001010101
            result = 0000000101010101
        */

        let opcode = (first as u16) << 8 | second as u16;
        opcode
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::allocate_memory;

    use super::MainCPU;

    #[test]
    fn check_allocated_memory() {
        // arrange
        let memory_expected: usize = 4096;
        let memory_size = 4096;

        // act
        let memory_allocated = allocate_memory::<u8>(memory_size);

        // assert
        assert_eq!(memory_allocated.len(), memory_expected);
    }

    #[test]
    fn should_initialize_cpu() {
        let instantiate_cpu = MainCPU::new();

        assert_eq!(instantiate_cpu.memory.len(), 4096);
        assert_eq!(instantiate_cpu.stack.len(), 16);
        assert_eq!(instantiate_cpu.virtual_registers.len(), 16);

        //registers
        assert_eq!(instantiate_cpu.program_counter, 0x200);
        assert_eq!(instantiate_cpu.index, 0x000);
        assert_eq!(instantiate_cpu.stack_pointer, 0x000);
    }

    #[test]
    fn should_load_into_memory() {
        let mut cpu = MainCPU::new();

        // sample instruction
        let instruction: [u8; 2] = [0x10, 0x00];

        // load to memory a piece of memory
        cpu.load_program(&instruction);

        // operation code must be the merge
        // in between two pieces of memory in u8
        let operation_code = cpu.fetch_operation();

        // op code must be 0x1000, which later we can decode and execute
        assert_eq!(operation_code, 0x1000)
    }

    #[test]
    fn fetch_decode_execute_jump() {
        let mut cpu = MainCPU::new();

        // sample instruction
        let instruction: [u8; 2] = [0x10, 0x00];

        // load to memory a piece of memory
        cpu.load_program(&instruction);

        // operation code must be the merge
        // in between two pieces of memory in u8
        let operation_code = cpu.fetch_operation();

        // decoding the operation (and executing it)
        cpu.decode_operation();

        // op code must be 0x1000, which later we can decode and execute
        assert_eq!(operation_code, 0x1000)
    }

    #[test]
    fn fetch_decode_execute_call() {
        let mut cpu = MainCPU::new();

        // sample instruction
        let instruction: [u8; 2] = [0x20, 0x00];

        // load to memory a piece of memory
        cpu.load_program(&instruction);

        // operation code must be the merge
        // in between two pieces of memory in u8
        let operation_code = cpu.fetch_operation();

        // decoding the operation (and executing it)
        cpu.decode_operation();

        // op code must be 0x1000, which later we can decode and execute
        assert_eq!(operation_code, 0x2000)
    }

    #[test]
    fn evaluate_loop() {
        let mut cpu = MainCPU::new();

        // jump + call routine
        let instruction: [u8; 4] = [0x10, 0x00, 0x20, 0x00];

        // load into memory
        cpu.load_program(&instruction);

        // emulate loop to avoid calling operations by hand
        cpu.emulate();
    }

    #[test]
    fn should_merge_two_bytes() {
        // arrange
        let first: u8 = 0xAA;
        let second: u8 = 0xFF;

        let instantiate_cpu = MainCPU::new();

        let result = instantiate_cpu.merge_opcode(first, second);

        assert_eq!(result, 0xAAFF);
    }

    #[test]
    fn should_return_operation_code() {
        let mut instantiate_cpu = MainCPU::new();

        let operation_code = instantiate_cpu.fetch_operation();

        assert_eq!(operation_code, 0x0000)
    }

    #[test]
    fn modify_allocated_memory() {
        // arrange
        let expected_value: u8 = 0x22;
        let memory_size = 4096;

        // act
        let mut memory_allocated = allocate_memory::<u8>(memory_size);

        memory_allocated[2] = expected_value;

        // assert
        assert_eq!(memory_allocated[0], 0x000);
        assert_eq!(memory_allocated[2], expected_value);
    }

    #[test]
    fn load_rom() {
        let mut cpu = MainCPU::new();

        // use path
        let chip8_rom = "roms/hello/HELLO.ch8";

        // load from path
        cpu.load_rom(chip8_rom).unwrap();

        // assert instructions ( increment counter by hand so we avoid decoding )
        assert_eq!(cpu.fetch_operation(), 0x6005);
        cpu.program_counter += 1;
        assert_eq!(cpu.fetch_operation(), 0x6105);
        cpu.program_counter += 1;
        assert_eq!(cpu.fetch_operation(), 0x8014);
        cpu.program_counter += 1;
        assert_eq!(cpu.fetch_operation(), 0x00E0); // this will clean the screan
        cpu.program_counter += 1;
        assert_eq!(cpu.fetch_operation(), 0x00EE);
    }
}
