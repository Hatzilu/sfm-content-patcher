use std::{ fs, io::{ErrorKind, Read, Write}, path::Path, sync::{Arc, Mutex}, thread, time::Instant};
use vpk::vpk::VPK;

static NEEDED_FOLDERS: &'static[&str] = &["maps", "models" , "materials", "particles", "sound"];
fn main(){

    let now = Instant::now();
    let vpk_paths = [
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_misc_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_misc_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_sound_vo_english_dir.vpk",
        "C:/Program Files (x86)/Steam/steamapps/common/Team Fortress 2/tf/tf2_textures_dir.vpk",
    ];

    let mut handles = Vec::<thread::JoinHandle<()>>::with_capacity(4);
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

         let handle =  thread::spawn(move || {
            
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
                    println!("[Thread {}]: creating dir '{}'", &*idx, &parent.to_string_lossy());
                    fs::create_dir_all(&parent).expect("Failed to create directories");
                    
                } 
         

                let mut dest_file = fs::File::create(&dest_path).expect("Could not create file");
                
                //  TODO: read and write to dest somehow

                // let mut buf = [0u8; 8192 ];
                // let mut bufs = Vec::new();
                // loop {
                //     match cloned_file.read(&mut buf) {
                //         Ok(0) => break, // End of file
                //         Ok(n) => {
                //             // bufs.push(&buf[..n]);
                //             let bytes = &buf[..n];
                //             println!("bytes {:?}",bytes.len());
                //             dest_file.write(bytes).expect("Failed to write to file");
                //         }
                //         Err(ref e) if e.kind() == ErrorKind::Interrupted => continue, // Retry on interrupted system calls
                //         Err(e) => panic!("Error reading file: {}", e),
                //     }
                //     println!("[Thread {}]: buffer size: {}", &*idx, &buf.len());
                //     // }
                //     // dest_file.write_all(&buf).expect("Failed to write to file");
                // }


                println!("[Thread #{}]: Wrote file {}", &*idx, &name);
            }
        });
        handles.push(handle)

    }

    for handle in handles {
        let id = handle.thread().id().clone();
        
        match handle.join() {
            Ok(()) => {println!("Thread {:?} is finished", &id)},
            Err(e) => {
                eprintln!("Failed to join Thread {:?}, err: {:?}", &id, &e)
            }
        };
    }
    
    let elapsed_time = now.elapsed();

    println!("Finish, time: {}", &elapsed_time.as_secs());
    
}