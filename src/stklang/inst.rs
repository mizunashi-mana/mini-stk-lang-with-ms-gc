use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub insts: Vec<Inst>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Inst {
    Push,
    Pop,
    NewInt { value: i32 },
    WriteInt { value: i32 },
    WriteAdd,
    NewProd,
    WriteFst,
    WriteSnd,
    Print,
    Gc,
}
