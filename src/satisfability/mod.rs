mod general;
mod dynamic;

pub use general::GeneralSatisfability;
pub use dynamic::DynamicSatisfability;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Expectative {
    True,
    False,
    Any,
}
