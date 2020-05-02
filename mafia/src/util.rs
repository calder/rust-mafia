pub type Map<K, V> = std::collections::BTreeMap<K, V>;
pub type Set<T> = std::collections::BTreeSet<T>;

pub type Faction = String;
pub type Player = String;
pub type PlayerOrFaction = String;

pub trait IsDefault {
    fn is_default(&self) -> bool;
}

impl<T> IsDefault for T
where
    T: Default + PartialEq,
{
    fn is_default(&self) -> bool {
        self == &Default::default()
    }
}

pub fn ron_pretty_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::default()
}
