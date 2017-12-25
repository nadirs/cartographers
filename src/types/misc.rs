pub trait HasName {
    fn get_name(&self) -> &str;
}

pub trait HasEffects {
    fn get_effects(&self) -> &str;
}