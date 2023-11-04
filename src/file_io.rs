use std::fs;
use std::fs::OpenOptions;

// Clippy does not like "CouldNot" as a prefix for all error variants.
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum InitLogDbError {
    #[error("Could not find directory for platform. This may be the case if running on a non-standard OS.")]
    CouldNotFindDirectoryForPlatform,
    #[error("Could not create flapjack data directory. ({0})")]
    CouldNotCreateFlapjackDataDirectory(#[source] std::io::Error),
    #[error("Could not create flapjack file (log_db.flap). ({0})")]
    CouldNotCreateFlapjackFile(#[source] std::io::Error),
}

/// Initializes the log database file.
/// It does this by creating a full directory path if it does not exist,
/// and then creating the log database file if it does not exist.
pub fn init_log_db() -> Result<String, InitLogDbError> {
    let local_data_dir = match dirs::data_local_dir() {
        Some(dir) => dir,
        None => return Err(InitLogDbError::CouldNotFindDirectoryForPlatform),
    };

    let flapjack_data_dir = local_data_dir.join("flapjack");

    if let Err(e) = fs::create_dir_all(&flapjack_data_dir) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(InitLogDbError::CouldNotCreateFlapjackDataDirectory(e)),
        }
    }

    let file_path = &flapjack_data_dir.join("log_db.flap");

    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_path);

    let path_string = match file {
        Ok(_) => {
            println!("Created {}", file_path.display());
            file_path.to_string_lossy().to_string()
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => file_path.to_string_lossy().to_string(),
            _ => return Err(InitLogDbError::CouldNotCreateFlapjackFile(e)),
        },
    };

    Ok(path_string)
}
