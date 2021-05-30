
use crate::drv;
use std::cell::RefCell;


pub struct DriverData {
    pub value: i8,
}

pub struct Data {
    pub data: RefCell<DriverData>,
}

unsafe impl Sync for Data {}


impl drv::gpio::DrvGpio for Data {

    fn init(&self) {
        let d = self.data.borrow_mut();
        println!("init linux, value = {}", d.value);
    }

    fn get(&self) -> bool {
        let mut d = self.data.borrow_mut();
        d.value = 5;
        true
    }
}


