use std::{io::{self, BufReader}, fs::{self, File}};

use vpk;

fn main() {

    let mut tf2_dir = "E:\\SteamLibrary\\steamapps\\common\\Team Fortress 2\\tf".to_owned();
    let sfm_dir = "E:\\SteamLibrary\\steamapps\\common\\SourceFilmmaker\\game\\tf";

    // TODO: auto-detect the tf2 and sfm directories. or ask user for input if auto-detection fails
    println!("Your TF2 directory sits at {}", tf2_dir);
    println!("Your SFM directory sits at {}", sfm_dir);


    tf2_dir.push_str("\\tf2_misc_dir.vpk");
    read_vpk_file(&tf2_dir);
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


fn read_vpk_file(path: &str) {

    let destination = "C:\\Users\\the linux drive\\Downloads\\test";
    
    // let file = File::open(path).expect("Unable to open file");
    
    // Load the VPK archive
    // let mut vpk = vpk::from_path(path).expect("Failed to load VPK archive");
    
    match vpk::from_path(path) {
        Ok(archive) => {
            // Iterate through the files and extract them
            for file_entry in archive.tree {
                let mut tf2_dir = "E:/SteamLibrary/steamapps/common/Team Fortress 2/tf/".to_owned();
                let file_name = file_entry.0;
                // let data = file_entry.1;
                
                tf2_dir.push_str(&file_name);
                println!("Copy directory: {}",tf2_dir);
                println!("Copy destination: {}",destination);
                fs::copy(tf2_dir, destination).expect("could not copy");
                // match file_entry {
                //     vpk::entry::VPKEntry(file) => {
                //         println!("test file {:?}",file)
                //     }
                //     vpk::entry::VPKDirectoryEntry(directory) => {

                //     }
                // }

                // println!("file data: {:?}",file_entry);
                // let file_data = file_entry.1.get();
                // fs::write(destination, file_entry).expect("balls")

                // match file_data {
                //     Ok(data) => println!("{:?}",data),
                //     Err(e) => eprintln!("FUCK", e.),
                // }
                // Process the extracted file data here (e.g., save to disk)
                // Example: Save the file to disk
                // std::fs::write(format!("extracted/{}", file_entry.path()), file_data)
                //     .expect("Failed to write file to disk");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

}