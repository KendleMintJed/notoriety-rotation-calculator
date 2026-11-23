pub use self::{
    crew::Crew, general::GeneralMultipliers, repeat::RepeatBonus, spesific::SpesificMultipliers,
};

pub mod perks;

mod crew;
mod general;
mod repeat;
mod spesific;

fn mul(lhs: bool, rhs: f32) -> f32 {
    if lhs { rhs } else { 0.0 }
}
