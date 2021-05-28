
pub mod gpio;

pub trait Driver {
    fn dump(&self);
    fn name(&self) -> &'static str;
}

