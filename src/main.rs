use std::{path::{Path, PathBuf},process::Command, fs};

fn main() {

    let tf2_dir = "E:/SteamLibrary/steamapps/common/Team Fortress 2".to_owned();
    let sfm_dir = "E:/SteamLibrary/steamapps/common/SourceFilmmaker/game/tf";
    // TODO: remove and write directly to SFM folder
    let destination = "E:/test";

    let mut vpk_exe_path = tf2_dir.clone().to_owned();

    vpk_exe_path.push_str("/bin/vpk.exe");

    // TODO: auto-detect the tf2 and sfm directories. or ask user for input if auto-detection fails
    println!("Your TF2 directory sits at {}", tf2_dir);
    println!("Your SFM directory sits at {}", sfm_dir);

    let vpk_file_paths = vec!["tf2_misc_dir.vpk", "tf2_textures_dir.vpk", "tf2_sound_misc_dir.vpk", "tf2_sound_vo_english_dir.vpk"];

    for vpk in vpk_file_paths {
        println!("\x1b[93m[SFM CONTENT PATCHER] extracting {vpk}...\x1b[0m");

        let mut vpk_path = tf2_dir.clone().to_owned();
        vpk_path.push_str("/tf/");
        vpk_path.push_str(vpk);
    
        extract_vpk_file(&vpk_path,&tf2_dir,destination);
    }


    println!("Hello, world!");
}


/**
 * get all file names to be extracted from the vpk file.
 */
fn get_file_names(vpk_file_path: &str, tf2_game_path: &str) -> Vec<String> {
    let mut file_names = Vec::new();

    let vpk_cmd_output = Command::new(format!("{}/bin/vpk.exe", tf2_game_path))
    .args(&["l", vpk_file_path])
    .output()
    .expect("Failed to execute vpk command");

    let output_lines = vpk_cmd_output.stdout.split(|&c| c == b'\n');

    for line in output_lines {
        if let Ok(utf8_line) = std::str::from_utf8(line) {
            if let Some(entry) = extract_entry(utf8_line) {
                file_names.push(entry.to_string());
            }
        }
    }
    return file_names;
}

fn extract_entry<'a>(entry: &'a str) -> Option<&'a str> {
    let prefixes = ["maps", "models", "materials", "particles", "sound"];
    for &prefix in &prefixes {
        if entry.starts_with(prefix) {
            return Some(entry);
        }
    }
    None
}

fn extract_vpk_file(vpk_file_path: &str,tf2_game_path: &str, destination: &str) {
    // let file_names = get_file_names(&vpk_file_path, &tf2_game_path);

    // let file_names = get_file_names(&vpk_file_path, &tf2_game_path);
    
    let vpk_file_name = vpk_file_path.split("/tf/").last().unwrap();
    
    println!("Output dir: {}", format!("{}/tf/{}",&tf2_game_path, &vpk_file_name));
    println!("vpk.exe file path: {}",format!("{}/bin/vpk.exe",&tf2_game_path));

    let mut vpk_cmd = Command::new(format!("{}/bin/vpk.exe",&tf2_game_path));
    vpk_cmd.arg(vpk_file_path);
    
    
    let split_name = vpk_file_name.split(".").collect::<Vec<&str>>();
    
    let output_dir_name = split_name.first().unwrap();

    let vpk_output_dir = format!("{}/tf/{}",&tf2_game_path,&output_dir_name);
    
    println!("output dir: {vpk_output_dir}");

    vpk_cmd.output().unwrap();
    
    let did_vpk_extract = Path::new(&vpk_output_dir.to_owned().to_string()).exists();

    if !did_vpk_extract {
        panic!("Something went wrong while extracting the VPK file.");
    }

    // move_extracted_files(&vpk_output_dir, &destination)

}



fn move_extracted_files(vpk_output_dir: &String, destination: &str){

    
    // remove unneeded folders (only maps, models, materials, particles, sound)
    let needed_folders = vec!["maps", "models", "materials", "particles", "sound"];
    
    let entries = match fs::read_dir(vpk_output_dir.to_string()) {
        Err(e) => {
           eprintln!("Couldn't read vpk output dir");
           panic!("{}",e)
        }
        Ok(e) => e,
    };

    for entry in entries {
        let entry = match entry {
            Err(e) => {
                println!("couldn't read entry in {}, skipping...",vpk_output_dir.to_string());
                eprintln!("Error: {}",e);
                continue;
            }
            Ok(a) => a,
        };

        let path = &entry.path();
        if !path.is_dir() {
            continue;
        }    

        let folder_name = match path.file_name() {
            None => {
                println!("\x1b[93m[WARNING] couldn't get file name for {:?}, skipping...\x1b[0m",path);
                continue;
            },
            Some(name) => name,
            
        };

        let name = match folder_name.to_str() {
            None => {
                println!("\x1b[93m[WARNING] couldn't turn {:?} to str, skipping...\x1b[0m",folder_name);
                continue;
            },
            Some(name) => name,
        };
        // If you want to work with the directory path
        println!("Folder found: {:?}", path);

        println!("Folder name: {}", name);
        if !needed_folders.contains(&name) {
            continue;
        }

        // check if the file already exists 
        let does_file_already_exist = Path::new(&path).exists();
        if does_file_already_exist {
            handle_folder_that_already_exists(&path, &name);
            continue;
        }
        else {
            // use 'rename' function to move the newly extracted folder to the destination.
            let mut destination_folder = destination.to_owned();
            destination_folder.push_str("/");
                destination_folder.push_str(name);
            println!("moving from {:?} to {destination_folder}",path);
            let res = fs::rename(&path, &destination_folder);
            match res {
                Err(e) => {
                    eprintln!("Failed to move {name} to {destination_folder}");
                    panic!("{}",e);
                }
                Ok(e) => e
            }
        }
    }
                   
    

    // delete remaining folders since we don't need them 
    match fs::remove_dir_all(vpk_output_dir) {
        Err(e) => {
            eprintln!("Failed to clean up the vpk dir in the /tf/ folder, it should be harmless, but you'll have to delete it manually");
            eprintln!("Error: {}",e);

        },
        Ok(e) => ()
    };

}

/**
 * If the folder already exists, check which specific files are missing and only move those into the destination.
 */
fn handle_folder_that_already_exists(path: &PathBuf, folder_name: &str) {
    // TODO
    println!("walking dir {:?}",path);

    let paths = match fs::read_dir(path) {
        Err(e) => {
            eprintln!("Couldn't read parent dir {}",folder_name.to_string());
            panic!("{}",e);
        }
        Ok(paths) => paths,
    };

    
    for file_path in paths {
        let entry = file_path.unwrap();
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();

        if file_type.is_dir() {
            let new_path = path.join(file_name.to_os_string());
            println!(" {:?} is dir, going down unda {:?}", file_name.to_os_string(), new_path);
            handle_folder_that_already_exists(&new_path, &folder_name);
        }
        else {
            println!("I KNOW WHO I AM");
            println!("{:?}",path);
        }
    }

}