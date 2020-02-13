// cargo new ppm --lib pour la création de la librairie
// cargo fmt --> to indent code
// cargo clippy -> 
use std::fmt;

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct Images {
    buffer: Vec<Pixel>,
    width: u32,
    height: u32,
}

impl Pixel {

    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue,
        }
    }
    
    fn dispaly(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }

    // Pixel getteur
    fn getRed(&self) -> u8 {
        return self.r
    }

    fn getGreen(&self) -> u8 {
        return self.g;
    }

    fn getBlue(&self) -> u8 {
        return self.b;
    }

    fn invert(&mut self){
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    // Function to compare two pixel structures
    fn eq(self, other: Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

    // To carry out the conversion, we will modify the level R, G and B ((0.3 * R) + (0.59 * G) + (0.11 * B)) 
    fn grayscale(&mut self){
        let gray = (self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11) as u8 ;

        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

}

 // Rewrite of function code eq to implement PartialEq Doc PartialEq!
 // Différence entre self minicule et masjuscule
impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}