// constant definitions
#![allow(dead_code)]

pub const ERROR_RED : &str = "\x1b[38;2;250;100;50m";
pub const WARN_ORANGE : &str = "\x1b[38;2;200;175;0m";
pub const GOOD_GREEN : &str = "\x1b[38;2;0;215;50m";
pub const TEXT_NORMAL : &str = "\x1b[0m";

pub const KEY : u8 = 0; // keyword
pub const CTL : u8 = 1; // control flow
pub const DAT : u8 = 2; // data
pub const REF : u8 = 3; // reference
pub const PTR : u8 = 4; // pointer
pub const OPS : u8 = 5; // operator
pub const SYM : u8 = 6; // symbol
pub const INV : u8 = 7; // invalid
pub const NLN : u8 = 8; // line seperator
pub const TYP : u8 = 9; // type

pub const IDMAP : [&str; 10] = ["KEY", "CTL", "DAT", "REF", "PTR", "OPS", "SYM", "INV", "NLN", "TYP"];

// regex patterns
pub const NUMBER_RE_PAT : &str = r"^(0b[01]+|0x[0-9a-f]+|[0-9]+(\.[0-9]+)?)";
pub const DECI_RE_PAT : &str = r"^[0-9]+$";
pub const OP_RE_PAT : &str = r"^([<>|&+*=-]{1,2}|([+*/%!-]=?)|\$)$";
pub const KEY_RE_PAT : &str = r"\b(func|return|halt|raise|try|catch|final|if|else|for|while|in|class|static|private|readonly|self|var|import|from|as|enum|undefined)\b";
// pub const TYPE_RE_PAT : &str = r"\b(int|str|uint|float|number|bool|void|list|dict|any)\b";

// pub fn test () {
//     println!("{}error red {} warning orange {} good green {} normal text", ERROR_RED, WARN_ORANGE, GOOD_GREEN, TEXT_NORMAL);
// }