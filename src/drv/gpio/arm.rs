
use crate::drv;
use std::cell::RefCell;

// STM32 GPIO driver

pub struct DriverData {
}

pub struct Data {
    pub data: RefCell<DriverData>,
}

unsafe impl Sync for Data {}

impl drv::gpio::DrvGpio for Data {

    fn init(&self) {
        println!("init stm32");
    }

    fn get(&self) -> bool {
        false
    }
}


