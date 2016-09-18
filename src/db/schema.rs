use std::collections::HashMap;

// owned by a Stream
#[derive(Debug)]
pub struct Schema {
    num_fields: u8,
    // maps a name to an id
    fields: HashMap<String, u8>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema{num_fields: 0, fields: HashMap::new()}
    }
}