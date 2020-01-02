use crate::instr::*;
use crate::instr::Inst::*;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidPointer,
    InvalidRegister,
    StackOverflow,
    StackUnderflow,
    MalformedLabel,
    DivByZero,
}
use RuntimeError::*;

type ExecutionResult = Result<(), (RuntimeError, usize)>;
type WordResult = Result<u32, (RuntimeError, usize)>;

#[derive(Debug)]
pub struct VM {
    p : Program,
    regs : [u32; 4],
    pc : usize,
    sp : usize,
    eq: bool,
    rem: u32, 
    mem: Vec<u32>,
    hlt : bool,
}

impl VM {
    pub fn new(p : Program) -> Self {
        VM {
            p : p,
            regs : [0; 4],
            pc : 0,
            sp : 0,
            eq : false,
            rem : 0,
            mem: vec![0; 100],
            hlt : false }
    }

    pub fn run_program(self: &mut Self) -> ExecutionResult {
        while !self.hlt {
            self.execute()?;
        }
        Ok(())
    }

    pub fn execute(self: &mut Self) -> ExecutionResult {
        let i = match self.p.code.get(self.pc) {
            Some(i) => i,
            None => return Err((InvalidPointer,self.pc))
        };
        match i {
            ADD (u,v) => { self.regs[*u] += self.loadval(v); self.pc += 1; },
            SUB (u,v) => { self.regs[*u] -= self.loadval(v); self.pc += 1; },
            MUL (u,v) => { self.regs[*u] *= self.loadval(v); self.pc += 1;},
            DIV (u,v) => { let t = self.regs[*u]; 
                            let denom = self.loadval(v);
                            if denom == 0 { return Err((DivByZero,self.pc)); }
                            self.regs[*u] /= denom;
                            self.rem = t % denom;
                            self.pc += 1; }
            AND (u,v) => { self.regs[*u] &= self.loadval(v); self.pc += 1;},
            OR  (u,v) => { self.regs[*u] |= self.loadval(v); self.pc += 1;},
            XOR (u,v) => { self.regs[*u] ^= self.loadval(v); self.pc += 1;},
            NOT (u) => { self.regs[*u] = ! self.regs[*u]; self.pc += 1;},
            MOV (u,v) => { self.regs[*u] = self.loadval(v); self.pc += 1;},
            RSFT (u,v) => { self.regs[*u] >>= self.loadval(v); self.pc += 1;},
            LSFT (u,v) => { self.regs[*u] <<= self.loadval(v); self.pc += 1;},
            CALL (lbl) => {  
                            let orig = self.pc as u32 + 1;
                            let dest = self.p.labels.get(lbl);
                            let dest = match dest {
                                Some(idx) => *idx,
                                None => return Err((MalformedLabel, self.pc)),
                            };
                            self.pc = dest; 
                            self.push(orig as u32)?; },
            RET => { 
                let retaddr = self.pop()?;
                self.pc = retaddr as usize;
            },
            EQ (v1,v2) => {
                let v1 = self.loadval(v1);
                let v2 = self.loadval(v2);
                self.eq = v1 == v2;
                self.pc += 1;
            },
            LBL (_) => { self.pc += 1; },
            JMP (s) => { let dest = self.opterr(self.p.labels.get(s), MalformedLabel)?;
                          self.pc = *dest; },
            TJMP (s) => { if self.eq { 
                                let dest = self.opterr(self.p.labels.get(s), MalformedLabel)?;
                                self.pc = *dest;
                            } else {
                                self.pc += 1;
                            }},
            FJMP (s) => { if ! self.eq { 
                                let dest = self.opterr(self.p.labels.get(s), MalformedLabel)?;
                                self.pc = *dest;
                            } else {
                                self.pc += 1;
                            }},
            PUSH (v) => { self.push(self.loadval(v))?; self.pc += 1;},
            POP (r) => { let r = *r; 
                        self.regs[r] = self.pop()?; self.pc += 1; }, 
            HLT => { self.hlt = true; },
            DISPNUM (r) => { 
                            let v = self.regs[*r]; 
                            print!("{}", v);
                            self.pc += 1;
            },
        };
        Ok(())
    }

    fn push(self: &mut Self, w : u32) -> ExecutionResult {
        if self.sp < self.mem.len() {
            self.mem[self.sp] = w;
            self.sp += 1;
            Ok(())
        } else {
            Err((StackOverflow, self.pc))
        }
    } 

    fn pop(self: &mut Self) -> WordResult {
        if self.sp == 0 {
            Err((StackUnderflow, self.pc))
        } else {
            self.sp -= 1;
            Ok(self.mem[self.sp])
        }
    }

    fn opterr<T>(self: &Self, v: Option<T>, e : RuntimeError) -> Result<T, (RuntimeError,usize)> {
        match v { 
            Some(v) => Ok(v),
            None => Err((e,self.pc)) 
        }
    }

    fn loadval(self: &Self, v : &Value) -> u32 {
        match v {
            Value::Imm (i) => *i,
            Value::Reg (r) => self.regs[*r]
        }
    }

}

