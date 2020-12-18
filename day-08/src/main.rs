use commons::io::load_file_lines;
use std::collections::HashSet;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
enum Instruction {
    Accumulate(i32),
    Jump(i32),
    Nop(i32),
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),
    #[error("Argument missing for instruction: {0}")]
    MissingArgument(String),
    #[error("Argument invalid for instruction: {0}")]
    InvalidArgument(String),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let ins_str = it.next().unwrap();
        let arg = match it.next() {
            Some(arg_str) => arg_str
                .parse()
                .map_err(|_| ParseError::InvalidArgument(s.to_string())),
            None => Err(ParseError::MissingArgument(s.to_string())),
        }?;
        let ins = match ins_str {
            "acc" => Instruction::Accumulate(arg),
            "jmp" => Instruction::Jump(arg),
            "nop" => Instruction::Nop(arg),
            _ => return Err(ParseError::UnknownInstruction(s.to_string())),
        };

        Ok(ins)
    }
}

#[derive(Debug)]
struct Machine<'a> {
    pc: usize,
    acc: i32,
    halted: bool,
    instructions: &'a Vec<Instruction>,
}

impl<'a> Machine<'a> {
    #[allow(clippy::ptr_arg)]
    pub fn from_instructions(instructions: &'a Vec<Instruction>) -> Machine<'a> {
        Machine {
            pc: 0,
            acc: 0,
            halted: false,
            instructions,
        }
    }

    pub fn step(&mut self) {
        if self.pc >= self.instructions.len() {
            self.halted = true;
        }
        if self.halted {
            return;
        }

        let ins = &self.instructions[self.pc];
        let mut pc_inc = true;
        match ins {
            Instruction::Accumulate(i) => {
                self.acc += i;
            }
            Instruction::Jump(i) => {
                pc_inc = false;
                let int_ipc = (self.pc as i32) + i;
                self.pc = int_ipc as usize;
            }
            Instruction::Nop(_) => {}
        }

        if pc_inc {
            self.pc += 1;
        }
    }

    fn run_until_loop(&mut self) -> Vec<usize> {
        let mut seen_instructions = HashSet::new();
        let mut pc_history = Vec::new();
        while !self.halted {
            if seen_instructions.contains(&self.pc) {
                break;
            }
            seen_instructions.insert(self.pc);
            pc_history.push(self.pc);
            self.step();
        }

        pc_history
    }
}

fn main() {
    let instructions: Vec<Instruction> = load_file_lines("input.txt").map(|x| x.unwrap()).collect();
    let mut machine = Machine::from_instructions(&instructions);

    let instruction_idxs = machine.run_until_loop();
    println!("{}", machine.acc);

    for idx_ref in instruction_idxs.iter().rev() {
        let idx = *idx_ref;
        let mut new_instructions = instructions.clone();
        // Flip the next instruction
        match new_instructions.get_mut(idx) {
            Some(ins) => {
                if let Instruction::Nop(i) = *ins {
                    *ins = Instruction::Jump(i);
                } else if let Instruction::Jump(i) = *ins {
                    *ins = Instruction::Nop(i);
                } else {
                    continue; // No need to try this machine, as we didn't change anything
                }
            }
            None => panic!("Ran off the end of the instructions array"),
        }
        let mut maybe_fixed_machine = Machine::from_instructions(&new_instructions);
        maybe_fixed_machine.run_until_loop();
        if maybe_fixed_machine.halted {
            println!("{}", maybe_fixed_machine.acc);
            break;
        }
    }
}
