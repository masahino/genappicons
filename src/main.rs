use image::GenericImageView;
use image::imageops::FilterType;
use std::env;
use std::fs;
use std::io;

fn main() {
    let icon_size: [u32; 31] = [16,20,29,32,40,48,50,55,57,58,60,64,72,76,80,87,88,100,114,120,128,144,152,167,172,180,196,216,256,512,1024];
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("filename: {}", filename);
    let img = image::open(filename).unwrap();
    println!("dimensions {:?}", img.dimensions());
    match fs::create_dir_all("AppIcons/Assets.xcassets/AppIcon.appiconset") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    // copy original image
    fs::copy(filename, "AppIcons/appstore.png");
    // copy playstore image
    let playstore_img = img.resize(512, 512, FilterType::Lanczos3);
    playstore_img.save("AppIcons/playstore.png").unwrap();

    // create resized icons
    for size in icon_size {
        let resized_img = img.resize(size, size, FilterType::Lanczos3);
        let resized_filename = format!("AppIcons/Assets.xcassets/AppIcon.appiconset/{}.png", size);
        resized_img.save(resized_filename).unwrap();
    }
}
