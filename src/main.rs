use chip8emu::chip8;
use std::fs::File;
use std::env;
use minifb::{Key, Window, WindowOptions};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: program_name file.ch8");
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No file specified"));
    }

    let ch8filename = &args[1];
    let ch8file = File::open(ch8filename)?; // Handle the Result using `?`

    let mut CHIP8 = chip8::Chip8::new(ch8file);
    
    let mut window = Window::new("Test - ESC to exit",
                                 640,
                                 320,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    CHIP8.CPU_reset();
    CHIP8.read_file();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        CHIP8.clock_cycle();
    }


    println!("Hello, world!");
    Ok(())
}

