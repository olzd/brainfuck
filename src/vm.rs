use std::io;
use std::io::{Read, Write};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Token {
    ShiftL,
    ShiftR,
    Incr,
    Decr,
    Write,
    Read,
    Jez,
    Jnez,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Ir {
    ShiftL(usize),
    ShiftR(usize),
    Incr(usize),
    Decr(usize),
    Write(usize),
    Read(usize),
    Jez(usize),
    Jnez(usize),
}

impl From<(Token, usize)> for Ir {
    fn from((token, rle): (Token, usize)) -> Self {
        (match token {
            Token::ShiftL => Ir::ShiftL,
            Token::ShiftR => Ir::ShiftR,
            Token::Incr => Ir::Incr,
            Token::Decr => Ir::Decr,
            Token::Write => Ir::Write,
            Token::Read => Ir::Read,
            Token::Jez => Ir::Jez,
            Token::Jnez => Ir::Jnez,
        })(rle)
    }
}

pub struct Vm {
    tape: Vec<u8>,
    program: Vec<Ir>,
    pc: usize,
    dp: usize,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            tape: vec![0; 30_000],
            program: Vec::new(),
            pc: 0,
            dp: 0,
        }
    }
}

impl Vm {
    pub fn new(tape_size: usize) -> Self {
        Self {
            tape: vec![0; tape_size],
            program: Vec::new(),
            pc: 0,
            dp: 0,
        }
    }

    pub fn load(&mut self, input: &str) {
        let tokens = Vm::parse(input);
        self.program = Vm::compile(&tokens);
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            self.exec_opt();
        }
    }

    #[inline]
    fn exec_opt(&mut self) {
        match self.program[self.pc] {
            Ir::ShiftR(n) => {
                self.dp += n;
            }
            Ir::ShiftL(n) => {
                self.dp -= n;
            }
            Ir::Incr(n) => {
                for _ in 0..n {
                    self.tape[self.dp] = self.tape[self.dp].wrapping_add(1);
                }
            }
            Ir::Decr(n) => {
                for _ in 0..n {
                    self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1);
                }
            }
            Ir::Write(n) => {
                for _ in 0..n {
                    io::stdout()
                        .write_all(&self.tape[self.dp..=self.dp])
                        .expect("Unable to write byte to stdout");
                }
            }
            Ir::Read(n) => {
                io::stdin()
                    .read_exact(&mut self.tape[self.dp..self.dp + n])
                    .expect("Unable to read byte from stdin");
            }
            Ir::Jez(addr) => {
                if self.tape[self.dp] == 0 {
                    self.pc = addr;
                }
            }
            Ir::Jnez(addr) => {
                if self.tape[self.dp] != 0 {
                    self.pc = addr;
                }
            }
        };
        self.pc += 1;
    }

    #[inline]
    fn parse(input: &str) -> Vec<Token> {
        input
            .chars()
            .filter_map(|b| match b {
                '>' => Some(Token::ShiftR),
                '<' => Some(Token::ShiftL),
                '+' => Some(Token::Incr),
                '-' => Some(Token::Decr),
                '.' => Some(Token::Write),
                ',' => Some(Token::Read),
                '[' => Some(Token::Jez),
                ']' => Some(Token::Jnez),
                _ => None,
            })
            .collect()
    }

    #[inline]
    fn compile(tokens: &Vec<Token>) -> Vec<Ir> {
        // run length encoding
        // ignore the jumps as they cannot be compressed
        let mut program = tokens
            .group_by(|x, y| *x != Token::Jez && *x != Token::Jnez && x == y)
            .map(|rle| Ir::from((rle[0], rle.len())))
            .collect();

        // last pass to fill the jump addr
        Vm::fill_jmp_addr(&mut program);

        program
    }

    #[inline]
    fn fill_jmp_addr(prog: &mut Vec<Ir>) {
        let mut stack = Vec::new();
        let mut i = 0;
        while i < prog.len() {
            match prog[i] {
                Ir::Jez(_) => {
                    stack.push(i);
                }
                Ir::Jnez(ref mut n) => {
                    let addr = stack.pop().unwrap();
                    *n = addr;
                    prog[addr] = Ir::Jez(i);
                }
                _ => (),
            }
            i += 1;
        }
    }
}
