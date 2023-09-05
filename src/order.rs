use std::{
    ptr::{self, NonNull},
    alloc::{ self, Layout },
    collections::HashMap,
};

static mut NEXT_AVAILABLE_ID: usize = 1;

pub struct Order {
    id: usize,
    pub ready: bool,

    pub items: NonNull<String>,
    capacity: usize,
    length: usize,
}

unsafe impl Send for Order {}
unsafe impl Sync for Order {}

impl Default for Order {
    fn default() -> Self {
            let id: usize;
            unsafe { // this program is not parallel or concurent so this is OK
                id = NEXT_AVAILABLE_ID;
                NEXT_AVAILABLE_ID += 1;
            }
            let ready: bool = false;
            let capacity: usize = 3; // as per directions
            let length: usize = 0;

            // prepare to allocate, by defining the memorry layout we will need to allocate
            let items_memory_layout: Layout = Layout::array::<String>(capacity).unwrap();
            assert!(items_memory_layout.size() <= isize::MAX as usize, "Allocation to large in Order::default()");
            
            // actually allocate memory for the String  array
            let items_pointer: *mut String;
            unsafe { items_pointer = alloc::alloc(items_memory_layout) as *mut String }
            
            // wrap the newly allocated pointer in a NonNull
            let items: NonNull<String>  = match NonNull::new(items_pointer) {
                Some(new_items) => new_items,
                None => alloc::handle_alloc_error(items_memory_layout),
            };

            return Order { id, ready, items, capacity, length };
    }
}

impl Order {
    pub fn new(items: &[String]) -> Self {
        let id: usize;
        unsafe { // this program is not parallel or concurent so this is OK
            id = NEXT_AVAILABLE_ID;
            NEXT_AVAILABLE_ID += 1;
        }
        let ready: bool = false;
        let capacity: usize = items.len(); // make room for any other items
        let length: usize = items.len();

        // prepare to allocate, by defining the memorry layout we will need to allocate
        let items_memory_layout: Layout = Layout::array::<String>(capacity).unwrap();
        assert!(items_memory_layout.size() <= isize::MAX as usize, "Allocation to large in Order::new()"); // i used isize becasue of llvm
        
        // actually allocate memory for the String  array
        let items_pointer: *mut String;
        unsafe { items_pointer = alloc::alloc(items_memory_layout) as *mut String }

        // wrap the newly allocated pointer in a NonNull
        let items_nonnull: NonNull<String> = match NonNull::new(items_pointer) {
            Some(new_items) => new_items,
            None => alloc::handle_alloc_error(items_memory_layout),
        };

        // copy from the slice to the new pointer
        for i in 0..items.len() {
            unsafe {
                let first_empty_slot: *mut String = items_nonnull.as_ptr().add(i);
                ptr::write(first_empty_slot, items[i].clone());
            }
        }

        return Order { id, ready, items: items_nonnull, capacity, length };
    }

    pub fn get_id(&self) -> usize {
        return self.id;
    }

    fn count_items(&self) -> HashMap<String, usize> {
        let mut item_counts: HashMap<String, usize> = HashMap::new();

        for i in 0..self.length {
            let item: String;
            unsafe { item = ptr::read(self.items.as_ptr().add(i)) }
            let count: &mut usize = item_counts.entry(item.clone()).or_insert(0);
            *count += 1;
        }
        
        return item_counts;
    }
}

impl ToString for Order {
    fn to_string(&self) -> String {
        let ready_str: &str = if self.ready {"Ready"} else {"Not Ready"};
        let mut item_string: String = String::new();
        for (item, count) in self.count_items().into_iter() {
            item_string.push_str(format!("{count} {item}\n").as_str());
        }
        return format!("Order number: {}\n{}\n{}", self.id, ready_str, item_string);
    }
}

impl std::ops::Deref for Order {
    type Target = [String];
    fn deref(&self) -> &[String] {
        unsafe {
            return std::slice::from_raw_parts(self.items.as_ptr(), self.length);
        }
    }
}

impl std::ops::DerefMut for Order {
    fn deref_mut(&mut self) -> &mut [String] {
        unsafe {
            return std::slice::from_raw_parts_mut(self.items.as_ptr(), self.length);
        }
    }
}

impl Drop for Order {
    fn drop(&mut self) {
        let self_layout: Layout = Layout::array::<String>(self.capacity).unwrap();
        unsafe { alloc::dealloc(self.items.as_ptr() as *mut u8, self_layout); }
    }
}