
pub mod gpio;
pub mod adc;

pub trait Driver {
    fn name(&self) -> String;

    fn dump(&self) {
        println!("dump generic");
    }
}
