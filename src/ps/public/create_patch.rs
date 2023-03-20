use super::super::private::utils;
use std::result::Result;

#[derive(Debug)]
pub enum CreatePatchError {
    CommitFailed(utils::ExecuteError),
}

pub fn create_patch() -> Result<(), CreatePatchError> {
    utils::execute("git", &["commit", "-v"]).map_err(CreatePatchError::CommitFailed)?;
    Ok(())
}
