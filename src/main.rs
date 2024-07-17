
use std::{env, fs};
use image::{ImageBuffer, Luma};
use linfa::dataset::Dataset;
use ndarray::{Array1, Array2};

// use Str
// use image
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut features = Vec::new();
    // let mut labels = Vec::new();
    let img_path = "src/chow/";
    let files = fs::read_dir(img_path)
        .unwrap();
    // let labelss = fs::read_dir(lbl_path)
    //     .unwrap();
    let mut labels = Vec::new();
    for file in files {
        println!("{:?}", file);
        let file = file.unwrap();
        let path = file.path();
        let img = image::open(path).expect("image not found.");
        let img_gray = img.to_luma8();
        let hog_features = hogs(&img_gray);
        features.push(hog_features);
        labels.push(1);
    }

    let n_samples = features.len();
    let n_features = features[0].len();
    let features : Array2<Vec<f64>> = Array2::from_shape_vec((n_samples, n_features), features.into_iter().flatten().collect()).unwrap();
    let labels = Array1::from(labels);
    let _dataset = Dataset::new(features, labels);

}

fn _sobel(input : ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>>{
    let height = input.height();
    let width = input.width();
    let mut buffer : ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for y in 1..height-1 {
        for x in 1..width-1 {
            let val0 = input.get_pixel(x-1, y-1).0[0] as i32; // [x-1, y-1]
            let val1 = input.get_pixel(x, y-1).0[0] as i32; // [x, y-1]
            let val2 = input.get_pixel(x+1, y-1).0[0] as i32; // [x+1, y-1]
            let val3 = input.get_pixel(x-1, y).0[0] as i32; // [x-1, y]
            let val5 = input.get_pixel(x+1, y).0[0] as i32; // [x+1, y]
            let val6 = input.get_pixel(x-1, y-1).0[0] as i32; // [x-1, y+1]
            let val7 = input.get_pixel(x, y).0[0] as i32; // [x, y+1]
            let val8 = input.get_pixel(x+1, y+1).0[0] as i32; // [x+1, y+1]
            
            let gx : i32 = val0 + (val3 * 2) + val6
                - (val2 + (val5 * 2) + val8);
            let gy : i32 = val0 + (2 * val1) + val2
                - (val6 + (2 * val7) + val8);

            let g = ((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt();
            buffer.put_pixel(x, y, Luma([g as u8]));
        }
    }

    buffer
}


fn hogs(input : &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<Vec<f64>> {
    let (hists, blocks_x, blocks_y) = hogs_calc_hists(input);
    let mut features = vec![vec![0.0; 36]; blocks_x * blocks_y];

    for y in 0.. blocks_y-1 {
        for x in 0..blocks_x-1 {
            let mut feature_vec = [0.0; 36];
            let indices = [
                (y * blocks_x + x),
                (y * blocks_x + x + 1),
                ((y+1) * blocks_x + x),
                ((y+1) * blocks_x + x + 1)];
            
            for i in 0..4 {
                let vals = &hists[indices[i]];
                for (j, &val) in vals.iter().enumerate() {
                    feature_vec[i * 9 + j] = val;
                }
            }

            let norm = feature_vec.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            feature_vec.iter_mut().for_each(|x| *x /= norm + 0.00001);
            features[y * blocks_x + x] = feature_vec.to_vec();
        }
    }
    features
}

fn hogs_calc_hists(input : &ImageBuffer<Luma<u8>, Vec<u8>>) -> (Vec<[f64; 9]>, usize, usize) {
    // thank you https://builtin.com/articles/histogram-of-oriented-gradients
    let height = input.height();
    let width = input.width();
    let mut gy_buffer : ImageBuffer<Luma<i16>, Vec<i16>> = ImageBuffer::new(width, height);
    let mut gx_buffer : ImageBuffer<Luma<i16>, Vec<i16>> = ImageBuffer::new(width, height);
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let val0 = input.get_pixel(x-1, y).0[0] as i16; // [x-1, y]
            let val2 = input.get_pixel(x+1, y).0[0] as i16; // [x+1, y]
            gx_buffer.put_pixel(x, y, Luma([-val0 + val2]));
            
            let val0 = input.get_pixel(x, y-1).0[0] as i16; // [x-1, y]
            let val2 = input.get_pixel(x, y+1).0[0] as i16; // [x+1, y]
            gy_buffer.put_pixel(x, y, Luma([-val0 + val2]));

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
    // let mut normed_hists : Vec<[f64; 9]> = Vec::new();
    let cells_y = ((height-1) / 8) as usize;
    let cells_x = ((width-1) / 8) as usize;
    let mut hists : Vec<[f64; 9]> = vec![[0.0; 9]; cells_x * cells_y];
    for y in 0..  cells_y {
        for x in 0.. cells_x {

            let mut hist : [f64; 9] = [0.0; 9];
            // can this be done through the use of a zip with mag + dir?
            for i in 0..8 {
                for j in 0..8 {
                    let index = ((y * 8 + i) * width as usize + (x * 8) + j) as usize;
                    let magnitude = mags[index];
                    let angle = angles[index];
                    let bin = (angle / 20.0).floor() as usize;
                    
                    let share = angle % 20.0 / 20.0;
                    hist[bin%9] += (1.0 - share) * magnitude;
                    hist[(bin+1) % 9] += share * magnitude;

                }
            }
            hists[y * cells_x + x] = hist;
        }
    }
    (hists, cells_x, cells_y)
}