
pub mod linux;
pub mod arm;

pub trait Driver: super::Driver {
    fn set(&self, _val: bool);
    fn get(&self) -> bool;
}

