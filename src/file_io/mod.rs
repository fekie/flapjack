use dirs;
use std::fs;

pub fn init_log_db() {
    let local_data_dir = match dirs::data_local_dir() {
        Some(dir) => dir,
        None => panic!("Could not find local data directory"),
    };
    let flapjack_data_dir = local_data_dir.join("flapjack");
    println!("{:?}", flapjack_data_dir);
    match fs::create_dir(flapjack_data_dir) {
        Ok(_) => println!("Created flapjack data directory"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Could not create flapjack data directory"),
        }, //println!("Could not create flapjack data directory: {}", e),
    }
}
