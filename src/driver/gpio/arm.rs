
pub struct Arm
{
}

impl Arm {
    
    pub fn new() -> Box<dyn super::Driver> {
        Box::new(Arm {})
    }

}

impl super::super::Driver for Arm {
    fn name(&self) -> &'static str {
        "gpio_linux"
    }
    fn dump(&self) {
        println!("dump");
    }
}

impl super::Driver for Arm {

    fn set(&self, _val: bool) {
        println!("set arm");
    }
    
    fn get(&self) -> bool {
        println!("get arm");
        true
    }

}
