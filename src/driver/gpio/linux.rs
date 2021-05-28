
pub struct Linux
{
    id: i8
}

impl Linux {
    
    pub fn new(id: i8, dir: super::Direction) -> Box<dyn super::Driver> {
        Box::new(Linux {id: id})
    }

}

impl super::super::Driver for Linux {
    fn name(&self) -> &'static str {
        "gpio_linux"
    }
    fn dump(&self) {
        println!("dump");
    }
}

impl super::Driver for Linux {

    fn set(&self, _val: bool) {
        println!("set linux");
    }
    
    fn get(&self) -> bool {
        println!("get linux");
        true
    }

}

