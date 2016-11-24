extern crate iron;
extern crate image;
extern crate iron_send_file;

use std::fs::File;
use std::path::Path;

use image::imageops::Nearest;
use image::imageops::Gaussian;
use image::GenericImage;

use iron::prelude::*;
use iron_send_file::send_file;

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let img = image::open(&Path::new("/Users/ebe/projects/dilem/owl-logo.jpg")).unwrap();
    let ref mut fout = File::create(&Path::new("/tmp/owl-logo.jpg")).unwrap();
    let (w, h) = img.dimensions();
    println!("{:?}", (w, h));
    let coef_reducer = 40;
    let _ = img
            .resize(w / coef_reducer, h / coef_reducer, Nearest)
            .resize(w, h, Nearest)
            .save(fout, image::JPEG)
            .unwrap();
    let path = Path::new("/tmp/owl-logo.jpg");
    let res = Response::new();
    send_file(req, res, path)
}

fn main() {
    let chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:3000").unwrap();
}
