// constant definitions

pub const ERROR_RED : &str = "\x1b[38;2;250;100;50m";
pub const WARN_ORANGE : &str = "\x1b[38;2;200;175;0m";
pub const GOOD_GREEN : &str = "\x1b[38;2;0;215;50m";
pub const TEXT_NORMAL : &str = "\x1b[0m";

pub fn test () {
    println!("{}error red {} warning orange {} good green {} normal text", ERROR_RED, WARN_ORANGE, GOOD_GREEN, TEXT_NORMAL);
}