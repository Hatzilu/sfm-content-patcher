
use std::{fs::{self, OpenOptions}, io::{BufWriter, LineWriter, Write}, path::Path, sync::{Arc, Mutex}, thread};

use vpk::VPK;

static NEEDED_FOLDERS: &'static[&str] = &["maps", "models" , "materials", "particles", "sound"];

static VPK_PATHS: &'static[&str] = &[
    "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_misc_dir.vpk",
    "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_misc_dir.vpk",
    "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_vo_english_dir.vpk",
    "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_textures_dir.vpk",
];


/**
 * Return the vpk files to process based on the current machine's available threads
 * if a machine has 4 threads, corresponding to all 4 vpk files to be processed, it will return 3 threaded paths and 1 main_thread_path
 * the paths in threaded_paths will be run on threads, while the paths in main_thread_path will be processed synchronously on the main thread
 */
pub fn get_vpk_paths() -> (&'static[&'static str], &'static[&'static str]) {
    
    let available_threads = thread::available_parallelism().expect("Failed to get thread count");

    let mut split_index = usize::try_from(available_threads).expect("Failed to get thread split index.");
    if split_index >= VPK_PATHS.len() {
        split_index = VPK_PATHS.len()
    }
    
    let  (threaded_paths, main_thread_paths) = VPK_PATHS.split_at(split_index - 1);    

    return (threaded_paths, main_thread_paths);
}

pub fn handle_vpk_extraction(vpk_mutex_clone: &Arc<Mutex<VPK>>, vpk_path_mutex_clone: &Arc<Mutex<&&str>>, thread_count: &usize) {
    let current_thread = thread::current();
    let thread_name = current_thread.name().expect("Failed to resolve thread name");
    let vpk_path = vpk_path_mutex_clone.lock().expect("Failed to lock vpk path str");
    
    println!("Thread #{}/{} working on {}. Thread ID={:?}",&thread_name,&thread_count, &*vpk_path.to_string(), &thread::current().id());

    let mut vpk_map = vpk_mutex_clone.lock().expect("Failed to lock VPK variable");


    for (name, file) in vpk_map.tree.iter_mut() {
        let mut should_skip_iteration = true;
        
        for folder_prefix in NEEDED_FOLDERS.iter() {
            if name.starts_with(folder_prefix) {
                should_skip_iteration = false;
                break;
            }
        }
        
        if should_skip_iteration {
            continue;
        }

        let dest_path_string = format!("C:/Users/the linux drive/Desktop/rust_sfm_test/{}", &name);
        let dest_path = Path::new(&dest_path_string);

        if Path::exists(&dest_path) {            

            continue;
        }

        let bytes = file.get().expect("Failed to get file bytes from VPK entry.");
        
        let parent = dest_path.parent().expect("Failed to get parent directory of VPK entry");

        if Path::exists(parent) == false {
            fs::create_dir_all(&parent).expect("Failed to create directories");
        } 
    

        let mut dest_file = fs::File::create(&dest_path).expect("Could not create file");
        
        dest_file.write_all(&bytes).expect("Failed to write buffer to file");
        println!("[Thread #{}/{}]: wrote {} bytes into {}", &thread_name,&thread_count, &bytes.len(), &name);
    }
}