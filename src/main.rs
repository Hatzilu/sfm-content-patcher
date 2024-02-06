use std::{ fs,  io::{Read, Write}, path::Path, sync::{Arc, Mutex}, thread::{self, Thread}, time::Duration};
use vpk::vpk::VPK;

// #[derive(Clone)]
// struct MyVPKEntry(VPKEntry);

// Implement Clone for MyVPKEntry
// impl Clone for MyVPKEntry {
//     fn clone(&self) -> Self {
//         // Clone the inner VPKEntry
//         Self(self.clone())
//     }
// }

fn main(){

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
                
                let cloned_file = file;
                // let mut buf = Vec::new();
                
                let mut dest_path_string = "C:/Users/the linux drive/Desktop/rust_sfm_test/".to_string();
                dest_path_string.push_str(&name);
                let dest_path = Path::new(&dest_path_string);
                
                let parent = dest_path.parent().unwrap();
                if Path::exists(parent) == false {
                    println!("creating dir '{}'", &parent.to_string_lossy());
                    match fs::create_dir_all(&parent) {
                        Err(e) => {eprintln!("Failed to create directories, error: {:?}", e);},
                        Ok(()) => {println!("Successfully Created dir {}", &parent.to_string_lossy())},
                    }
                    thread::sleep(Duration::from_millis(1000));
                    
                } else {
                    println!("path exists {}",&parent.to_string_lossy());
                }
                
                let mut opts = fs::OpenOptions::new();
                let mut dest = opts.write(true).open(&dest_path).expect("Failed to write file");

                for byte in cloned_file.bytes().into_iter() {
                    let b = byte.unwrap();
                    dest.write(&[b]).unwrap();
                    // buf.push(&b);
                }

                // dest.write_all(&bufas_chunks).unwrap();
                // cloned_file.read(buf)
                // match cloned_file.read_to_end(&mut buf) {
                //     Err(e) => {
                //         println!("Failed to read file {}", e);
                //         panic!("{}",e);
                //     },
                //     Ok(size) => println!("name {}, usize: {}", &name, &size),
                // };

    

                

                // fs::write(&dest_path,&bytes);
                // dest.write_all(&buf).unwrap();

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

    println!("Finish")
    
}