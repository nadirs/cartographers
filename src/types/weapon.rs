use std::collections::HashSet;
use types::stat::Stat;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Weapon {
    name: String,
    effects: HashSet<Stat>,
}