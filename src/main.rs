
//mod cell;
//mod driver;
mod units;

use units::*;
use std::cell::Cell;

const MAX_CELL_COUNT: usize = 32;

//static gpio1: driver::gpio::Driver = driver::gpio::linux::new();

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

//#[allow(non_snake_case)]
//#[derive(Clone,Copy,Debug)]
//struct Cell {
//    U: Voltage,
//}

//impl Default for Cell {
//    fn default() -> Cell {
//        Cell {
//            U: Voltage::from(3.3300)
//        }
//    }
//}

struct Bms {
    //gpio: [Box<dyn driver::gpio::Driver>; 2],
    //cell_count: usize,
    //cells: [Cell; MAX_CELL_COUNT],
}




// GPIO Driver interface

trait DrvGpio: Sync {
    fn init(&self);
    fn get(&self) -> bool;
}

// GPIO device instance

struct DevGpio {
    drv: &'static DrvGpio,
    nr: i8,
}

// Generic gpio functions

impl DevGpio {
    fn init(&self) {
        self.drv.init();
    }
    
    fn get(&self) -> bool {
        self.drv.get()
    }
}


// Linux GPIO driver

struct GpioLinux {
    value: Cell<i8>,
}
unsafe impl Sync for GpioLinux {}

impl DrvGpio for GpioLinux {
    fn init(&self) {
        println!("init linux, value = {}", self.value.get());
    }
    fn get(&self) -> bool {
        let prev = self.value.get();
        self.value.set(prev + 1);
        prev % 2 == 0
    }
}


// STM32 GPIO driver

struct GpioArm {
}

impl DrvGpio for GpioArm {
    fn init(&self) {
        println!("init stm32");
    }
    fn get(&self) -> bool {
        true
    }
}


// GPIO device instances

static gpio1_data: GpioLinux = GpioLinux {
    value: Cell::new(1),
};

static gpio2_data: GpioLinux = GpioLinux {
    value: Cell::new(1),
};

static gpio3_data: GpioArm = GpioArm {
};


static gpio1: DevGpio = DevGpio {
    drv: &gpio1_data,
    nr: 32,
};
static gpio2: DevGpio = DevGpio {
    drv: &gpio2_data,
    nr: 33,
};
static gpio3: DevGpio = DevGpio {
    drv: &gpio3_data,
    nr: 14,
};



fn main() {

    println!("Hello");

    gpio1.init();
    gpio2.init();
    gpio3.init();
    
    println!("get 1 {}", gpio1.get());
    println!("get 2 {}", gpio2.get());
    println!("get 3 {}", gpio3.get());


    //use driver::gpio::Direction;

    let mut bms = Bms {
        //gpio: [
        //    driver::gpio::linux::new(23, Direction::Input),
        //    driver::gpio::arm::new(),
        //],
        //cell_count: 8,
        //cells: [Cell::default(); MAX_CELL_COUNT],
    };

    //for g in bms.gpio.iter() {
    //    g.set(true);
    //    g.dump();
    //}

    let u = Voltage::from(8.0);
    let i = Current::from(-50.0);
    let t = Duration::from(3.0);

    println!("{}", u * i * t);
    
    let a = Acceleration::from(9.81);
    let d = Duration::from(10.0);

    println!("{}", a * d);

    //for i in 0..bms.cell_count {
    //    println!("{:?}", bms.cells[i])
    //}

}

