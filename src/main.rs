
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

fn hogs(input : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>>{
    // thank you https://builtin.com/articles/histogram-of-oriented-gradients
    let height = input.height();
    let width = input.width();
    // let mut buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut gy_buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut gx_buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for y in 1..height-1 {
        for x in 1..width-1 {
            let val0 = input.get_pixel(x-1, y).0[0] as i32; // [x-1, y]
            let val2 = input.get_pixel(x+1, y).0[0] as i32; // [x+1, y]
            gx_buffer.put_pixel(x, y, Luma([(-val0 + val2) as u8]));
            
            let val0 = input.get_pixel(x, y-1).0[0] as i32; // [x-1, y]
            let val2 = input.get_pixel(x, y+1).0[0] as i32; // [x+1, y]
            gy_buffer.put_pixel(x, y, Luma([(-val0 + val2) as u8]));

        }
    }
    
    // let g_mags : Vec<u8> = gx_buffer.iter().zip(gy_buffer.iter())
    // .map(|(gx, gy)| (((*gx as i32).pow(2) + (*gy as i32).pow(2)) as f64).sqrt() as u8) // could i x -> f64.. perform ops.. --> u8
    // .collect();
    
    let mut angles : Vec<f64> = Vec::new();
    let mut mags : Vec<u8> = Vec::new();

    let _ : Vec<_> = gx_buffer.iter().zip(gy_buffer.iter())
    .map(|(gx, gy)| {
        angles.push(((*gy as f64) / (*gx as f64)).atan());
        // mags.push((((*gx as i32).pow(2) + (*gy as i32)) .pow(2)).sqrt() as u8);
        mags.push((((*gx as i32).pow(2) + (*gy as i32).pow(2)) as f64).sqrt() as u8);
    }) // could i x -> f64.. perform ops.. --> u8
    .collect();
    // calculate the normalised histogram values of each cell of the image 

    for y in 0.. (height-1) / 8 {
        for x in 0.. (width-1) / 8 {
            let mut hist : [f64; 9] = [0.0; 9];

            for i in 0..8 {
                for j in 0..8 {
                    let index = (((y * 8 + i) * width) + (x * 8) + j) as usize;
                    let magnitude = mags[index];
                    let angle = angles[index];
                    let bin = (angle / 20.0).floor() as usize;
                    
                    let x = angle % 20 as f64;
                    hist[bin] += x * magnitude as f64;
                    hist[bin] += (20.0 - x) * magnitude as f64;
                    // need to add to the histogram for where mag > bin * 20 deg
                    // ahem j + 1 according to the article
                }
            }
        }
    }

    let gxy_buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, mags).expect("Failed to create image buffer.");
    gxy_buffer
}