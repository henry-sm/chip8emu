use chip8emu::chip8;
use std::fs::File;
use std::env;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: program_name file.ch8");
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No file specified"));
    }

    let ch8filename = &args[1];
    let mut ch8file = File::open(ch8filename)?; // Handle the Result using `?`

    let mut CHIP8 = chip8::Chip8::new(ch8file);
    CHIP8.CPU_reset();
    CHIP8.opcode();
    println!("Hello, world!");
    Ok(())
}


