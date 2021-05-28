
pub mod gpio;

pub trait Driver {
    fn name(&self) -> String;

    fn dump(&self) {
        println!("dump generic");
    }
}
