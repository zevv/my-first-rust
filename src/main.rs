
//mod cell;
mod driver;
mod units;

use units::*;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

struct Bms {
    gpio: [Box<dyn driver::gpio::Driver>; 2],
}

fn main() {

    println!("Hello");

    use driver::gpio::Direction;

    let bms = Bms {
        gpio: [
            driver::gpio::linux::new(23, Direction::Input),
            driver::gpio::arm::new(),
        ]
    };

    for g in bms.gpio.iter() {
        g.set(true);
        g.dump();
    }

    let u = Voltage::from(8.0);
    let i = Current::from(4.0);
    let t = Duration::from(3.0);

    println!("{}", u * i * t);
}

