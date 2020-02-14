// cargo new ppm --lib pour la création de la librairie
// cargo fmt --> to indent code
// cargo clippy -> 
use std::fmt;
use std::fs::File; // pour la lecture du fichier
use std::path::Path;
use std::io::{self, BufReader};
use std::io::prelude::*;

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

fn new_with_file(filename: &Path) -> Image{
    let mut file = File::open(filename).expect("Can't open file!"); // Ouverture du fichier
    let buffer = BufReader::new(file); // Initialisation du buffer pour la lecture des lignes 

    // let mut buffer: Vec<Pixel> = Vec::new();
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for (i, line) in buffer.lines().enumerate().by_ref() {// i réprésente la ligne (commence à 0) et line la valeur de la ligne
        // let l = line.unwrap(); // A SUPP
        // Vérifier que la premiere ligne nous dis que c'est un P3 avant de continuer
        if i == 0 { 
            if &line.unwrap() != "P3" {
                return None;
            }
            // The second line is the dimensions of the picture.
        }else if (i == 1){
            let index: Vec<u32> = mysubstring(&line.unwrap());
            width = index[0];
            height = index[1];
        }else {
            // TODO
            // le cas ou on n'est sur le maximum, un commentaire ou une ligne qui contient l'information qu'on recherche
        }

    }

    
    
    // return new Images{content: Vec::new, height: 0, width: 0}
}