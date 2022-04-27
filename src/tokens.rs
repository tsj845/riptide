// defines tokens
use std::collections::HashMap;

pub enum TokenData {
    String(String),
    List(Vec<Token>),
    Dict(HashMap<String, Token>),
    Int(i64),
    Float(f64),
    Bool(bool),
    Function((Vec<Token>, Vec<Token>)),
    Void
}

pub struct Token {
    id : u8,
    value : TokenData,
    // value : String,
    // list : Option<Vec<Token>>,
    // dict : Option<HashMap<String, Token>>,
}