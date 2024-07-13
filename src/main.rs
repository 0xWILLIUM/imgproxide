
use image::{ImageBuffer, Luma};

// use Str
// use image
fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    // println!("Hello, world!");
    let img = image::open("src/fun.jpg").expect("image not found.");
    let img_gray = img.to_luma8();
    let sobel_img = sobel(img_gray);
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
            
            let gx = val0 + (-1 * val2)
                + (2 * val3) + (-2 * val5)
                + (1 * val6) + (-1 * val8);
            
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