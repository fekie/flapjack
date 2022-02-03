use dirs;
use std::fs;
use std::fs::OpenOptions;

pub fn init_log_db() {
    let local_data_dir = match dirs::data_local_dir() {
        Some(dir) => dir,
        None => panic!("Could not find local data directory"),
    };
    let flapjack_data_dir = local_data_dir.join("flapjack");
    match fs::create_dir(&flapjack_data_dir) {
        Ok(_) => println!(
            "Created flapjack data directory at {}",
            flapjack_data_dir.display()
        ),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Could not create flapjack data directory"),
        },
    }
    let file_path = &flapjack_data_dir.join("log_db.flap");

    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_path);

    match file {
        Ok(_) => println!("Created {}", file_path.display()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Could not create log_db.flap"),
        },
    }
}

pub fn read_raw_db_contents() -> Result<String, std::io::Error> {
    let local_data_dir = match dirs::data_local_dir() {
        Some(dir) => dir,
        None => panic!("Could not find local data directory"),
    };
    let flapjack_data_dir = local_data_dir.join("flapjack");
    let file_path = &flapjack_data_dir.join("log_db.flap");

    let file = fs::read_to_string(file_path);

    match file {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e),
    }
}
