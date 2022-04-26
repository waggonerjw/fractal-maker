use image::{RgbImage, Rgb};

fn fract(xpos: f64, ypos: f64, creal: f32, cimag: f32, iterations: u32) -> f32 {
    let mut real = xpos * 2.0 - 1.0;
    let mut imag = ypos * 2.0 - 1.0;
    for i in 0..iterations {
        let newreal = real * real - imag * imag + (creal as f64);
        let newimag = 2.0 * real * imag + (cimag as f64);
        real = newreal;
        imag = newimag;
        if (real * real + imag * imag).sqrt() > 4.0 {
            return i as f32 / iterations as f32;
        };
    }
    return 0f32;
}

fn main() {
    let config: Config;
    match parse_args() {
        Ok(conf) => { config = conf; }
        Err(()) => { return; }
    }

    println!("🦀: Generating image buffer…");
    let img: RgbImage = RgbImage::from_fn(config.dimensions.x, config.dimensions.y, |x: u32, y: u32|{
        //our generating function
        //get x and y to be between 0 and 1. Easy.
        let xpos = x as f64 / config.dimensions.x as f64;
        let ypos = y as f64 / config.dimensions.y as f64;
    
        //use our fract function to calculate the grayscale value of the pixel
        let fractval = fract(xpos, ypos, config.creal, config.cimag, config.iterations);
        
        //turn that grayscale value into our r g and b components
        let r = (fractval * 255.0) as u8;
        let g = (fractval * 255.0) as u8;
        let b = (fractval * 255.0) as u8;
        //return the pixel
        Rgb([r, g, b])
    });
    println!("🦀: Image buffer generated.");

    println!("🦀: Saving image…");
    img.save(config.path).expect("🦀Error: Failed to save image!");
    println!("🦀: Image saved.")
}

pub struct Config {
    path: String,
    dimensions: Dimensions,
    creal: f32,
    cimag: f32,
    iterations: u32
}
struct Dimensions { x: u32, y: u32 }

pub fn parse_args() -> Result<Config, ()> {
    if std::env::args().len() < 1 /*executable path*/ + 1 /*img file path*/ + 2 /*Dimensions*/ {
        println!("🦀Error: Too few args.");
        println!("🦀Help: syntax follows: <program path> <output file path>\
            <file dimensions x> <file dimensions y> [c real val] [c imag value] [max iterations]");
        println!("🦀Help: Example: path/to/program myimage.png 1024 1024 -1 0 50");
        return Err(());
    }
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
        println!("🦀Error: Failed to parse dimension x. Dimensions are nessecary. Exiting…");
        return Err(());
    }
    if let Ok(parsed_y) = args.next().unwrap().parse() {
        dim_y = parsed_y;
    } else {
        println!("🦀Error: Failed to parse dimension y. Dimensions are nessecary. Exiting…");
        return Err(());
    }

    //get creal and cimag
    let creal: f32; let cimag: f32;
    if let Some(arg_creal) = args.next() {
        if let Ok(parsed_creal) = arg_creal.parse() {
            creal = parsed_creal;
        } else {
            println!("🦀Warning: Failed to parse c real value. Initializing default value…");
            creal = 0.0;
        }
    } else { creal = 0.0; }
    if let Some(arg_cimag) = args.next() {
        if let Ok(parsed_cimag) = arg_cimag.parse() {
            cimag = parsed_cimag;
        } else {
            println!("🦀Warning: Failed to parse c imag value. Initializing default value…");
            cimag = 0.0;
        }
    } else { cimag = 0.0; }

    //get iterations
    let iterations: u32;
    if let Some(arg_iterations) = args.next() {
        if let Ok(parsed_iterations) = arg_iterations.parse() {
            iterations = parsed_iterations;
        } else { iterations = 50; }
    } else { iterations = 50; }

    //put it all into the struct
    Ok(Config {
        //shorthand for path: path
        path,
        dimensions: Dimensions { x: dim_x, y: dim_y },
        creal,
        cimag,
        iterations
    })
}