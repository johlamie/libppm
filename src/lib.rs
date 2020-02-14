// cargo new ppm --lib pour la création de la librairie
// cargo fmt --> to indent code
// cargo clippy ->
// #![feature(test)] //
//extern crate test; 

use std::fmt;
use std::fs::File; // pour la lecture du fichier
use std::path::Path;
use std::io::{self, BufReader};



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

    let mut file = File::open(filename).expect("Can't open file!"); // Ouverture du fichier
    let buf = BufReader::new(file); // Initialisation du buffer pour la lecture des lignes 

    for (i, line) in buf.lines().enumerate().by_ref() {// i réprésente la ligne (commence à 0) et line la valeur de la ligne
        // let l = line.unwrap(); // A SUPP
        // Vérifier que la premiere ligne nous dis que c'est un P3 avant de continuer
        if i == 0 { 
            if (&line.unwrap() != "P3") {
                println!("None");
            }
            // The second line is the dimensions of the picture.
        }else if (i == 1){
            let index: Vec<u32> = mysubstring(&line.unwrap());
            width = index[0];
            height = index[1];
        }else {
            // TODO
            // ici que la variable buffer doit être remplie
            // le cas ou on n'est sur le maximum, un commentaire ou une ligne qui contient l'information qu'on recherche
        }
    }
   
    return Some(Image::new(width, height, buffer));
    // return new Images{content: Vec::new, height: 0, width: 0}
}

impl Image{

    pub fn new(width: u32, height: u32, buffer: Vec<Pixel>) -> Image {
        Image { width, height, buffer }
    }

    // Sauvegarde notre image dans un fichier ppm en respectant la structure
    fn save(&self,filename: &Path) {
            // On lui donne le type du fichier, le width, height et la valeur maximale
            let mut str: String = format!("P3\n{} {}\n255\n", self.width, self.height);
    
            for p in self.buffer.iter() {
                // On lui donne les pixels r,g et b
                str = str + format!("{} {} {}\n", p.r, p.g, p.b).as_ref();
            }
            
            // Création du fichier puis ecriture
            let mut file = File::create(filename).expect("error");
            file.write_all(str.as_ref());

    }

    // Inversion d'une image en utilisant la fonction inversion des pixels
    fn invert(&mut self){
        for pixel in self.buffer.iter_mut() {
            pixel.invert();
        }
    }

    // Mise en Noir & Blanc d'une image en utilisant la fonction grayscale des pixels
    fn grayscale(&mut self) {
        for pixel in self.buffer.iter_mut() {
            pixel.grayscale();
        }
    }

    // fn new_with_file_bin(filename: &Path ) -> Option<Image>{}


}





//----------------------------------------------Benchmark Tests---------------------------------------------------------------------------


/*#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn testPixel() {
        // let pixel = new Pixel(255,255,255);
    }
}
*/

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

