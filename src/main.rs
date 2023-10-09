use image;
use reqwest;
use std::{env, error::Error, io};
use wallpaper;
fn set_wallpaper(internet: bool) -> Result<(), Box<dyn Error>> {
    if internet == true {
        println!("LINK??????");
        let mut input_text = String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
        let img_bytes = reqwest::blocking::get(input_text.trim())?.bytes()?;

        let img = image::load_from_memory(&img_bytes)?;

        // Save the image to a file
        img.save("image.png")?;
    }
    // Download the image

    // Set the wallpaper
   
    if let Ok(current_exe) = env::current_exe () {
        if let Some(dir) = current_exe.parent() {
            if let Some(path) = dir.to_str() {
                println!("Current directory: {}", path);
                if internet == true{
                wallpaper::set_from_path(&(path.to_owned() + "\\image.png"))?;}
                
                else{
                    println!("where is ur image located?");
                    let mut input_text = String::new();
                    io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");
                    println!("{}",input_text);
                    wallpaper::set_from_path(&(path.to_owned() + "\\" + &input_text.trim()))?;
                }
            }
        }
    }
    
    // Set the wallpaper mode
    wallpaper::set_mode(wallpaper::Mode::Crop)?;

    // Print the wallpaper path
    println!("{:?}", wallpaper::get()?);

    Ok(())
}

fn main() {
    println!("Would you like to change your wallpaper from link[0] or image[1]?");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let trimmed = input_text.trim();
    let nice = match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => panic!("this was not an integer: {}", trimmed),
    };
    let true_false = if nice == 0{true}else{false};
   
    if let Err(err) = set_wallpaper(true_false) {
        eprintln!("Error: {}", err);
    } 
}
