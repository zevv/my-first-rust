
use crate::driver;

pub struct Driver
{
}

    
pub fn new() -> Box<dyn super::Driver> {
    Box::new(Driver {})
}


impl super::super::Driver for Driver {
    fn name(&self) -> String {
        "gpio_arm".to_string()
    }
}

impl super::Driver for Driver {

    fn set(&self, _val: bool) {
        println!("set arm");
    }
    
    fn get(&self) -> bool {
        println!("get arm");
        true
    }

}
