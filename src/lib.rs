extern crate regex;
extern crate lazy_static;
extern crate terminal_size;
extern crate rand;
// controller is the only file meant to be exposed to outside code as it handles everything
pub mod controller;
mod statics;
mod tokens;
mod tokenizer;
mod runner;
mod scopes;
mod bindings;