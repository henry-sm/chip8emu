use std::io::Read;
use rand::Rng;



const FONT: [[u8; 5]; 16] = [
    [0xf0, 0x90, 0x90, 0x90, 0xf0], // 0, 1 all the way to F
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xf0, 0x10, 0xf0, 0x80, 0xf0],
    [0xf0, 0x10, 0xf0, 0x10, 0xf0],
    [0x90, 0x90, 0xf0, 0x10, 0x10],
    [0xf0, 0x80, 0xf0, 0x10, 0xf0],
    [0xf0, 0x80, 0xf0, 0x90, 0xf0],
    [0xf0, 0x10, 0x20, 0x40, 0x40],
    [0xf0, 0x90, 0xf0, 0x90, 0xf0],
    [0xf0, 0x90, 0xf0, 0x10, 0xf0],
    [0xf0, 0x90, 0xf0, 0x90, 0x90],
    [0xe0, 0x90, 0xe0, 0x90, 0xe0],
    [0xf0, 0x80, 0x80, 0x80, 0xf0], 
    [0xe0, 0x90, 0x90, 0x90, 0xe0],
    [0xf0, 0x80, 0xf0, 0x80, 0xf0], 
    [0xf0, 0x80, 0xf0, 0x80, 0x80], 
];

pub struct Chip8 {
    register : [u8; 16], //16 general purpose registers
    i : u16, //store mem adress
    dt : u8, //delay itmer
    st : u8, //sound timer
    pc : u16, //program counter
   // sp : u8, //stack pointer
    stk : Vec<u16>, // stack
    memory : [u8; 0xfff], // 4k RAM
    display : [[u8;64];32], // 64x32 display
    pub keypad : [bool; 16], // keypad input
    file : std::fs::File,
}


impl Chip8{
    
    pub fn new(chfile :std::fs::File ) -> Chip8{
        let mut memory = [0; 0xfff];
        memory[..5].copy_from_slice(&FONT[0]); // the 5 could be 80
        Chip8{
            register : [0; 16],
            i : 0,
            dt : 0,
            st : 0,
            pc : 0x200,
            stk : Vec::new(),
            memory : memory,
            display : [[0;64];32],
            keypad : [false; 16],
            file : chfile,
        }
    }


    pub fn CPU_reset(&mut self){
        self.pc = 0x200;
        //memset (register, 0, std::mem::size_of_val(&register)); //registers are 0 //where did memset go?
    }

    pub fn read_file(&mut self){
        let mut buffer: [u8; 3583] = [0; 3583]; //the fist 512 bytes are reserved for the interpreter
        self.file.read(&mut buffer).unwrap();
        self.memory[0x200..].copy_from_slice(&buffer);
    }
    pub fn opcode(&mut self){
        //take opcode from memory and read 2 bytes from it
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);
        let nnn: u16 = opcode & 0xfff;
        let kk = (opcode & 0xff) as u8;
        let n = (opcode & 0xf) as u8 ;
        let x = ((nnn>>8) & 0xf) as usize;
        let y = ((kk>>4) & 0xf) as usize;

        match &opcode & 0xf000 {
            0x0000 => match opcode {
                0x00e0 => self._00e0(),
                0x00ee => self._00ee(),
                _ => { 
                    self.pc += 2;
                    println!("Invalid opcode {:X}", opcode);
                }
            }

            0x1000 => self._1nnn(nnn),
            0x2000 => self._2nnn(nnn),
            0x3000 => self._3xkk(x, kk),
            0x4000 => self._4xkk(x, kk),
            0x5000 => self._5xy0(x, y),
            0x6000 => self._6xkk(x, kk),
            0x7000 => self._7xkk(x, kk),

            0x8000 => match n{
                0x0000 => self._8xy0(x, y),
                0x0001 => self._8xy1(x, y),
                0x0002 => self._8xy2(x, y),
                0x0003 => self._8xy3(x, y),
                0x0004 => self._8xy4(x, y),
                0x0005 => self._8xy5(x, y),
                0x0006 => self._8xy6(x, y),
                0x0007 => self._8xy7(x, y),
                0x000e => self._8xye(x, y),
                _ =>{
                    self.pc += 2;
                    println!("Invalid opcode {:X}", opcode);
                }
            }

            0x9000 => self._9xy0(x, y),
            0xa000 => self._annn(nnn),
            0xb000 => self._bnnn(nnn),
            0xc000 => self._cxkk(x, kk),
            0xd000 => self._dxyn(x, y, n),

            0xe000 => match kk{
                0x009e => self._ex9e(x),
                0x00a1 => self._exa1(x),
                _ =>{
                    self.pc += 2;
                    println!("Invalid opcode {:X}", opcode);
                }
            }

            0xf000 => match kk{
                0x0007 => self._fx07(x),
                0x000a => self._fx0a(x),
                0x0015 => self._fx15(x),
                0x0018 => self._fx18(x),
                0x001e => self._fx1e(x),
                0x0029 => self._fx29(x),
                0x0033 => self._fx33(x),
                0x0055 => self._fx55(x),
                0x0065 => self._fx65(x),
                _ =>{
                    self.pc += 2;
                    println!("Invalid opcode {:X}", opcode);
                }
            }
            
            _ =>{
                self.pc += 2;
                println!("Invalid opcode {:X}", opcode);
            }
        }

        if opcode & 0xf000 != 0x1000 && opcode & 0xf000 != 0x2000 && opcode != 0x00ee{
            self.pc += 2;
        }

    }

    pub fn clock_cycle(&mut self){
            self.opcode(); //fetch opcode)
            if self.dt > 0{ 
                self.dt -= 1;
            }
            if self.st > 0{
                self.st -= 1;
            }
            // decrement the timers
    }

    pub fn get_buffer(&self) -> Vec<u32> {
        let mut buffer = vec![0; 64 * 32];
        for y in 0..32 {
            for x in 0..64 {
                buffer[y * 64 + x] = if self.display[y][x] == 1 { 0xffffff } else { 0 };
            }
        }
        buffer
    }   



}

impl Chip8{ //separate impl for opcodes 

    pub fn _00e0(&mut self){
        self.display.fill([0;64]);
    }

    pub fn _00ee(&mut self){
        self.pc = self.stk.pop().unwrap();
    }

    pub fn _1nnn(&mut self, nnn : u16){
        self.pc = nnn;
    }

    pub fn _2nnn(&mut self, nnn : u16){
        self.stk.push(self.pc);
        self.pc = nnn;
    }

    pub fn _3xkk(&mut self, x : usize, kk:u8){
        if self.register[x] == kk{
            self.pc += 2;
        }
    }

    pub fn _4xkk(&mut self, x:usize, kk:u8){
        if self.register[x] != kk{
            self.pc += 2;
        }
    }

    pub fn _5xy0(&mut self, x:usize, y:usize){
        if self.register[x] == self.register[y]{
            self.pc += 2;
        }
    }

    pub fn _6xkk(&mut self, x:usize, kk:u8){
        self.register[x] = kk;
    }

    pub fn _7xkk(&mut self, x:usize, kk:u8){
        self.register[x] = self.register[x].wrapping_add(kk);
    }

    pub fn _8xy0(&mut self, x:usize, y:usize){
        self.register[x] = self.register[y];
    }

    pub fn _8xy1(&mut self, x:usize, y:usize){
        self.register[x] |= self.register[y];
    }

    pub fn _8xy2(&mut self, x:usize, y:usize){
        self.register[x] &= self.register[y];
    }

    pub fn _8xy3(&mut self, x:usize, y:usize){
        self.register[x] ^= self.register[y];
    }

    pub fn _8xy4(&mut self, x:usize, y:usize){
        if self.register[x] + self.register[y] > 0xff{
            self.register[0xf] = 1;
        }
        else{self.register[0xf] = 0;}
        self.register[x] = (self.register[x] + self.register[y]) & 0xff;
    }

    pub fn _8xy5(&mut self, x:usize, y:usize){
        self.register[0xf] = if self.register[x] > self.register[y]{1} else {0};
        self.register[x] -= self.register[y];
    }

    pub fn _8xy6(&mut self, x:usize, y:usize){
        self.register[0xf] = if self.register[x] & 0x1 == 1{1} else {0};
        self.register[x] >>= 1; //divided by 2
    }

    pub fn _8xy7(&mut self, x:usize, y:usize){
        self.register[0xf] = if self.register[y] > self.register[x]{1} else {0};
        self.register[x] = self.register[y] - self.register[x];
    }

    pub fn _8xye(&mut self, x:usize, y:usize){
        self.register[0xf] = if self.register[x] & 0x80 == 0x80{1} else {0};
        self.register[x] <<= 1; //multiply by 2
    }

    pub fn _9xy0(&mut self, x:usize, y:usize){
        self.pc+= if self.register[x] != self.register[y]{2} else {0};
    }

    pub fn _annn(&mut self, nnn:u16){
        self.i = nnn;
    }

    pub fn _bnnn(&mut self, nnn:u16){
        self.pc = (nnn + (self.register[0] as u16)) as u16;
    }

    pub fn _cxkk(&mut self, x:usize, kk:u8){
        self.register[x] = rand::thread_rng().gen_range(0..255) & kk;
    }

    //not confident on this opcode
    pub fn _dxyn(&mut self, x:usize, y:usize, n:u8){ 

        let xcord = self.register[x] as usize % 64;
        let ycord = self.register[y] as usize % 32;
        self.register[0xf] = 0;

        for p in 0..n{
            let px = self.memory[(self.i + p as u16) as usize];

            for q in 0..8{
                let screen = self.display[ycord + p as usize][xcord + q as usize];
                if ((px >>q)& 0x1) ==1 && screen ==1 {
                    self.register[0xf] = 1;
                }
                let _x = (xcord + q as usize) % 64;
                let _y = (ycord + p as usize) % 32;
                self.display[_y][_x] ^= ((px >> q) & 0x1);
            }
        }

        // alternate way 
        /* 
    let xcord = x as usize % 64;
    let ycord = y as usize % 32;

    self.register[0xf] = 0;

    for nrows in 0..n {
        let sprite = self.memory[(self.i + nrows as u16) as usize];
        for _8cols in 0..8 {
            if (sprite & (0x80 >> _8cols)) != 0 {
                let px = xcord + _8cols %64;
                let py = ycord + (nrows as usize) % 32;
                
                if self.display[py][px] == 1 {
                    self.register[0xf] = 1;
                }
                self.display[py][px] = if self.display[py][px] == 1 {0} else {1};
            }
        }
    }
    */
    }


    pub fn _ex9e(&mut self, x:usize){
        self.pc += if self.keypad[self.register[x] as usize]{2} else {0};
    }

    pub fn _exa1(&mut self, x:usize){
        self.pc += if !self.keypad[self.register[x] as usize]{2} else {0};
    }

    pub fn _fx07(&mut self, x:usize){
        self.register[x] = self.dt;
    }

    pub fn _fx0a(&mut self, x:usize){
        for i in 0..16{
            if self.keypad[i]{
                self.register[x] = i as u8;
            }
        }
    }

    pub fn _fx15(&mut self, x:usize){
        self.dt = self.register[x];
    }

    pub fn _fx18(&mut self, x:usize){
        self.st = self.register[x];
    }

    pub fn _fx1e(&mut self, x:usize){
        self.i += self.register[x] as u16;
    }

    pub fn _fx29(&mut self, x:usize){
        self.i = (self.register[x] as u16) * 5; //sprites are 5 bytes
    }

    pub fn _fx33(&mut self, x:usize){
        self.memory[self.i as usize] = self.register[x] / 100;
        self.memory[(self.i + 1) as usize] = (self.register[x] / 10) % 10;
        self.memory[(self.i + 2) as usize] = self.register[x] % 10;
    }

    pub fn _fx55(&mut self, x:usize){
        for q in 0..x{
            self.memory[self.i as usize + q] = self.register[q];
        }
    }

    pub fn _fx65(&mut self, x:usize){
        for q in 0..x{
            self.register[q] = self.memory[self.i as usize + q];
        }
    }
    
}


    
