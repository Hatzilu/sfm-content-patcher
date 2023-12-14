use std::process::Command;


fn main() {

    let tf2_dir = "E:/SteamLibrary/steamapps/common/Team Fortress 2".to_owned();
    let sfm_dir = "E:/SteamLibrary/steamapps/common/SourceFilmmaker/game/tf";
    // TODO: remove and write directly to SFM folder

    // TODO: auto-detect the tf2 and sfm directories. or ask user for input if auto-detection fails
    println!("Your TF2 directory sits at {}", tf2_dir);
    println!("Your SFM directory sits at {}", sfm_dir);

    let mut vpk_path = tf2_dir.clone().to_owned();
    vpk_path.push_str("/tf/tf2_misc_dir.vpk");

    extract_vpk_file(&vpk_path,&tf2_dir,destination);

    println!("Hello, world!");
}


/**
 * get all file names to be extracted from the vpk file.
 */
fn get_file_names(path: &str, tf2_game_path: &str) -> Vec<String> {
    
    // store all file names from the vpk in an array
    let mut file_names = Vec::new();
    let mut tf2_dir = tf2_game_path.clone().to_owned();

    tf2_dir.push_str("/bin/vpk.exe");

    let mut vpk_cmd = Command::new(tf2_dir);
    vpk_cmd.arg("l").arg(path);

    let output = String::from_utf8(vpk_cmd.output().unwrap().stdout).unwrap().to_string();

    for raw_file_name in output.lines() {
        if raw_file_name.starts_with("maps") || raw_file_name.starts_with("models") || raw_file_name.starts_with("materials") || raw_file_name.starts_with("particles")  || raw_file_name.starts_with("sound") {
            file_names.push(raw_file_name.to_string());
        }
    }

    return file_names;
}

fn extract_vpk_file(vpk_file_path: &str,tf2_game_path: &str, destination: &str) {
    let file_names = get_file_names(&vpk_file_path, &tf2_game_path);
    println!("{}",file_names.last().unwrap().to_string());

    let tf2_dir = tf2_game_path.clone().to_owned();

    // let mut vpk_cmd = Command::new(tf2_dir);
    // vpk_cmd.arg("x").arg(vpk_file_path);

    // let output = String::from_utf8(vpk_cmd.output().unwrap().stdout).unwrap().to_string();

    // println!("Output: {}",output);
}

