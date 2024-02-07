use std::{ fs, io::{ Read, Write}, path::Path, sync::{Arc, Mutex}, thread, time::Instant};
use vpk::vpk::VPK;

static NEEDED_FOLDERS: &'static[&str] = &["maps", "models" , "materials", "particles", "sound"];
fn main() {

    let now = Instant::now();
    let vpk_paths = [
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_misc_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_misc_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_vo_english_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_textures_dir.vpk",
    ];

    let number_of_threads = vpk_paths.len();
    let mut handles = Vec::<thread::JoinHandle<()>>::with_capacity(number_of_threads);

    // Create a Mutex to protect access to the VPK instance
    for (i, vpk_path) in vpk_paths.into_iter().enumerate() {
        let path = Path::new(&vpk_path);
        let vpk = VPK::read(&path).unwrap();
        
        let vpk_path_mutex = Arc::new(Mutex::new(vpk_path));
        let vpk_path_mutex_clone = Arc::clone(&vpk_path_mutex);

        let vpk_mutex = Arc::new(Mutex::new(vpk));
        let vpk_mutex_clone = Arc::clone(&vpk_mutex);
        
        let thread_idx_guard = Arc::new(Mutex::new(i));
        let thread_idx = Arc::clone(&thread_idx_guard);
        
        let builder =  thread::Builder::new().name(format!("Thread {}",&i)).spawn(move || {
            
            let idx = thread_idx.lock().unwrap();
            let vpk_path = vpk_path_mutex_clone.lock().unwrap();
            println!("Thread #{} working on {}. Thread ID={:?}",&*idx, &*vpk_path.to_string(), &thread::current().id());

            let mut vpk_map = vpk_mutex_clone.lock().unwrap();


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

                let cloned_file = file;
                
    
                let parent = dest_path.parent().unwrap();
                if Path::exists(parent) == false {
                    println!("[Thread #{}]: creating dir '{}'", &*idx, &parent.to_string_lossy());
                    fs::create_dir_all(&parent).expect("Failed to create directories");
                    
                } 
         

                let mut dest_file = fs::File::create(&dest_path).expect("Could not create file");
                
                let mut bytes = vec![0u8];
                let length = usize::try_from(cloned_file.dir_entry.file_length).unwrap();
                for byte in cloned_file.bytes().into_iter() {
                    if bytes.len() >= length {
                        break;
                    }

                    let b = byte.unwrap();
                    bytes.push(b);
                }

                dest_file.write_all(&bytes).expect("Failed to write");

                let bytes_string = format_byte_size(bytes.len());

                println!("[Thread #{}]: wrote {} bytes into {}", &*idx, &bytes_string, &name);
            }
        });
        handles.push(builder.unwrap())

    }

    for handle in handles {
        let id = handle.thread().id().clone();
        
        match handle.join() {
            Ok(()) => {println!("Thread {:?} is finished", &id)},
            Err(e) => {
                println!("Failed to join Thread {:?}", &id);
            }
        };
    }
    
    let elapsed_time = now.elapsed();

    println!("Finish, time: {}", &elapsed_time.as_secs());   
}




fn format_byte_size(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = 1024 * KB;
    const GB: usize = 1024 * MB;

    if bytes >= GB {
        return format!("{:.2}GB", bytes as f64 / GB as f64)
    }
    if bytes >= MB {
        return format!("{:.2}MB", bytes as f64 / MB as f64)
    } 
    if bytes >= KB {
        return format!("{:.2}KB", bytes as f64 / KB as f64)
    } 
    return bytes.to_string();
}