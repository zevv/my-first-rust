
pub mod gpio;

pub trait Driver {
    fn dump(&self);
    fn name(&self) -> &'static str;
}

impl Driver {
    fn dump(&self) {
        println!("dump generic");
    }
}
