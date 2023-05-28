use std::ptr::NonNull;
use std::io::{Result, Error, ErrorKind};
use std::sync::Mutex;

// `()` is the universal dummy type. Actual types are either ItemBody*.
#[derive(Debug, Copy, Clone)]
pub struct Ptr(*mut Item<()>);

impl Ptr {
    fn as_nonnull(self) -> Result<NonNull<Item<()>>> {
        match NonNull::new(self.0) {
            None => {
                Err(Error::new(ErrorKind::AddrNotAvailable, "Null pointer cannot free."))?
            }
            Some(x) => {
                Ok(x)
            }
        }
    }

    pub unsafe fn as_prod_mut<'a>(self) -> Result<Option<&'a mut ItemBodyProd>> {
        let item_ptr = self.as_nonnull()?;
        let item = item_ptr.as_ref();

        let item_body_ptr = match item.typ {
            ItemType::Int => {
                return Ok(None)
            }
            ItemType::Prod => {
                item.body.as_ptr() as *mut ItemBodyProd
            }
        };

        Ok(Some(&mut *item_body_ptr))
    }
}

pub struct Heap {
    free_subheap_int: Mutex<*mut Item<ItemBodyInt>>,
    free_subheap_prod: Mutex<*mut Item<ItemBodyProd>>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            free_subheap_int: Mutex::new(std::ptr::null_mut()),
            free_subheap_prod: Mutex::new(std::ptr::null_mut()),
        }
    }

    pub fn alloc_int(&mut self, value: i32) -> Result<Ptr> {
        let use_item_ptr_opt = {
            let mut free_subheap = self.free_subheap_int.lock().unwrap();
            match NonNull::new(*free_subheap) {
                Some(free_item_ptr) => {
                    *free_subheap = unsafe { free_item_ptr.as_ref().next };
                    Some(free_item_ptr)
                }
                None => {
                    None
                }
            }
        };

        let use_item_ptr = match use_item_ptr_opt {
            Some(mut use_item_ptr) => {
                unsafe {
                    let mut use_item_body = use_item_ptr.as_mut().body.as_mut();
                    use_item_body.value = value;
                };
                use_item_ptr
            }
            None => {
                let new_item_body = NonNull::from(Box::leak(Box::new(ItemBodyInt {
                    value: value,
                })));
                let new_item = NonNull::from(Box::leak(Box::new(Item {
                    typ: ItemType::Int,
                    next: std::ptr::null_mut(),
                    body: new_item_body,
                })));
                new_item
            }
        };

        Ok(Ptr(use_item_ptr.as_ptr() as *mut Item<()>))
    }

    pub fn alloc_prod(&mut self, first: Ptr, second: Ptr) -> Result<Ptr> {
        let use_item_ptr_opt = {
            let mut free_subheap = self.free_subheap_prod.lock().unwrap();
            match NonNull::new(*free_subheap) {
                Some(free_item_ptr) => {
                    *free_subheap = unsafe { free_item_ptr.as_ref().next };
                    Some(free_item_ptr)
                }
                None => {
                    None
                }
            }
        };

        let use_item_ptr = match use_item_ptr_opt {
            Some(mut use_item_ptr) => {
                unsafe {
                    let mut use_item_body = use_item_ptr.as_mut().body.as_mut();
                    use_item_body.first = first;
                    use_item_body.second = second;
                };
                use_item_ptr
            }
            None => {
                let new_item_body = NonNull::from(Box::leak(Box::new(ItemBodyProd {
                    first: first,
                    second: second,
                })));
                let new_item = NonNull::from(Box::leak(Box::new(Item {
                    typ: ItemType::Int,
                    next: std::ptr::null_mut(),
                    body: new_item_body,
                })));
                new_item
            }
        };

        Ok(Ptr(use_item_ptr.as_ptr() as *mut Item<()>))
    }

    pub fn free(&mut self, pointer: Ptr) -> Result<()> {
        let mut pointer_nonnull = pointer.as_nonnull()?;
        unsafe {
            let mut item = pointer_nonnull.as_mut();
            match item.typ {
                ItemType::Int => {
                    let mut free_subheap = self.free_subheap_int.lock().unwrap();
                    item.next = *free_subheap as *mut Item<()>;
                    *free_subheap = pointer_nonnull.as_ptr() as *mut Item<ItemBodyInt>;
                }
                ItemType::Prod => {
                    let mut free_subheap = self.free_subheap_prod.lock().unwrap();
                    item.next = *free_subheap as *mut Item<()>;
                    *free_subheap = pointer_nonnull.as_ptr() as *mut Item<ItemBodyProd>;
                }
            }
        };
        Ok(())
    }
}

pub struct Item<T> {
    typ: ItemType,
    next: *mut Item<T>,
    body: NonNull<T>,
}

pub enum ItemType {
    Int,
    Prod,
}

pub struct ItemBodyInt {
    pub value: i32,
}

pub struct ItemBodyProd {
    pub first: Ptr,
    pub second: Ptr,
}
