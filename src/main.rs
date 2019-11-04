extern crate image;
use image::GenericImageView;
use std::fs;

fn save_jpg_clean(path: &str) {
    println!("salvando {} em editadas", path);
    let img = image::open(path).unwrap();
    println!("dimensions: {:?}, color: {:?}\n",
        img.dimensions(),
        img.color()
    );
    img.save(format!("editadas/{}", path)).unwrap();
}

fn main() {
    let _ = fs::create_dir("editadas");
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let file = path.unwrap().path();
        let name = file.display();
        let extension = file.extension();
        match extension {
            None => {},
            Some(ext) => {
                if ext == "jpg" {
                    save_jpg_clean(&name.to_string());
                }
            }
        }
    }
}
