use crate::Strategy;

#[derive(Clone)]
pub struct StrategyPreset {
    pub id: u32,
    pub name: String,
    pub min_skills: u8,
    pub strategy: Strategy,
}
