// defines tokens
use std::collections::HashMap;

/**
 * Token is used when a token could be pulled from TokenData
 * otherwise TokenData is used
 */

pub enum TokenData {
    String(String),
    List(Vec<Token>),
    Dict(HashMap<String, Token>),
    Int(i64),
    Float(f64),
    Bool(bool),
    // args, return type, code
    Function((Vec<TokenData::Pair>, TokenData, Vec<Token>)),
    // name, statics, instance
    Class((String, HashMap<String, Token>, HashMap<String, Token>)),
    // name, type, default
    Pair((String, TokenData, TokenData)),
    Void
}

pub struct Token {
    id : u8,
    value : TokenData,
    // value : String,
    // list : Option<Vec<Token>>,
    // dict : Option<HashMap<String, Token>>,
}