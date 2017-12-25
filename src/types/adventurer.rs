use super::class::Class;
use super::equip::Equip;
use super::equip::EquipItem;
use super::equip::EquipError;

type Level = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Adventurer {
    class: Class,
    level: Level,
    equip: Equip,
}

impl Adventurer {
    pub fn equip<I: EquipItem>(&self, item: &I) -> Result<Equip, EquipError> {
        if item.can_equip(&self.class) {
            let equip = item.equip_on(&self.equip);
            Ok(equip)
        } else {
            Err(EquipError::ClassUnableToEquipItem)
        }
    }
}