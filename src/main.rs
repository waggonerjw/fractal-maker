use image::{RgbImage, Rgb};

const ITERATIONS: u32 = 50;

fn fract(xpos: f64, ypos: f64, creal: f32, cimag: f32) -> f32 {
    let mut real = xpos * 2.0 - 1.0;
    let mut imag = ypos * 2.0 - 1.0;
    for i in 0..ITERATIONS {
        let newreal = real * real - imag * imag + (creal as f64);
        let newimag = 2.0 * real * imag + (cimag as f64);
        real = newreal;
        imag = newimag;
        if (real * real + imag * imag).sqrt() > 4.0 {
            return i as f32 / ITERATIONS as f32;
        };
    }
    return 0f32;
}

fn main() {
    if std::env::args().len() < 1 + 1 + 2 {
        println!("🦀Error: Too few args. Needs file name, dimensions, and optinally CReal and CImag and iterations.");
        return;
    }
    //get args
    let mut args = std::env::args();
    //pop executable path
    args.next();

    //get image file path. We checked earlier and know this is there.
    let path = args.next().unwrap();

    //get dimensions. We can skip the ckecking and unwrap because above
    let dim_x: u32; let dim_y: u32;
    if let Ok(parsed_x) = args.next().unwrap().parse() {
        dim_x = parsed_x;
    } else {
        println!("🦀Error: failed to parse dimension X. Exiting…");
        return;
    }
    if let Ok(parsed_y) = args.next().unwrap().parse() {
        dim_y = parsed_y;
    } else {
        println!("🦀Error: failed to parse dimension Y. Exiting…");
        return;
    }

    //get creal and cimag
    let creal: f32; let cimag: f32;
    if let Some(arg_creal) = args.next() {
        if let Ok(parsed_creal) = arg_creal.parse() {
            creal = parsed_creal;
        } else { creal = 0.0; }
    } else { creal = 0.0; }
    if let Some(arg_cimag) = args.next() {
        if let Ok(parsed_cimag) = arg_cimag.parse() {
            cimag = parsed_cimag;
        } else { cimag = 0.0; }
    } else { cimag = 0.0; }


    println!("🦀: Generating image buffer…");
    let img: RgbImage = RgbImage::from_fn(dim_x, dim_y, |x: u32, y: u32|{
        //our generating function
        //get x and y to be between 0 and 1. Easy.
        let xpos = x as f64 / dim_x as f64;
        let ypos = y as f64 / dim_y as f64;
    
        //use our fract function to calculate the grayscale value of the pixel
        let fractval = fract(xpos, ypos, creal, cimag);
        
        //turn that grayscale value into our r g and b components
        let r = (fractval * 255.0) as u8;
        let g = (fractval * 255.0) as u8;
        let b = (fractval * 255.0) as u8;
        //return the pixel
        Rgb([r, g, b])
    });
    println!("🦀: Image buffer generated.");

    println!("🦀: Saving image…");
    img.save(path).expect("🦀Error: Failed to save image!");
    println!("🦀: Image saved.")
}