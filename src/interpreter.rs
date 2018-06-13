use std::fs;


pub struct Interpreter {
    code: Vec<char>,
    mem: Vec<u8>,
    out_data: Vec<u8>,
    in_data: Vec<u8>,
    ic: usize,
    mc: usize,
}

impl Interpreter  {
    pub fn new(path: &str, in_data: &str) -> Self {
        Interpreter {
            code: fs::read_to_string(path).unwrap_or_else(|e| panic!(e)).chars().collect::<Vec<char>>(),
            mem: [0u8; 30000].to_vec(),
            out_data: Vec::<u8>::new(),
            in_data: if in_data.is_ascii() { String::from(in_data).chars().map(|x| x as u8).collect::<Vec<u8>>() } else { Vec::<u8>::new() },
            ic: 0,
            mc: 0,
        }
    }

    pub fn run(&mut self) {
        let mut bracket_counter: usize = 0;
        while self.ic != self.code.len() - 1 {
            match self.code[self.ic] {
                '>' => self.mc += 1,
                '<' => self.mc -= 1,
                '+' => self.mem[self.mc] += 1,
                '-' => self.mem[self.mc] -= 1,
                '.' => self.out_data.push(self.mem[self.mc]),
                ',' => self.mem[self.mc] = self.in_data.pop().unwrap_or_else(|| panic!("expected input!")),
                '[' => {
                    if self.mem[self.mc] == 0 {
                        self.ic += 1;
                        while bracket_counter > 0 || self.code[self.ic] != ']' {
                            if self.code[self.ic] == '[' { bracket_counter += 1; }
                            if self.code[self.ic] == ']' { bracket_counter -= 1; }
                            self.ic += 1;
                        }
                    }
                },
                ']' => {
                    if self.mem[self.mc] != 0 {
                        self.ic -= 1;
                        while bracket_counter > 0 || self.code[self.ic] != '[' {
                            if self.code[self.ic] == ']' { bracket_counter += 1; }
                            if self.code[self.ic] == '[' { bracket_counter -= 1; }
                            self.ic -= 1;
                        }
                        self.ic -= 1;
                    }
                },
                _ => panic!("Unsuppoerted token!"),
            };
            self.ic += 1;
        };
        println!("{}", String::from_utf8_lossy(self.out_data.as_slice()));
    }
}
