// defines tokens
use std::collections::HashMap;

pub enum TokenData {
    String(String),
    List(Vec<Token>),
    Dict(HashMap<String, Token>),
    Number(i64),
    Float(f64),
    Null
}

pub struct Token {
    id : u8,
    value : TokenData,
    // value : String,
    // list : Option<Vec<Token>>,
    // dict : Option<HashMap<String, Token>>,
}