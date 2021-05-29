
//mod cell;
//mod driver;
mod units;
mod drv;

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




// GPIO device instances


macro_rules! def_gpio {
    ($name:ident, $dd:ident, $type:ident) => {

        static $name: drv::gpio::DevGpio = drv::gpio::DevGpio {
            drv: & $dd,
            nr: 32,
        };

        static $dd: drv::gpio::$type::Data = drv::gpio::$type::Data {
            value: Cell::new(1),
        };
    }
}


def_gpio!(gpio1, gpio1_data, linux);


static gpio2: drv::gpio::DevGpio = drv::gpio::DevGpio {
    drv: &gpio2_data,
    nr: 33,
};

static gpio2_data: drv::gpio::linux::Data = drv::gpio::linux::Data {
    value: Cell::new(15),
};


static gpio3: drv::gpio::DevGpio = drv::gpio::DevGpio {
    drv: &gpio3_data,
    nr: 14,
};

static gpio3_data: drv::gpio::arm::Data = drv::gpio::arm::Data {
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

