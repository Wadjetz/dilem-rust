extern crate iron;
extern crate image;
extern crate router;
extern crate mount;
extern crate iron_send_file;
extern crate staticfile;
extern crate crypto;

use std::fs::File;
use std::path::Path;
use std::net::*;

use crypto::md5::Md5;
use crypto::digest::Digest;

use image::imageops::Nearest;
use image::imageops::Gaussian;
use image::GenericImage;

use router::Router;

use mount::Mount;

use staticfile::Static;

use iron::prelude::*;
use iron::status;
use iron_send_file::send_file;

// Unique hash for a filename and a blur level
fn unique_hash(path: &Path, blur_level: &u32) -> String {
    let filename = match path.file_name() {
        Some(name) => match name.to_str() {
            Some(x) => x,
            None    => ""
        },
        None       => ""
    };

    let mut hasher = Md5::new();
    hasher.input_str(&format!("{}_{}", filename, blur_level.to_string()));
    return hasher.result_str()
}

// Blur the provided picture
fn photo_process(path: &Path, blur_level: &u32) -> Path {
    let ref outputPath = &format!("./src/photos/processed/{}", unique_hash(path, blur_level));
    let output = Path::new(outputPath);

    if !output.exists() {
        let ref mut outputFile = File::create(output).unwrap();
        let img =  image::open(path).unwrap();
        let (w, h) = img.dimensions();
        
        let _ = img
                .resize(w / blur_level, h / blur_level, Nearest)
                .resize(w, h, Nearest)
                .save(outputFile, image::JPEG)
                .unwrap();
    }

    return output
}

// Endpoints

// Photo
fn photo(req: &mut Request) -> IronResult<Response> {
    let photo_processed = photo_process(&Path::new("./src/photos/trump.jpg"), &40);
    let res = Response::new();
    send_file(req, res, &photo_processed);
}

// Hello world
fn environment(_: &mut Request) -> IronResult<Response> {
    let powered_by:String = "Zengularity".to_string();
    let message = format!("Powered by: {}, pretty cool aye", powered_by);
    Ok(Response::with((status::Ok, message)))
}

fn main() {
    let mut router = Router::new();
    router.get("/hello_world", environment, "index");
    //router.get("/photo.jpg", photo, "photo");

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static", Static::new(Path::new("./src/static/")));

    let middleware = Chain::new(mount);

    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}
