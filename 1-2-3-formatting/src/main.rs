mod city {
    use std::fmt::{self, Formatter, Display};

    struct City {
        name: &'static str,

        /// latitude
        lat: f32,

        /// longitude
        lon: f32,
    }

    impl City {
        pub fn new(name: &'static str, lat: f32, lon: f32) -> City {
            City { name, lat, lon }
        }
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(f, "{}: {:.3}{} {:.3}{}",
                self.name,
                self.lat.abs(), lat_c,
                self.lon.abs(), lon_c,
            )
        }
    }

    pub fn main() {
        for city in [
            City::new("Dublin", 53.347778, -6.259722),
            City::new("Oslo", 59.95, 10.75),
            City::new("Vancouver", 49.25, -123.1),
        ].iter() {
            println!("{}", *city);
        }
    }
}

mod color {
    use std::fmt::{self, Formatter, Display};

    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.r, self.g, self.b)
        }
    }

    pub fn main() {
        for color in [
            Color { r: 0x80, g: 0xFF, b: 0x5A },
            Color { r: 0x00, g: 0x03, b: 0xFE },
            Color { r: 0x00, g: 0x00, b: 0x00 },
        ].iter() {
            println!("{}", *color);
        }
    }
}

fn main() {
    println!("city----------------------------");
    city::main();
    println!("color---------------------------");
    color::main();
}
