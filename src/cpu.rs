extern crate num;

use num::{Integer, Zero};

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
    stack: Vec<u16>, // 2 bytes
}

#[allow(dead_code)]
impl MainCPU {
    // here we initialize the main CPU
    pub fn new() -> MainCPU {
        let mut created_cpu = MainCPU {
            operation_code: 0x000,
            memory: allocate_memory::<u8>(4096),
            virtual_registers: allocate_memory::<u8>(16),

            // usize elements, grows as memory allows it (64 bit will be 8 bytes)
            index: 0x000,
            program_counter: 0x200,
            stack_pointer: 0x000,

            stack: allocate_memory::<u16>(16),
        };

        MainCPU::set_font_system(&mut created_cpu);

        created_cpu
    }

    pub fn emulate() {
        // fetch operation
        // decode operation
        // execute operation

        // update timers
    }

    fn set_font_system(cpu: &mut MainCPU) {
        for i in (std::ops::Range { start: 0, end: 80 }) {
            // allocate in the memory fonts utilized later
            cpu.memory[i] = FONTSET[i];
        }
    }
    fn fetch_operation(&self) -> u16 {
        self.merge_opcode(
            self.memory[self.program_counter],
            self.memory[self.program_counter + 1],
        )
    }
    fn decode_operation() {}
    fn execute_operation() {}

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
    fn should_merge_two_bytes() {
        // arrange
        let first: u8 = 0xAA;
        let second: u8 = 0xFF;

        let instantiate_cpu = MainCPU::new();

        let result = instantiate_cpu.merge_opcode(first, second);

        assert_eq!(result, 0xAAFF);
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
}
