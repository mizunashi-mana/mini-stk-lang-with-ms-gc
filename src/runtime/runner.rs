use crate::stklang;
use crate::runtime;
use std::io::Result;

pub fn init() -> Result<RustAllocRunner> {
    let mut heap = runtime::heap::Heap::new();
    let stack = runtime::stack::Stack::new();
    let initial_register = heap.alloc_int(0)?;

    Ok(RustAllocRunner {
        register: initial_register,
        stack: stack,
        heap: heap,
    })
}

pub struct RustAllocRunner {
    register: runtime::heap::Ptr,
    stack: runtime::stack::Stack,
    heap: runtime::heap::Heap,
}

impl stklang::runner::Runner for RustAllocRunner {
    fn push(&mut self) -> Result<()> {
        self.stack.push(self.register);
        Ok(())
    }

    fn pop(&mut self) -> Result<()> {
        panic!("Not implemented")
    }

    fn new_int(&mut self, value: i32) -> Result<()> {
        let new_value = self.heap.alloc_int(value)?;
        self.register = new_value;
        Ok(())
    }

    fn write_add(&mut self) -> Result<()> {
        panic!("Not implemented")
    }

    fn new_prod(&mut self) -> Result<()> {
        panic!("Not implemented")
    }

    fn write_fst(&mut self) -> Result<()> {
        let prod_item_opt = unsafe { self.register.as_prod_mut() }?;
        let mut prod_item = match prod_item_opt {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };

        let pop_value = match self.stack.pop() {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };

        prod_item.first = pop_value;
        Ok(())
    }

    fn write_snd(&mut self) -> Result<()> {
        let prod_item_opt = unsafe { self.register.as_prod_mut() }?;
        let mut prod_item = match prod_item_opt {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };

        let pop_value = match self.stack.pop() {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };

        prod_item.second = pop_value;
        Ok(())
    }

    fn print(&mut self) -> Result<()> {
        panic!("Not implemented")
    }
}
