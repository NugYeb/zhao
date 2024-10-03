use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tonic::Status;

pub static EXTEND: Lazy<Arc<RwLock<Extend>>> = Lazy::new(|| Arc::new(RwLock::new(Extend::new())));

pub struct Extend {
    pub connection: HashMap<String, String>,
}

impl Extend {
    pub fn new() -> Extend {
        Extend {
            connection: HashMap::new(),
        }
    }

    pub fn get_addr(&self, name: String) -> Option<&String> {
        self.connection.get(&name)
    }

    pub fn add_extend(&mut self, name: String, addr: String) -> Option<String> {
        self.connection.insert(name, addr)
    }

    pub fn remove_extend(&mut self, name: String) -> Option<String> {
        self.connection.remove(&name)
    }
}

pub fn add_write_extend(name: String, addr: String) -> Result<Option<String>, Status> {
    let mut extend = EXTEND.write();
    Ok(extend.add_extend(name, addr))
}

pub fn get_read_extend(name: String) -> Result<Option<String>, Status> {
    let extend = EXTEND.read();
    Ok(match extend.get_addr(name) {
        Some(addr) => Some(addr.clone()),
        None => None,
    })
}

pub fn remove_write_extend(name: String) -> Result<Option<String>, Status> {
    let mut extend = EXTEND.write();
    Ok(extend.remove_extend(name))
}

pub fn get_read_extend_list() -> Result<HashMap<String, String>, Status> {
    let extend = EXTEND.read();
    Ok(extend.connection.clone())
}
