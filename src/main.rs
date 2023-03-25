use crate::cpu::MainCPU;

mod cpu;

// dynamic trait for errora not known at compile time
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // instantiate cpu
    let mut cpu = MainCPU::new();

    // read input from parameter
    let args: Vec<String> = std::env::args().collect();
    let rom_name = &args[1];

    // jump to 0x0001 + call routine + exit (0x0010)
    let path = format!("roms/{}/{}.ch8", rom_name, rom_name.to_uppercase());

    cpu.log("info", format!("Loading ROM: {}", path));

    cpu.load_rom(&path)?;

    // run program
    loop {
        let status = cpu.emulate();
        if status == -1 {
            break;
        }
    }

    Ok(())
}
