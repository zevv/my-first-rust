
//mod cell;
//mod driver;
mod units;
mod drv;

use units::*;
use paste::paste;
use std::cell::RefCell;

// GPIO device instances

def_gpio!(gpio1, linux, drv::gpio::linux::DriverData { value: 5, });
def_gpio!(gpio2, linux, drv::gpio::linux::DriverData { value: 6, });
def_gpio!(gpio3, arm, drv::gpio::arm::DriverData {});



fn main() {

    println!("Hello");

    gpio1.init();
    gpio2.init();
    gpio3.init();
    
    println!("get 1 {}", gpio1.get());
    println!("get 2 {}", gpio2.get());
    println!("get 3 {}", gpio3.get());


    let u = Voltage::from(8.0);
    let i = Current::from(-50.0);
    let t = Duration::from(3.0);

    println!("{}", u * i * t);
    
    let a = Acceleration::from(9.81);
    let d = Duration::from(10.0);

    println!("{}", a * d);

}

