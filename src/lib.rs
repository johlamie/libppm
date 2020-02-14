// cargo new ppm --lib pour la création de la librairie
// cargo fmt --> to indent code
// cargo clippy ->

// #![feature(test)] //
// extern crate test; 

use std::fmt;
use std::fs::File; // for reading the file
use std::path::Path;
use std::io::{self, BufReader};
use std::io::prelude::*;

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {

    fn newPixel(red: u8, green: u8, blue: u8) -> Pixel {
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
 // Difference between self minicule and masjuscule
impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

fn mysubstring(line: &String) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::new();
    let mut n = String::new();
    for c in line.chars() {
        if c == ' ' || c == '\0' {
            if !n.is_empty() {
                list.push(n.parse().unwrap());
                n.clear();
            }
        } else if c.is_digit(10){
            n.push(c);
        }
    }
    // Add if a numer is at the end of the line
    if !n.is_empty() {
        list.push(n.parse().unwrap());
    }
    return list;
}

// Get the RGB values ​​on the line and put it in an array that we can use.
fn getcolorOnLine(line: &String) -> Vec<u8>{
    let mut list_num: Vec<u8> = Vec::new();
    let mut n = String::new();
    for c in line.chars() {
        if c == ' ' || c == '\0' {
            if !n.is_empty() {
                list_num.push(n.parse().unwrap());
                n.clear();
            }
        } else if c.is_digit(10){
            n.push(c);
        }
    }
    if !n.is_empty() {
        list_num.push(n.parse().unwrap());
    }
    return list_num;
}

struct Image {
    buffer: Vec<Pixel>,
    width: u32,
    height: u32,
}

fn new_with_file(filename: &Path) -> Option<Image>{
    let mut img = Image {
        width: 0,
        height: 0,
        buffer : Vec::new()
    };
    let mut buffer: Vec<Pixel> = Vec::new();
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    let mut file = File::open(filename).expect("Can't open file!"); // Opening the file
    let buf = BufReader::new(file); // Initialization of the buffer for reading lines

    for (i, line) in buf.lines().enumerate().by_ref() {// i represents the line (starts at 0) and line the value of the line
        // let l = line.unwrap(); // A SUPP
        // Check that the first line tells us that it is a P3 before continuing
        if i == 0 { 
            if (&line.unwrap() != "P3") {
                return None;
            }
            // The second line is the dimensions of the picture.
        }else if (i == 1){
            let index: Vec<u32> = mysubstring(&line.unwrap());
            width = index[0];
            height = index[1];
        }else {
            let l = &line.unwrap();
            if (l.chars().next().unwrap() != '#') || (i != 2) {
                let pix = getcolorOnLine(&l);
                if pix.len() == 3 {
                    let c = Pixel::newPixel(pix[0], pix[1], pix[2]);
                    buffer.push(c);
                }

            }
        }
    }
   
    return Some(Image::new(width, height, buffer));
}

impl Image{

    pub fn new(width: u32, height: u32, buffer: Vec<Pixel>) -> Image {
        Image { width, height, buffer }
    }

    // Save our image in a ppm file respecting the structure
    fn save(&self,filename: &Path) {
            // We give it the type of the file, the width, height and the maximum value
            let mut str: String = format!("P3\n{} {}\n255\n", self.width, self.height);
    
            for p in self.buffer.iter() {
                // We give it the pixels r, g and b
                str = str + format!("{} {} {}\n", p.r, p.g, p.b).as_ref();
            }
            
            // File creation then writing
            let mut file = File::create(filename).expect("error");
            file.write_all(str.as_ref());

    }

    // Inverting an image using the pixel inversion function
    fn invert(&mut self){
        for pixel in self.buffer.iter_mut() {
            pixel.invert();
        }
    }

    // Black & White of an image using the grayscale function of the pixels
    fn grayscale(&mut self) {
        for pixel in self.buffer.iter_mut() {
            pixel.grayscale();
        }
    }
}





//----------------------------------------------Benchmark Tests---------------------------------------------------------------------------

/*
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn testPixel() {
        // let pixel = new Pixel(255,255,255);
    }
}*/


#[cfg(test)]
mod tests{
  use super ::*;
  #[test]
/*  fn is_grayscale()
  {
      let pix= Pixel :: newPixel(200,30,50);
      assert_eq!((60,17,23),pix.grayscale())
  }
  
  fn it_invert()
  {
      let pix= Pixel :: newPixel(200,30,50);
      //let pix_res= Pixel :: newPixel(55,225,205);
      assert_eq!((55,225,205),pix.invert())
          }
*/

  fn it_getGreen(){
    let pix = Pixel :: newPixel(200,30,50);
    assert_eq!((30), pix.getGreen())
}
  fn it_getBlue(){
    let pix = Pixel :: newPixel(200,30,50);
    assert_eq!((50), pix.getBlue())
}
  fn it_getRed(){
    let pix = Pixel :: newPixel(200,30,50);
    assert_eq!((200),pix.getRed())
}

      
}