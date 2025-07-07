use std::{collections::HashMap, net::SocketAddr};

use uuid::Uuid;

pub trait AuthorizationHandlerTrait: Send + Sync {
    fn add_entity(&mut self, addr: SocketAddr, entity_id: Uuid);
    fn get_character_id(&self, addr: SocketAddr) -> Option<Uuid>;
    fn remove_entity(&mut self, addr: SocketAddr, entity_id: Uuid);
    fn is_authorized(&self, addr: SocketAddr, entity_id: Uuid) -> bool;
}

pub struct SocketOwned {
    pub player_character: Uuid,
    // first one is always the player character
    pub entities: Vec<Uuid>,
}

pub struct AuthorizationHandler {
    owned: HashMap<SocketAddr, SocketOwned>,
}

impl AuthorizationHandler {
    pub fn new() -> Self {
        AuthorizationHandler {
            owned: HashMap::new(),
        }
    }
}

impl AuthorizationHandlerTrait for AuthorizationHandler {
    fn add_entity(&mut self, addr: SocketAddr, entity_id: Uuid) {
        let entry = self.owned.entry(addr).or_insert_with(|| SocketOwned {
            player_character: entity_id,
            entities: vec![],
        });

        if !entry.entities.contains(&entity_id) {
            entry.entities.push(entity_id);
        }
    }

    fn get_character_id(&self, addr: SocketAddr) -> Option<Uuid> {
        self.owned.get(&addr).map(|entry| entry.player_character)
    }

    fn remove_entity(&mut self, addr: SocketAddr, entity_id: Uuid) {
        if let Some(entry) = self.owned.get_mut(&addr) {
            entry.entities.retain(|&id| id != entity_id);
        }
    }

    fn is_authorized(&self, addr: SocketAddr, entity_id: Uuid) -> bool {
        if let Some(entry) = self.owned.get(&addr) {
            entry.entities.contains(&entity_id)
        } else {
            false
        }
    }
}
