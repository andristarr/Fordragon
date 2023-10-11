use serde::{Serialize, Deserialize};

// these are used as dbos for now

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemType {
    OneHandedSword(EquipableItem), TwoHandedSword(EquipableItem)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EquipableItem {
    pub stats: Stats
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub stamina: isize,
    pub intellect: isize,
    pub agility: isize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub uuid: String,
    pub name: String,
    pub item_type: ItemType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemInstance {
    pub uuid: String,
    pub item: Item
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Entity {
    pub uuid: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Npc {
    pub uuid: String,
    pub entity: Entity
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NpcSpawn {
    pub uuid: String,
    pub npc: Npc,
    pub location: Vec3d
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LootTable {
    pub npc_uuid: String,
    pub entries: Vec<LootEntry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LootEntry {
    pub chance: f64,
    pub item: Item
}