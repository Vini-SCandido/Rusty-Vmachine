pub struct Stackvm {
    pc: usize, //program counter
    st: usize, // stack pointer
    memory: Vec<i32>,
    typ: i32,
    dat: i32,
    run: bool,
}

impl Stackvm {
    fn get_type(instruction: i32) -> i32 {
        let header = instruction >> 30;
        return header;
    }

    fn get_data(instruction: i32) -> i32 {
        let mut data = 0x3fffffff;
        data = data & instruction;
        return data;
    }

    fn fetch(&mut self) {
        self.pc += 1;
    }

    fn decode(&mut self) {
        self.typ = Stackvm::get_type(self.memory[self.pc]);
        self.dat = Stackvm::get_data(self.memory[self.pc]);

    }

    fn execute(&mut self) {
        if self.typ == 0 || self.typ == 2 {
            self.st += 1;
            self.memory[self.st] = self.dat;
        } else {
            self.do_primitive();
        }
    }

    fn do_primitive(&mut self) {
        match self.dat {
            0 => {
                println!("Hlt: ---- ----");
                self.run = false;
            },
            1 => {
                println!("Add: {:>4} {:>4}", self.memory[self.st - 1], self.memory[self.st]);
                self.memory[self.st - 1] = self.memory[self.st - 1] + self.memory[self.st];
                self.st -= 1;
            },
            2 => {
                println!("Sub: {:>4} {:>4}", self.memory[self.st - 1], self.memory[self.st]);
                self.memory[self.st - 1] = self.memory[self.st - 1] - self.memory[self.st];
                self.st -= 1;
            },
            3 => {
                println!("Mul: {:>4} {:>4}", self.memory[self.st - 1], self.memory[self.st]);
                self.memory[self.st - 1] = self.memory[self.st - 1] * self.memory[self.st];
                self.st -= 1;
            },
            4 => {
                println!("Div: {:>4} {:>4}", self.memory[self.st - 1], self.memory[self.st]);
                self.memory[self.st - 1] = self.memory[self.st - 1] / self.memory[self.st];
                self.st -= 1;
            },
            _ => ()
        }
    }

    pub fn new(memory: Vec<i32>) -> Self{ // constructor
        Stackvm {
            memory: memory,
            pc: 100,
            st: 0,
            dat: 0,
            typ: 0,
            run: true
        }
    }

    pub fn run(&mut self) {
        self.pc -= 1;

        while self.run {
            self.fetch();
            self.decode();
            self.execute();
            println!("Tos: {:>4}", self.memory[self.st]);
        }
    }

    pub fn load_program(&mut self, program: Vec<i32>) {
        let end = program.len();
        for i in 0..end {
            self.memory[self.pc + i] = program[i];
        }
    }
}
