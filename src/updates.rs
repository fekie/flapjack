//! Used to update the program when a new version is available on github.

use self_update::cargo_crate_version;

/// An enum of all the ways the updater can fail.
#[derive(thiserror::Error, Debug)]
pub(crate) enum UpdateError {
    #[error("Failed to build update system! ({0})")]
    FailedToBuildUpdateSystem(#[source] self_update::errors::Error),
    #[error("Updater failed! ({0})")]
    UpdaterFailed(#[source] self_update::errors::Error),
}

/// If this function returns an Ok, then the wrapped bool
/// determines whether it was updated or not, and error otherwise.
/// This function will ask the user if they want to update,
/// it will not be completely automatic.
///
/// We allow dead code here because I think rust-analyzer is bugged,
/// this code is definitely being used...
#[allow(dead_code)]
pub(super) fn try_update() -> Result<bool, UpdateError> {
    let update_system_build_result = self_update::backends::github::Update::configure()
        .repo_owner("chloe-woahie")
        .repo_name("flapjack")
        .bin_name("flapjack")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build();

    let update_result = match update_system_build_result {
        Ok(update_system) => update_system.update_extended(),
        Err(e) => {
            return Err(UpdateError::FailedToBuildUpdateSystem(e));
        }
    };

    match update_result {
        Ok(update_status) => match update_status {
            self_update::update::UpdateStatus::UpToDate => Ok(false),
            self_update::update::UpdateStatus::Updated(_) => Ok(true),
        },
        Err(e) => match &e {
            self_update::errors::Error::Update(reason) => {
                if reason == "Update aborted" {
                    Ok(false)
                } else {
                    Err(UpdateError::UpdaterFailed(e))
                }
            }
            _ => Err(UpdateError::UpdaterFailed(e)),
        },
    }
}
