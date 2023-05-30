use std::ptr::NonNull;
use std::io::{Result, Error, ErrorKind};
use std::sync::Mutex;

// `()` is the universal dummy type. Actual types are `Item<_>`.
#[derive(Debug, Copy, Clone)]
pub struct Ptr(*mut ());

const TAGBIT_INT: usize = 0b01;
const TAGBIT_PROD: usize = 0b10;

impl Ptr {
    fn get_tagbit(self) -> usize {
        (self.0 as usize) & 0b11usize
    }

    fn new_int(raw: NonNull<Item<ItemBodyInt>>) -> Ptr {
        Ptr(unsafe { raw.as_ptr().add(TAGBIT_INT) } as *mut ())
    }

    fn new_prod(raw: NonNull<Item<ItemBodyProd>>) -> Ptr {
        Ptr(unsafe { raw.as_ptr().add(TAGBIT_PROD) } as *mut ())
    }

    pub unsafe fn as_prod_mut<'a>(self) -> Result<Option<&'a mut ItemBodyProd>> {
        let tagbit = self.get_tagbit();

        if tagbit != TAGBIT_PROD {
            return Ok(None)
        };

        let item = match NonNull::new(self.0.sub(tagbit) as *mut Item<ItemBodyProd>) {
            None => {
                Err(Error::new(ErrorKind::AddrNotAvailable, "Null pointer cannot free."))?
            }
            Some(mut x) => {
                x.as_mut()
            }
        };

        Ok(Some(&mut item.body))
    }

    pub unsafe fn as_int_mut<'a>(self) -> Result<Option<&'a mut ItemBodyInt>> {
        let tagbit = self.get_tagbit();

        if tagbit != TAGBIT_INT {
            return Ok(None)
        };

        let item = match NonNull::new(self.0.sub(tagbit) as *mut Item<ItemBodyInt>) {
            None => {
                Err(Error::new(ErrorKind::AddrNotAvailable, "Null pointer cannot free."))?
            }
            Some(mut x) => {
                x.as_mut()
            }
        };

        Ok(Some(&mut item.body))
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
                    let mut use_item = use_item_ptr.as_mut();
                    use_item.body.value = value;
                };
                use_item_ptr
            }
            None => {
                let new_item = NonNull::from(Box::leak(Box::new(Item {
                    next: std::ptr::null_mut(),
                    body: ItemBodyInt {
                        value: value,
                    },
                })));
                new_item
            }
        };

        Ok(Ptr::new_int(use_item_ptr))
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
                    let mut use_item = use_item_ptr.as_mut();
                    use_item.body.first = first;
                    use_item.body.second = second;
                };
                use_item_ptr
            }
            None => {
                let new_item = NonNull::from(Box::leak(Box::new(Item {
                    next: std::ptr::null_mut(),
                    body: ItemBodyProd {
                        first: first,
                        second: second,
                    },
                })));
                new_item
            }
        };

        Ok(Ptr::new_prod(use_item_ptr))
    }

    pub unsafe fn free(&mut self, pointer: Ptr) -> Result<()> {
        let tagbit = pointer.get_tagbit();

        if tagbit == TAGBIT_INT {
            let mut item_ptr = match NonNull::new(pointer.0.sub(tagbit) as *mut Item<ItemBodyInt>) {
                None => {
                    Err(Error::new(ErrorKind::AddrNotAvailable, "Null pointer cannot free."))?
                }
                Some(x) => {
                    x
                }
            };
            let mut free_subheap = self.free_subheap_int.lock().unwrap();
            item_ptr.as_mut().next = *free_subheap;
            *free_subheap = item_ptr.as_ptr();
        } else if tagbit == TAGBIT_PROD {
            let mut item_ptr = match NonNull::new(pointer.0.sub(tagbit) as *mut Item<ItemBodyProd>) {
                None => {
                    Err(Error::new(ErrorKind::AddrNotAvailable, "Null pointer cannot free."))?
                }
                Some(x) => {
                    x
                }
            };
            let mut free_subheap = self.free_subheap_prod.lock().unwrap();
            item_ptr.as_mut().next = *free_subheap;
            *free_subheap = item_ptr.as_ptr();
        } else {
            Err(Error::new(ErrorKind::AddrNotAvailable, "The tag bit is illegal."))?
        };
        Ok(())
    }
}

pub struct Item<Body> {
    next: *mut Item<Body>,
    body: Body,
}

pub struct ItemBodyInt {
    pub value: i32,
}

pub struct ItemBodyProd {
    pub first: Ptr,
    pub second: Ptr,
}
