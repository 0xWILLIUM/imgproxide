
use image::{ImageBuffer, Luma};
// use Str
// use image
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    // println!("Hello, world!");
    let img = image::open("src/fun.jpg").expect("image not found.");
    // let img: DynamicImage = image::open("src/fun.jpg").expect("Failed to open image.");
    
    let img_gray = img.to_luma8();
    let sobel_img = sobel(img_gray);
    // let hogs_img = hogs(img_gray);
    sobel_img.save("src/output.png").expect("Failed to save image.");
}

fn sobel(input : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>>{
    let height = input.height() - 2;
    let width = input.width() - 2;
    let mut buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for y in 1..height-1 {
        for x in 1..width-1 {
            let val0 = input.get_pixel(x-1, y-1).0[0] as i32; // [x-1, y-1]
            let val1 = input.get_pixel(x, y-1).0[0] as i32; // [x, y-1]
            let val2 = input.get_pixel(x+1, y-1).0[0] as i32; // [x+1, y-1]
            let val3 = input.get_pixel(x-1, y).0[0] as i32; // [x-1, y]
            let _val4 = input.get_pixel(x, y).0[0] as i32; // [x, y]
            let val5 = input.get_pixel(x+1, y).0[0] as i32; // [x+1, y]
            let val6 = input.get_pixel(x-1, y-1).0[0] as i32; // [x-1, y+1]
            let val7 = input.get_pixel(x, y).0[0] as i32; // [x, y+1]
            let val8 = input.get_pixel(x+1, y+1).0[0] as i32; // [x+1, y+1]
            
            let gx : i32 = val0 + (val3 * 2) + val6
                - (val2 + (val5 * 2) + val8);
            let gy : i32 = val0 + (2 * val2) + val1
                - (val6 + (2 * val7) + val8);

            let g = ((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt();
            buffer.put_pixel(x, y, Luma([g as u8]));
        }
    }

    buffer
}


fn hogs(input : ImageBuffer<Luma<u8>, Vec<u8>>) {
    let hists = hogs_calc_hists(input);

    
}

fn hogs_calc_hists(input : ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<[f64; 9]>{
    // thank you https://builtin.com/articles/histogram-of-oriented-gradients
    let height = input.height();
    let width = input.width();
    // let mut buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut gy_buffer : ImageBuffer<Luma<i16>, Vec<i16>> = ImageBuffer::new(width, height);
    let mut gx_buffer : ImageBuffer<Luma<i16>, Vec<i16>> = ImageBuffer::new(width, height);
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let val0 = input.get_pixel(x-1, y).0[0] as i32; // [x-1, y]
            let val2 = input.get_pixel(x+1, y).0[0] as i32; // [x+1, y]
            gx_buffer.put_pixel(x, y, Luma([(-val0 + val2) as i16]));
            
            let val0 = input.get_pixel(x, y-1).0[0] as i32; // [x-1, y]
            let val2 = input.get_pixel(x, y+1).0[0] as i32; // [x+1, y]
            gy_buffer.put_pixel(x, y, Luma([(-val0 + val2) as i16]));

        }
    }
    
    let mut angles : Vec<f64> = Vec::new();
    let mut mags : Vec<f64> = Vec::new();

    for (gx, gy) in gx_buffer.iter().zip(gy_buffer.iter()) {
        let gx = *gx as f64;
        let gy = *gy as f64;
        angles.push(gy.atan2(gx).to_degrees());
        mags.push((gx * gx + gy * gy).sqrt());
    }

    // calculate the normalised histogram values of each cell of the image 
    let mut normed_hists : Vec<[f64; 9]> = Vec::new();
    
    for y in 0.. (height-1) / 8 {
        for x in 0.. (width-1) / 8 {

            let mut hist : [f64; 9] = [0.0; 9];
            // can this be done through the use of a zip with mag + dir?
            for i in 0..8 {
                for j in 0..8 {
                    let index = (((y * 8 + i) * width) + (x * 8) + j) as usize;
                    let magnitude = mags[index];
                    let angle = angles[index];
                    let bin = (angle / 20.0).floor() as usize;
                    
                    let share = angle % 20.0 / 20.0;
                    hist[bin] += (1.0 - share) * magnitude;
                    hist[(bin+1) % 9] += share * magnitude;

                }
            }

            let sum : f64 = hist.iter().sum();
            if sum > 0.0 {
                // normalise!
                for val in hist.iter_mut() {
                    *val /= sum;
                }
            }

            normed_hists[(y * (width / 8) + x) as usize] = hist;
        }
    }

    normed_hists
}