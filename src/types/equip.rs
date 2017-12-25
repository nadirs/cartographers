use std::collections::HashMap;
use super::misc::HasName;
use super::weapon::Weapon;
use super::armor::{ArmorType, ArmorItem};
use super::class::Class;


pub trait EquipItem: HasName {
    fn can_equip(&self, class: &Class) -> bool;
    fn equip_on(&self, equip: &Equip) -> Equip;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Equip {
    weapon: Option<Weapon>,
    armors: HashMap<ArmorType, ArmorItem>,
}

pub enum EquipError {
    ClassUnableToEquipItem,
}