
pub mod linux;
pub mod arm;

pub enum Direction {
    Input,
    InputPullUp,
    InputPullDown,
    Output
}

pub trait Driver: super::Driver {
    fn set(&self, _val: bool);
    fn get(&self) -> bool;
}

