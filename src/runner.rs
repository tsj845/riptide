#[allow(unused_imports)]
use crate::statics::*;
use crate::tokens::*;
use crate::scopes::*;
use crate::bindings::*;

// executes code
#[allow(dead_code)]
pub struct Runner {
    scopes : ScopeManager,
    binds : Builtins,
}

impl Runner {
    pub fn new () -> Runner {
        Runner {
            scopes : ScopeManager::new(),
            binds : Builtins::new(),
        }
    }
    pub fn run (&mut self, tokens : Vec<Token>) -> Result<String, String> {
        println!("{:#?}", tokens);
        // self.scopes.set(String::from("test"), Token::new(DAT, 0, TokenData::String(String::from("xyz"))));
        // println!("{:?}", self.scopes.get(String::from("test")));
        // self.scopes.flag("test", Flags::System, true);
        // println!("{}", self.scopes.get_flag("test").bits());
        return Ok(String::from("placeholder"));
    }
}