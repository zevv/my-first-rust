
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


pub trait Unit {
    fn unit(&self) -> &str;
    fn to_float(&self) -> f64;
}


macro_rules! mk_unit {
    ($type: ident, $unit:literal) => {

        #[derive(Clone, Copy, PartialEq, PartialOrd)]
        pub struct $type {
            val: f64,
        }

        impl $type {
            pub fn from(val: f64) -> $type {
                $type {val}
            }
        }

        impl Unit for $type {
            fn unit(&self) -> &str { $unit }
            fn to_float(&self) -> f64 { self.val }
        }

        // Display formatter
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let val = self.to_float();
                for &(prefix, scale) in SCALE_LIST.iter() {
                    if val > scale {
                        return write!(f, "{} {}{}", val / scale, prefix, self.unit());
                    }
                }
                write!(f, "")
            }
        }

        // Add unit type to itself
        impl std::ops::Add<$type> for $type {
            type Output = $type;
            fn add(self, v: $type) -> $type {
                Self::Output::from(self.to_float() + v.to_float())
            }
        }
       
        // Mul unit with constant
        impl std::ops::Mul<f64> for $type {
            type Output = $type;
            fn mul(self, v: f64) -> $type {
                Self::Output::from(self.to_float() + v)
            }
        }
       
        // Mul constant with unit
        impl std::ops::Mul<$type> for f64 {
            type Output = $type;
            fn mul(self, v: $type) -> $type {
                Self::Output::from(self + v.to_float())
            }
        }

    }
}


macro_rules! gen_operators {
    ($a:ty, $b:ty, $c:ty) => {

        //    a
        //  -----
        //  b * c
        
        impl std::ops::Div<$c> for $a {
            type Output = $b;
            fn div(self, v: $c) -> Self::Output {
                Self::Output::from(self.to_float() / v.to_float())
            }
        }

        impl std::ops::Div<$b> for $a {
            type Output = $c;
            fn div(self, v: $b) -> Self::Output {
                Self::Output::from(self.to_float() / v.to_float())
            }
        }

        impl std::ops::Mul<$c> for $b {
            type Output = $a;
            fn mul(self, v: $c) -> Self::Output {
                Self::Output::from(self.to_float() * v.to_float())
            }
        }
        
        impl std::ops::Mul<$b> for $c {
            type Output = $a;
            fn mul(self, v: $b) -> Self::Output {
                Self::Output::from(self.to_float() * v.to_float())
            }
        }
    }
}


mk_unit!(Current,     "A");
mk_unit!(Voltage,     "V");
mk_unit!(Power,       "W");
mk_unit!(Resistance,  "Ω");
mk_unit!(Duration,    "s");
mk_unit!(Charge,      "C");
mk_unit!(Energy,      "J");
mk_unit!(Temperature, "K");

gen_operators!(Power, Voltage, Current);
gen_operators!(Voltage, Current, Resistance);
gen_operators!(Charge, Current, Duration);
gen_operators!(Energy, Power, Duration);

