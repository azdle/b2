extern crate image;
extern crate rand;
extern crate rustc_serialize;
extern crate docopt;

use image::{GenericImage, ImageBuffer, Rgba};
use rand::Rng;

use std::process;

use docopt::Docopt;

const USAGE: &'static str = "
B2 - Set your pins to B2

A tool to XOR images together.

Usage:
  b2 makenoise <file> <outfile>
  b2 xor <file> <file2> <outfile>
  b2 (-h | --help)
  b2 (-v | --version)

Options:
  -h --help     Show this screen.
  -v --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    arg_file2: String,
    arg_outfile: String,
    cmd_makenoise: bool,
    cmd_xor: bool,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
                          .and_then(|d| d.decode())
                          .unwrap_or_else(|e| e.exit());

  if args.cmd_makenoise {
    /*************************************************
    * 
    * makenoise command
    *
    *************************************************/

    // setup random number generator
    let mut rng = rand::thread_rng();
    let mut r = rng.gen_iter::<u8>();
    // get dimensions of reference picture
    let (imgx, imgy) = {
      let img = image::open(&args.arg_file).unwrap_or_else(|e| {
        println!("Invalid File `{}`: {}", &args.arg_file, e);
        std::process::exit(-1);
      });

      img.dimensions()
    };

    let mut imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(imgx, imgy);

    for pixel in imgbuf.pixels_mut() {
        *pixel = Rgba([r.next().unwrap(), r.next().unwrap(), r.next().unwrap(), r.next().unwrap()]);
    }

    imgbuf.save(&args.arg_outfile).unwrap();
  } else if args.cmd_xor {
    /*************************************************
    * 
    * xor command
    *
    *************************************************/

    // get images to xor
    let img = image::open(&args.arg_file).unwrap_or_else(|e| {
      println!("Invalid File `{}`: {}", &args.arg_file , e);
      process::exit(-1);
    }).to_rgba();

    let img2 = image::open(&args.arg_file2).unwrap_or_else(|e| {
      println!("Invalid File `{}`: {}", &args.arg_file2, e);
      process::exit(-1);
    }).to_rgba();

    // ensure they're the same size
    if img.dimensions() != img2.dimensions() {
      println!("Image Dimensions Must Match: {:?} != {:?}", 
        img.dimensions(), img2.dimensions());
      process::exit(-2);
    }

    let (imgx, imgy) = img.dimensions();

    let mut imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(imgx, imgy);

    // xor each color(&alpha) channel for each pixel
    for (x , y, pixel) in imgbuf.enumerate_pixels_mut() {
      let Rgba{data: p1} = *img.get_pixel(x, y);
      let Rgba{data: p2} = *img2.get_pixel(x, y);
      *pixel = Rgba([p1[0] ^ p2[0], p1[1] ^ p2[1], p1[2] ^ p2[2], p1[3] ^ p2[3]]);
    }

    imgbuf.save(&args.arg_outfile).unwrap();
  } else {
    panic!("No Command Specified, This Shouldn't be Possible");
  }

}