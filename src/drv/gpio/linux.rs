
use crate::drv;
use std::cell::Cell;


pub struct Data {
    pub value: Cell<i8>,
}

unsafe impl Sync for Data {}


impl drv::gpio::DrvGpio for Data {

    fn init(&self) {
        println!("init linux, value = {}", self.value.get());
    }

    fn get(&self) -> bool {
        let prev = self.value.get();
        self.value.set(prev + 1);
        prev % 2 == 0
    }
}


