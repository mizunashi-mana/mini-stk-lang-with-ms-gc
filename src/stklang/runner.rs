use std::io::Result;
use crate::stklang::inst;

pub trait Runner {
    fn push(&mut self) -> Result<()>;
    fn pop(&mut self) -> Result<()>;
    fn new_int(&mut self, value: i32) -> Result<()>;
    fn write_add(&mut self) -> Result<()>;
    fn new_prod(&mut self) -> Result<()>;
    fn write_fst(&mut self) -> Result<()>;
    fn write_snd(&mut self) -> Result<()>;
    fn print(&mut self) -> Result<()>;
}

pub fn run(runner: &mut impl Runner, prog: &inst::Program) -> Result<()> {
    for inst in &prog.insts {
        run_inst(runner, &inst)?;
    };
    Result::Ok(())
}
pub fn run_inst(runner: &mut impl Runner, inst: &inst::Inst) -> Result<()> {
    match &inst {
        inst::Inst::Push => {
            runner.push()?;
        }
        inst::Inst::Pop => {
            runner.pop()?;
        }
        inst::Inst::NewInt { value } => {
            runner.new_int(*value)?;
        }
        inst::Inst::WriteAdd => {
            runner.write_add()?;
        }
        inst::Inst::NewProd => {
            runner.new_prod()?;
        }
        inst::Inst::WriteFst => {
            runner.write_fst()?;
        }
        inst::Inst::WriteSnd => {
            runner.write_snd()?;
        }
        inst::Inst::Print => {
            runner.print()?;
        }
    };
    Result::Ok(())
}
