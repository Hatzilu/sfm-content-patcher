use std::{ collections::HashMap, path::Path, sync::{Arc, Mutex}, thread, time::Instant};
use vpk::vpk::VPK;

use sfm_content_patcher::{get_vpk_paths, handle_vpk_extraction};



fn main() {

    let now = Instant::now();

    let (threaded_paths, main_thread_paths) = get_vpk_paths();

    let thread_count = threaded_paths.len();
    let mut handles = Vec::<thread::JoinHandle<()>>::with_capacity(thread_count);

    // Create a Mutex to protect access to the VPK instance
    for (i, vpk_path) in threaded_paths.into_iter().enumerate() {
        let path = Path::new(&vpk_path);
        let vpk = VPK::read(&path).expect("Failed to read VPK archive file");
        
        let vpk_path_mutex = Arc::new(Mutex::new(vpk_path));
        let vpk_path_mutex_clone = Arc::clone(&vpk_path_mutex);

        let vpk_mutex = Arc::new(Mutex::new(vpk));
        let vpk_mutex_clone = Arc::clone(&vpk_mutex);
        
        
        let builder =  thread::Builder::new().name(format!("{}",&i+1)).spawn(move || {

            handle_vpk_extraction(&vpk_mutex_clone, &vpk_path_mutex_clone, &thread_count);
        });

        handles.push(builder.unwrap());
    }
    for  vpk_path in main_thread_paths.into_iter() {
        if main_thread_paths.len() == 0 {
            break;
        }

        let path = Path::new(&vpk_path);
        let vpk = VPK::read(&path).unwrap();
        
        let vpk_path_mutex = Arc::new(Mutex::new(vpk_path));
        let vpk_path_mutex_clone = Arc::clone(&vpk_path_mutex);

        let vpk_mutex = Arc::new(Mutex::new(vpk));
        let vpk_mutex_clone = Arc::clone(&vpk_mutex);
        

        handle_vpk_extraction(&vpk_mutex_clone, &vpk_path_mutex_clone, &1);
    }

    let mut elapsed_threads = HashMap::<String, u64>::new();

    for handle in handles {       
        let thread_elapsed = now.elapsed().as_secs().clone(); 
        let name = handle.thread().name().map_or("N/A".to_string(), |n| n.to_string());
        handle.join().expect("Failed to join thread");

        elapsed_threads.insert(name, thread_elapsed);
    }
    elapsed_threads.insert("main".to_string(), now.elapsed().as_secs());    

    for (thread_name, elapsed_secs) in elapsed_threads.iter() {
        println!("Thread {} finished in {} seconds", &thread_name, &elapsed_secs)
    }
    // println!("Finished in {} seconds", &elapsed_time.as_secs());   
}