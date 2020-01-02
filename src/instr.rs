use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug,Serialize,Deserialize)]
pub enum Value {
    Imm (u32),
    Reg (usize),
}

impl Value {
    pub fn valid(self:&Self) -> bool {
        match self {
            Value::Imm (_) => true,
            Value::Reg (r) => r <= &4,
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Inst {
    ADD (usize, Value),
    SUB (usize, Value),
    MUL (usize, Value),
    DIV (usize, Value),
    AND (usize, Value),
    OR  (usize, Value),
    XOR (usize, Value),
    NOT (usize),
    MOV (usize, Value),
    RSFT (usize, Value),
    LSFT (usize, Value),
    CALL (String),
    RET,
    EQ (Value, Value),
    LBL (String),
    JMP (String),
    TJMP (String),
    FJMP (String),
    PUSH (Value),
    POP (usize),
    HLT,
    DISPNUM (usize),
}
use Inst::*;

impl Inst {

    pub fn valid(self: &Self, labels: &HashMap<String,usize>) -> bool {
        match self {
            ADD (r, v) => r <= &4 && v.valid(),
            SUB (r, v) => r <= &4 && v.valid(),
            MUL (r, v) => r <= &4 && v.valid(),
            DIV (r, v) => r <= &4 && v.valid(),
            AND (r, v) => r <= &4 && v.valid(),
            OR  (r, v) => r <= &4 && v.valid(),
            XOR (r, v) => r <= &4 && v.valid(),
            NOT (r) => r <= &4,
            MOV (r, v) => r <= &4 && v.valid(),
            RSFT (r, v) => r <= &4 && v.valid(),
            LSFT (r, v) => r <= &4 && v.valid(),
            CALL (s) => labels.get(s).is_some(),
            RET => true,
            EQ (v1, v2) => v1.valid() && v2.valid(),
            LBL (s) => true,
            JMP (s) => labels.get(s).is_some(),
            TJMP (s) => labels.get(s).is_some(),
            FJMP (s) => labels.get(s).is_some(),
            PUSH (v) => v.valid(),
            POP (r) => r <= &4,
            HLT => true,
            DISPNUM (r) => r <= &4,
        }

    }

}

   
#[derive(Debug,Serialize,Deserialize)]
pub struct Program {
    pub code: Vec<Inst>,
    pub labels: HashMap<String, usize>
}

impl Program {
    pub fn new(code: Vec<Inst>) -> Self {
        let mut labels = HashMap::new();
        for (ln,i) in code.iter().enumerate() {
            match i {
                 Inst::LBL (l) => labels.insert(l.to_string(), ln),
                 _ => None 
            };
        }

        Program { code : code, labels: labels }
    }

    pub fn valid(self: &Self) -> bool {
        self.code.iter().all(|i| i.valid(&self.labels))

    }


}
