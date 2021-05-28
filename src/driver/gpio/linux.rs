
pub struct Driver
{
    id: i8
}
    
pub fn new(id: i8, _dir: super::Direction) -> Box<dyn super::Driver> {
    Box::new(Driver {id: id})
}


impl super::super::Driver for Driver {
    fn name(&self) -> String {
        format!("gpio_linux {}", self.id)
    }
    fn dump(&self) {
        println!("dump {} {}", self.name(), self.id.to_string());
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

