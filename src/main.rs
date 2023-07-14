//! Overview of brainfuck:
//!
//! Brainfuck programs have an implicit pointer, "the pointer", which is free to move around an array of 30k unsigned bytes,
//! all initially set to 0.
//! Decrementing the pointer below 0 or incrementing the pointer above 30k is undefined behavior.
//! Decrementing a byte below 0 or incrementing a byte above 255 wraps its value.
//! The newline character is read and written as the value 10.
//! The EOF (End of File) character is read as the value 0.
//!

mod cmd;
use std::io::{ErrorKind, Read, Write};

use cmd::Command;

const NEWLINE: u8 = 10;
const EOF: u8 = 0;

struct State {
    memory: Vec<u8>,
    pointer: usize,
}

impl State {
    fn new() -> Self {
        let mut memory = Vec::with_capacity(30 * 1024);
        memory.resize(30 * 1024, 0);
        Self { memory, pointer: 0 }
    }

    fn run_cmd(&mut self, cmd: &Command, code: &Code) -> usize {
        match cmd {
            Command::IncrementPointer => self.pointer += 1,
            Command::DecrementPointer => self.pointer -= 1,
            Command::IncrementByte => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
            }
            Command::DecrementByte => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
            }
            Command::WriteByte => {
                if self.memory[self.pointer] == NEWLINE {
                    println!();
                    return code.pointer + 1;
                }

                std::io::stdout()
                    .write_all(&self.memory[self.pointer..self.pointer + 1])
                    .expect("write error");
            }
            Command::ReadByte => {
                let mut buf = [0u8; 1];
                match std::io::stdin().read_exact(&mut buf) {
                    Ok(_) => {}
                    Err(err) => match err.kind() {
                        ErrorKind::UnexpectedEof => {
                            buf[0] = EOF;
                        }
                        _ => {
                            panic!("{}", err.to_string());
                        }
                    },
                }

                if buf[0] == b'\n' || buf[0] == b'\r' {
                    buf[0] = NEWLINE;
                }
                self.memory[self.pointer] = buf[0];
            }
            Command::LoopStart => {
                // find the coupled `]`, and jump to it
                if self.memory[self.pointer] == 0 {
                    let mut sum = 0;
                    for i in code.pointer..code.sequence.len() {
                        if code.sequence[i] == Command::LoopStart {
                            sum += 1;
                        }
                        if code.sequence[i] == Command::LoopEnd {
                            if sum == 0 {
                                return i + 1;
                            } else {
                                sum -= 1;
                            }
                        }
                    }
                }
            }
            Command::LoopEnd => {
                // find the previous `[`, and jump to it
                if self.memory[self.pointer] != 0 {
                    let mut sum = 0;
                    for i in (0..code.pointer).rev() {
                        if code.sequence[i] == Command::LoopEnd {
                            sum += 1;
                        }
                        if code.sequence[i] == Command::LoopStart {
                            if sum == 0 {
                                return i + 1;
                            } else {
                                sum -= 1;
                            }
                        }
                    }
                }
            }
        }

        code.pointer + 1
    }
}

struct Code {
    sequence: Vec<Command>,
    pointer: usize,
}

impl Code {
    fn get_cmd(&self) -> Option<&Command> {
        self.sequence.get(self.pointer)
    }
}

fn execute(input: &[u8]) {
    let mut state = State::new();

    let cmd_sequence = input
        .iter()
        .filter_map(|byte| Command::try_from(*byte).ok())
        .collect::<Vec<_>>();

    let mut code = Code {
        sequence: cmd_sequence,
        pointer: 0,
    };

    while let Some(cmd) = code.get_cmd() {
        code.pointer = state.run_cmd(cmd, &code);
    }
}

fn main() {
    let input = b"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let input = b">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]<.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.";
    execute(input);
}
