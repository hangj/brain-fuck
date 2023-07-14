//! Brainfuck commands:
//!
//! >  increment pointer
//! <  decrement pointer
//! +  increment current byte
//! -  decrement current byte
//! .  write current byte to stdout
//! ,  read byte from stdin and store value in current byte
//! [  jump past matching ] if current byte is zero
//! ]  jump back to matching [ if current byte is nonzero
//! any other character ignore, treat as comment

use literal_enum::LiteralEnum;

#[derive(LiteralEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    /// increment pointer
    #[lit = b'>']
    IncrementPointer, // >
    /// decrement pointer
    #[lit = b'<']
    DecrementPointer, // <
    /// increment current byte
    #[lit = b'+']
    IncrementByte, // +
    /// decrement current byte
    #[lit = b'-']
    DecrementByte, // -
    /// write current byte to stdout
    #[lit = b'.']
    WriteByte, // .
    /// read byte from stdin and store value in current byte
    #[lit = b',']
    ReadByte, // ,
    /// jump past matching ] if current byte is zero
    #[lit = b'[']
    LoopStart, // [
    /// jump back to matching [ if current byte is nonzero
    #[lit = b']']
    LoopEnd, // ]
}
