use std::{env, ffi::OsStr, fs::read_dir, path::Path};
use webp::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let deepOption = args.contains(&"deep".to_string());
    get_all_files(".", deepOption);
}

fn get_all_files(path: &str, deep: bool) {
    let files = read_dir(path).unwrap();

    for file in files {
        let unwrapFile = file.unwrap();
        let pathFile = unwrapFile.path();

        if unwrapFile.file_type().unwrap().is_dir() && deep {
            get_all_files(unwrapFile.path().display().to_string().as_str(), deep);
        } else if !pathFile.extension().is_none() {
            if OsStr::new("png") == pathFile.extension().unwrap() {
                let displayPath = pathFile.as_os_str();
                let img = image::open(unwrapFile.path()).unwrap();

                let encoder: Encoder = Encoder::from_image(&img).unwrap();
                // Encode the image at a specified quality 0-100
                let webp: WebPMemory = encoder.encode(90f32);
                // Define and write the WebP-encoded file to a given path
                let output_path = Path::new(&unwrapFile.path()).with_extension("webp");
                std::fs::write(&output_path, &*webp).unwrap();
                println!("Converted => {:?}", displayPath);
            }
        }
    }
}
