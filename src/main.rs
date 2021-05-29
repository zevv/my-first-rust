
//mod cell;
//mod driver;
mod units;

use units::*;

const MAX_CELL_COUNT: usize = 32;

//static gpio1: driver::gpio::Driver = driver::gpio::linux::new();

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

#[allow(non_snake_case)]
#[derive(Clone,Copy,Debug)]
struct Cell {
    U: Voltage,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            U: Voltage::from(3.3300)
        }
    }
}

struct Bms {
    //gpio: [Box<dyn driver::gpio::Driver>; 2],
    cell_count: usize,
    cells: [Cell; MAX_CELL_COUNT],
}



struct DevGpio {
    drv: &DrvGpio,
    nr: i8,
}

struct DrvGpio {
    init: fn(&DevGpio),
}


fn main() {

    println!("Hello");

    //use driver::gpio::Direction;

    let mut bms = Bms {
        //gpio: [
        //    driver::gpio::linux::new(23, Direction::Input),
        //    driver::gpio::arm::new(),
        //],
        cell_count: 8,
        cells: [Cell::default(); MAX_CELL_COUNT],
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

    for i in 0..bms.cell_count {
        println!("{:?}", bms.cells[i])
    }

}

