use std::{io, fs};

fn main() {

    let tf2_dir = "E:\\SteamLibrary\\steamapps\\common\\Team Fortress 2\\tf";
    let sfm_dir = "E:\\SteamLibrary\\steamapps\\common\\SourceFilmmaker\\game\\tf";
    // locate the tf2 and sfm directories.
    let paths = fs::read_dir("./").unwrap();
    println!("Your TF2 directory sits at {}", tf2_dir);
    println!("Your SFM directory sits at {}", sfm_dir);

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    // Ask the user if they wanna change to a different directory
    // let mut input = String::new();
    // match io::stdin().read_line(&mut input) {
    //     Ok(n) => {
    //         println!("{} bytes read", n);
    //         println!("{}", input);
    //     }
    //     Err(error) => println!("error: {error}"),
    // }
    println!("Hello, world!");
}
