use std::{
    ptr::{self, NonNull},
    alloc::{ self, Layout },
};

use crate::order::Order;

pub struct OrderList {
    pointer: NonNull<Order>,
    capacity: usize,
    length: usize,
}

impl OrderList {
    pub fn new() -> OrderList {
        let pointer: NonNull<Order> = NonNull::dangling();
        let capacity: usize = 0;
        let length: usize = 0;

        return OrderList { pointer, capacity, length };
    }

    fn grow(&mut self) {
        let zero_capacity: bool = self.capacity == 0;

        let new_capacity: usize = if zero_capacity {1} else {2 * self.capacity};
        
        let new_memory_layout: Layout = Layout::array::<Order>(new_capacity).unwrap();
        assert!(new_memory_layout.size() <= isize::MAX as usize, "Allocation to large in OrderList::grow");

        let new_pointer: *mut u8 = if zero_capacity {
            unsafe { alloc::alloc(new_memory_layout) }
        } else {
            let old_memory_layout: Layout = Layout::array::<Order>(self.capacity).unwrap();
            let old_pointer: *mut u8 = self.pointer.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_pointer, old_memory_layout, new_memory_layout.size()) }
        };

        self.pointer = match NonNull::new(new_pointer as *mut Order) {
            Some(new_nonnull) => new_nonnull,
            None => alloc::handle_alloc_error(new_memory_layout),
        };

        self.capacity = new_capacity;
    }

    fn push(&mut self, order: Order) {
        if self.capacity == self.length {
            self.grow();
        }

        unsafe {
            let first_empty_slot: *mut Order = self.pointer.as_ptr().add(self.length);
            ptr::write(first_empty_slot, order);
        }

        self.length += 1;
    }

    fn pop(&mut self) -> Option<Order> {
        if self.length == 0 {
            return None;
        } else {
            self.length -= 1;
            unsafe {
                let last_order: *mut Order = self.pointer.as_ptr().add(self.length);
                return Some(ptr::read(last_order));
            }
        }
    }

    fn get(&self, index: usize) -> Option<&Order> {
        if index <= self.length {
            return Some(&self[index]);
        } else {
            return None;
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Order> {
        if index <= self.length {
            return Some(&mut self[index]);
        } else {
            return None;
        }
    }

    fn remove(&mut self, index_to_remove: usize) -> Option<Order> {
        if index_to_remove < self.length || index_to_remove > self.length {
            return None;
        }

        unsafe {
            self.length -= 1;
            let removed_order: Order = ptr::read(self.pointer.as_ptr().add(index_to_remove));
            ptr::copy(
                self.pointer.as_ptr().add(index_to_remove + 1),
                self.pointer.as_ptr().add(index_to_remove),
                self.length - index_to_remove
            );
            return Some(removed_order);
        }
    }
}

unsafe impl Send for OrderList {}
unsafe impl Sync for OrderList {}

impl std::ops::Deref for OrderList {
    type Target = [Order];
    fn deref(&self) -> &[Order] {
        unsafe { 
            return std::slice::from_raw_parts(self.pointer.as_ptr(), self.length);
        }
    }
}

impl std::ops::DerefMut for OrderList {
    fn deref_mut(&mut self) -> &mut [Order] {
        unsafe { 
            return std::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length);
        }
    }
}

impl Drop for OrderList {
    fn drop(&mut self) {
        if self.capacity != 0 {
            while let Some(_order) = self.pop() {}

            let self_memory_layout: Layout = Layout::array::<Order>(self.capacity).unwrap();

            unsafe { alloc::dealloc(self.pointer.as_ptr() as *mut u8, self_memory_layout); }
        }
    }
}

impl OrderList {
    pub fn add_order(&mut self, order: Order) {
        self.push(order);
    }

    fn get_index_from_order_id(&self, id: usize) -> Option<usize> {
        for i in 0..self.length {
            if self[i].get_id() == id {
                return Some(i);
            }
        }
        return None;
    }

    pub fn remove_order(&mut self, order_id: usize) -> Option<Order> {
        let index_to_remove: usize =  self.get_index_from_order_id(order_id)?;
        return self.remove(index_to_remove);
    }

    /// false if there was no order for the given order_id
    pub fn ready_order(&mut self, order_id: usize) -> bool {
        let index_to_ready: usize = match self.get_index_from_order_id(order_id) {
            Some(index) => index,
            None => return false,
        };

        match self.get_mut(index_to_ready) {
            Some(order) => {
                order.ready = true;
                return true;
            },
            None => return false,
        };
    }

    pub fn sort_orders(&mut self) {
        let mut swapped: bool;
        if self.len() != 0{
            loop {
                swapped = false;
                for i in 0..(self.len() - 1) {
                    let order_id: usize =  self[i].get_id();
                    let adjacent_order_id: usize = self[i + 1].get_id();

                    if order_id > adjacent_order_id {
                        self.swap(i, i + 1);
                        swapped = true;
                    }
                }
                if !swapped { break; }
            }
        }
    }

    pub fn print_order(&self, order_id: usize) -> Option<String> {
        let index_to_print: usize = self.get_index_from_order_id(order_id)?;
        let order: &Order = self.get(index_to_print)?;
        return Some(order.to_string());
    }
    
    pub fn print_orders(&mut self) -> String {
        let mut ready: String = String::from("READY\n");
        let mut pending: String = String::from("PENDING\n");
        self.sort_orders();
        for order in self.iter() {
            let order_id: String = format!("{}\n", order.get_id());
            if order.ready {
                ready.push_str(order_id.as_str());
            } else {
                pending.push_str(order_id.as_str())
            }
        }
        return format!("{ready}{pending}");
    }
}
