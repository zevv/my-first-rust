
pub struct Driver
{
}

    
pub fn new() -> Box<dyn super::Driver> {
    Box::new(Driver {})
}


impl super::super::Driver for Driver {
    fn name(&self) -> &'static str {
        "gpio_arm"
    }
    fn dump(&self) {
        println!("dump {}", self.name());
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
