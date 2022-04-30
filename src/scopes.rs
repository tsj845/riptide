// use crate::statics::*;
use crate::tokens::*;
use std::collections::HashMap;

pub enum Flags {
    Global = 0, // in global scope
    Readonly = 1, // readonly
    Private = 2, // private
    Exposed = 3, // exposed from library
    Protected = 5, // managed by system
    System = 6, // system object
    Active = 7, // this flag is valid
}

#[derive(Clone, Copy)]
pub struct Flag {
    pub value : u8,
}

impl Flag {
    pub fn null () -> Flag {
        Flag {
            value : 0,
        }
    }
    pub fn new () -> Flag {
        Flag {
            value : 0b1u8 << 7,
        }
    }
    pub fn bits (&self) -> String {
        let mut f : String = String::new();
        for i in [0,1,2,3,4,5,6,7] {
            f.push(char::from_digit(((self.value & 1u8 << i as u8) >> i as u8) as u32, 10).unwrap());
        }
        unsafe {
            f.as_bytes_mut().reverse();
        }
        return f;
    }
    pub fn or (self, other : Flag) -> Flag {
        if self.valid() {
            return self;
        }
        return other;
    }
    pub fn set (&mut self, target : Flags, value : bool) {
        match value {
            false => {self.value = self.value & !(1u8 << target as u8)},
            true => {self.value = self.value | (1u8 << target as u8)}
        }
    }
    pub fn valid (&self) -> bool {
        return self.value & (1u8 << 7) == 128;
    }
    pub fn matches (&self, target : Flags, value : bool) -> bool {
        return self.valid() && (self.value & (1u8 << target as u8) > 0) == value;
    }
    pub fn matching (&self, targets : Vec<Flags>, values : Vec<bool>) -> bool {
        if !self.valid() {
            return false;
        }
        for (target, value) in targets.into_iter().zip(values.into_iter()) {
            if !self.matches(target, value) {
                return false;
            }
        }
        return true;
    }
}

// handles scopes
pub struct ScopeManager {
    // complex objects
    pub heap : HashMap<String, Token>,
    // normal scopes
    pub scopes : Vec<HashMap<String, (Token, Flag)>>,
    // quick accessor
    scope_count : usize,
}

impl ScopeManager {
    pub fn new () -> ScopeManager {
        ScopeManager {
            heap : HashMap::new(),
            scopes : vec![HashMap::new(), HashMap::new()],
            scope_count : 2,
        }
    }
    pub fn get_flag (&self, name : &str) -> Flag {
        if self.scopes[self.scope_count-1].contains_key(name) {
            return self.scopes[self.scope_count-1].get(name).unwrap().1;
        }
        if self.scopes[0].contains_key(name) {
            return self.scopes[0].get(name).unwrap().1;
        }
        return Flag::null();
    }
    fn get_from (&self, scope : usize, name : String) -> Token {
        if self.scopes[scope].contains_key(&name) {
            return self.scopes[scope].get(&name).unwrap().0.clone();
        }
        return void_token();
    }
    pub fn flag (&mut self, name : &str, target : Flags, value : bool) {
        let mut f : Flag = self.get_flag(name);
        // println!("FB: {}, {}", f.bits(), f.valid());
        if f.valid() {
            let log : bool = f.matches(Flags::Global, true);
            f.set(target, value);
            let v = self.scopes[match log {true => 0, _ => self.scope_count-1}].get_mut(name).unwrap();
            v.1 = f;
            // println!("{}, {}", f.bits(), v.1.bits());
        }
    }
    pub fn get (&self, name : String) -> Token {
        if self.get_flag(&name).matches(Flags::Global, true) {
            return self.get_from(0, name);
        }
        return self.get_from(self.scope_count-1, name);
    }
    pub fn set (&mut self, name : String, value : Token) {
        let f : Flag = self.get_flag(&name);
        if f.matches(Flags::System, true) {
            return;
        }
        if f.matches(Flags::Global, true) {
            self.scopes[0].insert(name, (value, f));
            return;
        }
        self.scopes[self.scope_count-1].insert(name, (value, f.or(Flag::new())));
    }
}