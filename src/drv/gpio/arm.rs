
use crate::drv;
use std::cell::Cell;

// STM32 GPIO driver

pub struct Data {
}

impl drv::gpio::DrvGpio for Data {

    fn init(&self) {
        println!("init stm32");
    }

    fn get(&self) -> bool {
        true
    }
}


