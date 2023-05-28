use crate::runtime;

pub struct Stack {
    internal: Vec<runtime::heap::Ptr>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            internal: Vec::with_capacity(256),
        }
    }

    pub fn push(&mut self, item: runtime::heap::Ptr) {
        self.internal.push(item)
    }

    pub fn pop(&mut self) -> Option<runtime::heap::Ptr> {
        self.internal.pop()
    }
}
