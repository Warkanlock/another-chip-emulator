use crate::cpu::MainCPU;

mod cpu;

// dynamic trait for errora not known at compile time
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // instantiate cpu
    let mut cpu = MainCPU::new();

    // jump to 0x0001 + call routine + exit (0x0010)
    let instruction: [u8; 6] = [0x10, 0x00, 0x20, 0x00, 0x00, 0x10];

    // load program into memory
    cpu.load_program(&instruction);

    // run program
    loop {
        let status = cpu.emulate();
        if status == -1 {
            break;
        }
    }

    Ok(())
}
