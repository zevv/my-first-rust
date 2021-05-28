
mod cell;
mod driver;


// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

struct Bms {
    gpio1: Box<dyn driver::gpio::Driver>,
    gpio2: Box<dyn driver::gpio::Driver>,
}

fn main() {

    println!("Hello");

    let bms = Bms {
        gpio1: driver::gpio::linux::Linux::new(),
        gpio2: driver::gpio::arm::Arm::new(),
    };

    bms.gpio.set(true);
   
}

