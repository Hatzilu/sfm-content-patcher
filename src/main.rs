use std::{path::Path, sync::{Arc, Mutex}, thread};

use vpk::vpk::VPK;


fn main(){

    let vpk_paths = [
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_misc_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_misc_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_vo_english_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_textures_dir.vpk",
    ];

       // Create a Mutex to protect access to the VPK instance
       for (i, vpk_path) in vpk_paths.into_iter().enumerate() {
           let path = Path::new(&vpk_path);
           let vpk = VPK::read(&path).unwrap();
           let vpk_mutex: Arc<Mutex<VPK>> = Arc::new(Mutex::new(vpk));

         thread::spawn(move || {
            println!("Thread {} working on {}",&i + 1, &vpk_path);
            // let path = Path::new(&vpk_path);
            // let result = VPK::read(&path).unwrap();
            let vpk_map = vpk_mutex.lock().unwrap();
            for (name, file) in vpk_map.tree.iter() {
                
                println!(" Thread #{}: reading file {}",&i,&name)
            }
        });

    }
    
}