use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;

/// Simple struct that contains an integer.
#[derive(Debug, Clone)]
struct Thing(i32);

/// Complex datatype that contains the kitchen sink.
/// Note the 'static lifetime parameter. This assumes that the data may live forever.
#[derive(Debug)]
struct Everything {
    /// Value that gets stored in the struct
    value: Thing,

    /// Value that is stored as a pointer.
    simple_ref: &'static Thing,

    /// Value that is stored as a pointer and length.
    slice: &'static [Thing],

    /// Value that is allocated on the heap.
    unqiue_ptr: Box<Thing>,

    /// List of values that are allocated on the heap.
    vector: Vec<Thing>,

    /// Keys and values allocated on the heap.
    hash_map: HashMap<i32, Thing>,

    /// Value that may be shared in a thread.
    shared_ptr: Rc<Thing>,

    /// Value that may be shared between threads.
    global_shared_ptr: Arc<Thing>,
}

impl Everything {
    /// Make my complex datatype using a mixture of methods.
    fn new() -> Self {
        Self {
            value: Thing(1),
            simple_ref: &Thing(2),
            slice: &[Thing(3), Thing(4)],
            unqiue_ptr: Box::new(Thing(5)),
            vector: vec![Thing(6)],
            hash_map: [(7, Thing(7)), (8, Thing(8))].iter().cloned().collect(),
            shared_ptr: Rc::new(Thing(9)),
            global_shared_ptr: Arc::new(Thing(10)),
        }
    }
}

fn main() {
    let e = Everything::new();
    println!("e={:?}", e);
    println!("e.hash_map[&7]={:?}", e.hash_map[&7]);
}
