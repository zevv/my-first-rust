
pub mod linux;
pub mod arm;

pub trait DrvGpio: Sync {
    fn init(&self);
    fn get(&self) -> bool;
}

pub struct DevGpio {
    pub drv: &'static dyn DrvGpio,
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


#[macro_export]
macro_rules! def_gpio {
    ($name:ident, $type:ident, $dd:expr) => {

        paste!{

            static $name: drv::gpio::DevGpio = drv::gpio::DevGpio {
                drv: & [<$name _dd>],
                nr: 32,
            };

            static [<$name _dd>]: drv::gpio::$type::Data = drv::gpio::$type::Data {
                data: RefCell::new($dd),
            };
        }
    }
}
