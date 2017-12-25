type StatValue = usize;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum StatType {
    Strength,
    Vitality,
    Technique,
    Intelligence,
    Luck,
    Agility,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Stat(StatType, StatValue);