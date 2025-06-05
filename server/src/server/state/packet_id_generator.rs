use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub struct PacketIdGenerator {
    id_containers: Arc<Mutex<HashMap<SocketAddr, u128>>>,
}

impl Default for PacketIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketIdGenerator {
    pub fn new() -> Self {
        PacketIdGenerator {
            id_containers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn generate_id(&self, conn: SocketAddr) -> u128 {
        let mut id_containers = self
            .id_containers
            .lock()
            .expect("Failed to lock id_containers");

        let id = id_containers.get(&conn);

        if let Some(id) = id {
            let new_id = *id + 1;
            id_containers.insert(conn, new_id);
            new_id
        } else {
            let new_id = 0;
            id_containers.insert(conn, new_id);
            new_id
        }
    }
}
