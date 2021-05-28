
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
        gpio1: driver::gpio::linux::new(23, driver::gpio::Direction::Input),
        gpio2: driver::gpio::arm::new(),
    };

    bms.gpio1.set(true);
    bms.gpio2.set(true);
    
    bms.gpio1.dump();
    bms.gpio2.dump();
   
}

