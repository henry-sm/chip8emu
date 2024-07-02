use chip8emu::chip8;
use std::{fs::File, time::Duration};
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
    
    let mut window = Window::new("chip8emu",
                                 640,
                                 320,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    CHIP8.CPU_reset();
    CHIP8.read_file();
    
    let cycle = Duration::from_millis(1000/60);
    let mut last_cycle = std::time::Instant::now();

    let key_map: std::collections::HashMap<Key, u8> = [
    (Key::Key1, 0x1), (Key::Key2, 0x2), (Key::Key3, 0x3), (Key::Key4, 0xC),
    (Key::Q, 0x4), (Key::W, 0x5), (Key::E, 0x6), (Key::R, 0xD),
    (Key::A, 0x7), (Key::S, 0x8), (Key::D, 0x9), (Key::F, 0xE),
    (Key::Z, 0xA), (Key::X, 0x0), (Key::C, 0xB), (Key::V, 0xF),].iter().cloned().collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        

        let now = std::time::Instant::now();
        if now.duration_since(last_cycle) >= cycle {
            if let Some(keys) = window.get_keys_pressed(minifb::KeyRepeat::No) {
              
            CHIP8.keypad = [false; 16];  
              for key in &key_map {
                if window.is_key_down(*key.0) {
                   // println!("Key: {:?} pressed {:?}", key.0, key.1);
                    CHIP8.keypad[*key.1 as usize] = true;
                  }
                }          
            

           // CHIP8.clock_cycle();
            //CHIP8.keypad = [false; 16];
            


            }

            CHIP8.clock_cycle();

            let buffer = CHIP8.get_buffer();
            window.update_with_buffer(&buffer, 64, 32).unwrap();
         last_cycle = now;
        }
        window.update();
    }

    println!("GAME OVER!");
    Ok(())
}

