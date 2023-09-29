use serde::{Serialize, Deserialize};

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
    pub name: String,
    pub item_type: ItemType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemInstance {
    pub id: String,
    pub item: Item
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Entity {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Npc {
    pub entity: Entity
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NpcSpawn {
    pub npc: Npc,
    pub location: Vec3d
}