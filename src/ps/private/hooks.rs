use std::path::{Path, PathBuf};
use super::paths::{PathExistsAndIsExecutable, path_exists_and_is_executable};
use home_dir::{self, HomeDirExt};

#[derive(Debug)]
pub enum FindHookError {
  NotExecutable(PathBuf),
  PathExpandHomeFailed(home_dir::Error),
  NotFound
}

pub fn find_hook(repo_root: &str, filename: &str) -> Result<PathBuf, FindHookError> {
  let communal_repository_level_hook_path_string = format!("{}/.git-ps/hooks/{}", repo_root, filename);
  let communal_repository_level_hook_path = Path::new(communal_repository_level_hook_path_string.as_str());
  let repository_level_hook_path_string = format!("{}/.git/git-ps/hooks/{}", repo_root, filename);
  let repository_level_hook_path = Path::new(repository_level_hook_path_string.as_str());
  let user_level_hook_path_string = format!("~/.config/git-ps/hooks/{}", filename);
  let user_level_hook_path = Path::new(user_level_hook_path_string.as_str()).expand_home().map_err(FindHookError::PathExpandHomeFailed)?;
  match path_exists_and_is_executable(communal_repository_level_hook_path) {
    PathExistsAndIsExecutable::ExistsAndIsExecutable => Ok(communal_repository_level_hook_path.to_path_buf()),
    PathExistsAndIsExecutable::ExistsButNotExecutable => Err(FindHookError::NotExecutable(communal_repository_level_hook_path.to_path_buf())),
    PathExistsAndIsExecutable::DoesNotExist => match path_exists_and_is_executable(repository_level_hook_path) {
      PathExistsAndIsExecutable::ExistsAndIsExecutable => Ok(repository_level_hook_path.to_path_buf()),
      PathExistsAndIsExecutable::ExistsButNotExecutable => Err(FindHookError::NotExecutable(repository_level_hook_path.to_path_buf())),
      PathExistsAndIsExecutable::DoesNotExist => match path_exists_and_is_executable(&user_level_hook_path) {
        PathExistsAndIsExecutable::ExistsAndIsExecutable => Ok(user_level_hook_path.to_path_buf()),
        PathExistsAndIsExecutable::ExistsButNotExecutable => Err(FindHookError::NotExecutable(user_level_hook_path.to_path_buf())),
        PathExistsAndIsExecutable::DoesNotExist => Err(FindHookError::NotFound)
      }
    }
  }
}
