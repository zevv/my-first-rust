
pub struct Driver
{
    id: i8
}
    
pub fn new(id: i8, dir: super::Direction) -> Box<dyn super::Driver> {
    Box::new(Driver {id: id})
}


impl super::super::Driver for Driver {
    fn name(&self) -> &'static str {
        "gpio_linux"
    }
    fn dump(&self) {
        println!("dump {}", self.name());
    }
}

impl super::Driver for Driver {

    fn set(&self, _val: bool) {
        println!("set linux");
    }
    
    fn get(&self) -> bool {
        println!("get linux");
        true
    }

}

