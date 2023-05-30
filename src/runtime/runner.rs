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
        match self.stack.pop() {
            None => {
                return Ok(())
            }
            Some(pop_value) => {
                // TODO: free current register
                self.register = pop_value;
                Ok(())
            }
        }
    }

    fn new_int(&mut self, value: i32) -> Result<()> {
        let new_value = self.heap.alloc_int(value)?;
        // TODO: free current register
        self.register = new_value;
        Ok(())
    }

    fn write_int(&mut self, value: i32) -> Result<()> {
        let int_item_opt = unsafe { self.register.as_int_mut() }?;
        let mut int_item = match int_item_opt {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };

        int_item.value = value;

        Ok(())
    }

    fn write_add(&mut self) -> Result<()> {
        let int_item_opt = unsafe { self.register.as_int_mut() }?;
        let mut int_item = match int_item_opt {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };

        let pop_item_ptr = match self.stack.pop() {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };
        let pop_item_int_opt = unsafe { pop_item_ptr.as_int_mut() }?;
        match pop_item_int_opt {
            None => {
                // do nothing
            }
            Some(pop_item) => {
                int_item.value = int_item.value + pop_item.value;
            }
        };

        // TODO: free pop_item_ptr

        Ok(())
    }

    fn new_prod(&mut self) -> Result<()> {
        let pop_value1 = match self.stack.pop() {
            None => {
                return Ok(())
            }
            Some(x) => {
                x
            }
        };
        let pop_value2 = match self.stack.pop() {
            None => {
                pop_value1
            }
            Some(x) => {
                x
            }
        };

        let new_value = match self.heap.alloc_prod(pop_value1, pop_value2) {
            Err(err) => {
                // TODO: free pop_value1 and pop_value2
                Err(err)?
            }
            Ok(x) => {
                x
            }
        };

        // TODO: free current register
        self.register = new_value;

        Ok(())
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

        // TODO: free current first
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

        // TODO: free current first
        prod_item.second = pop_value;
        Ok(())
    }

    fn print(&mut self) -> Result<()> {
        panic!("Not implemented")
    }

    fn gc(&mut self) -> Result<()> {
        panic!("Not implemented")
    }
}
