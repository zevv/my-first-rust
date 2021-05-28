
pub struct Linux
{
}

impl Linux {
    
    pub fn new() -> Box<dyn super::Driver> {
        Box::new(Linux {})
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

