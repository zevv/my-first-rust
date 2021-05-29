
pub mod linux;
pub mod arm;

pub trait DrvGpio: Sync {
    fn init(&self);
    fn get(&self) -> bool;
}

pub struct DevGpio {
    pub drv: &'static DrvGpio,
    pub nr: i8,
}

impl DevGpio {
    pub fn init(&self) {
        self.drv.init();
    }
    
    pub fn get(&self) -> bool {
        self.drv.get()
    }
}


