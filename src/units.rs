
use std::fmt;

static SCALE_LIST: [(&str, f64); 8] = [
    ("G", 1e9),
    ("M", 1e6),
    ("k", 1e3),
    ("",  1e0),
    ("m", 1e-3),
    ("µ", 1e-6),
    ("n", 1e-9),
    ("p", 1e-12),
];

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Integrity {
    Low, Medium, High
}

pub trait Unit: {
    fn unit(&self) -> &str;
    fn symbol(&self) -> &str;
    fn get(&self) -> f64;
}


macro_rules! gen_unit {
    ($type: ident, $symbol: literal, $unit:literal) => {

        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        pub struct $type {
            val: f64,
            integrity: Integrity,
        }

        impl From<f64> for $type {
            fn from(val: f64) -> $type {
                $type {
                    val: val,
                    integrity: Integrity::High,
                }
            }
        }
        
        impl From<i32> for $type {
            fn from(val: i32) -> $type {
                $type {
                    val: val as f64,
                    integrity: Integrity::High,
                }
            }
        }

        impl Unit for $type {
            fn unit(&self) -> &str { $unit }
            fn symbol(&self) -> &str { $symbol }
            fn get(&self) -> f64 { self.val }
        }

        // Display formatter
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let val = self.get();
                let val_abs = val.abs();
                for &(prefix, scale) in SCALE_LIST.iter() {
                    if val_abs > scale {
                        return write!(f, "{}: {:.3} {}{}", 
                                      self.symbol(),
                                      val / scale,
                                      prefix, self.unit());
                    }
                }
                write!(f, "")
            }
        }

        // type + type = type
        impl std::ops::Add<$type> for $type {
            type Output = $type;
            fn add(self, v: $type) -> $type {
                Self::Output::from(self.get() + v.get())
            }
        }
       
        // type * const = type
        impl std::ops::Mul<f64> for $type {
            type Output = $type;
            fn mul(self, v: f64) -> $type {
                Self::Output::from(self.get() + v)
            }
        }
       
        // const * type = type
        impl std::ops::Mul<$type> for f64 {
            type Output = $type;
            fn mul(self, v: $type) -> $type {
                Self::Output::from(self + v.get())
            }
        }

    }
}


macro_rules! gen_operators {
    ($a:ty, $b:ty, $c:ty) => {

        // a / b = c
        impl std::ops::Div<$b> for $a {
            type Output = $c;
            fn div(self, v: $b) -> Self::Output {
                Self::Output::from(self.get() / v.get())
            }
        }
      
        // a / c = b
        impl std::ops::Div<$c> for $a {
            type Output = $b;
            fn div(self, v: $c) -> Self::Output {
                Self::Output::from(self.get() / v.get())
            }
        }

        // b * c = a
        impl std::ops::Mul<$c> for $b {
            type Output = $a;
            fn mul(self, v: $c) -> Self::Output {
                Self::Output::from(self.get() * v.get())
            }
        }
       
        // c * b = a
        impl std::ops::Mul<$b> for $c {
            type Output = $a;
            fn mul(self, v: $b) -> Self::Output {
                Self::Output::from(self.get() * v.get())
            }
        }
    }
}


gen_unit!(Current,        "I",  "A"     );
gen_unit!(Voltage,        "U",  "V"     );
gen_unit!(Power,          "P",  "W"     );
gen_unit!(Resistance,     "R",  "Ω"     );
gen_unit!(Duration,       "T",  "s"     );
gen_unit!(Charge,         "Q",  "C"     );
gen_unit!(Energy,         "E",  "J"     );
gen_unit!(Temperature,    "T",  "K"     );
gen_unit!(Distance,       "S",  "m"     );
gen_unit!(Speed,          "V",  "m/s "  );
gen_unit!(Acceleration,   "a",  "m/s²"  );
gen_unit!(Force,          "F",  "N"     );

gen_operators!(Power, Voltage, Current);
gen_operators!(Power, Force, Speed);
gen_operators!(Voltage, Current, Resistance);
gen_operators!(Charge, Current, Duration);
gen_operators!(Energy, Power, Duration);
gen_operators!(Distance, Duration, Speed);
gen_operators!(Speed, Duration, Acceleration);
