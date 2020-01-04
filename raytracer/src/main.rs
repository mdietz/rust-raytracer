extern crate bmp;
use bmp::Image;

fn main() {
    let img = Image::new(100,100);
    let _ = img.save("out.bmp");
}
