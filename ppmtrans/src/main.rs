use clap::Parser;
use array2::Array2;
use csc411_image::{Read, Write, RgbImage, Rgb};
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Row-Major
    #[clap(long="row-major")]
    row_major: bool,
    // Col-Major
    #[clap(long="col-major")]
    col_major: bool,
    // Flip
    #[clap(short='f', long="flip")]
    flip: Option<String>,
    // Rotate
    #[clap(short='r', long="rotate")]
    rotate: Option<u32>,
    // Transpose
    #[clap(long="transpose")]
    transpose: bool,
    // File Name
    input_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = RgbImage::read(args.input_file.as_deref()).unwrap();
    let image = Array2::from_row_major(input.width as usize, input.height as usize, input.pixels).unwrap();

    // Flip Check
    if args.flip.as_deref() != None { flip(image.clone(), args.flip.clone(), args.row_major, input.denominator); }

    // Rotate Check
    if args.rotate != None { rotation(image.clone(), args.rotate.clone(), args.row_major, input.denominator); }

    // Transpose Check
    if args.transpose { transpose(image.clone(), args.row_major, input.denominator); }
}

fn flip (mut image: Array2<Rgb>, flip: Option<String>, is_row_major: bool, denom: u16) {
    // Horizontal Flip
    if flip.as_deref() == Some("horizontal") {
        let _start = Instant::now();
        image.flip_horizontal(is_row_major);
       // eprintln!("Horizontal flip took {:.8} seconds.",start.elapsed().as_secs_f64());

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
        
        if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // Vertical Flip
    if flip.as_deref() == Some("vertical") {
        let _start = Instant::now();
        image.flip_vertical(is_row_major);
        // eprintln!("Vertical flip took {:.8} seconds.",start.elapsed().as_secs_f64());

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
        
        if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }
}

fn rotation (mut image: Array2<Rgb>, rotate: Option<u32>, is_row_major: bool, denom: u16) {
    // 0 Degree Rotation
    if rotate == Some(0) {
        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
        if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // 90 Degree Rotation
    if rotate == Some(90) {
        let _start = Instant::now();
        image.rotate_90(is_row_major);
        // eprintln!("90 degree rotation took {:.8} seconds.",start.elapsed().as_secs_f64());

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
        
        if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // 180 Degree Rotation
    if rotate == Some(180) {
        let _start = Instant::now();
        image.rotate_180(is_row_major);
        // eprintln!("180 degree rotation took {:.8} seconds.",start.elapsed().as_secs_f64());

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
        
        if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // 270 Degree Rotation
    if rotate == Some(270) {
        let _start = Instant::now();
        image.rotate_270(is_row_major);
        // eprintln!("270 degree rotation took {:.8} seconds.",start.elapsed().as_secs_f64());

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
        
        if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }
}

fn transpose (mut image: Array2<Rgb>, is_row_major: bool, denom: u16) {
    let _start = Instant::now();
    image.transpose(is_row_major);
    // eprintln!("Transposition took {:.8} seconds.",start.elapsed().as_secs_f64());

    let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.width() as u32, height: image.height() as u32, denominator: denom };
    
    if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
}
