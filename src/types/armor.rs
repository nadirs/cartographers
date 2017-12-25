use std::collections::HashSet;
use super::stat::Stat;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmorItem {
    name: String,
    effects: HashSet<Stat>,
    armor_type: ArmorType,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ArmorType {
    Body,
    Head,
    Hands,
    Feet,
}