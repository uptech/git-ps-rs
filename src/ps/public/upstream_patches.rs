use super::super::private::git;
use super::super::private::paths;
use super::super::private::config;
use ansi_term::Colour::{Yellow, Cyan};

#[derive(Debug)]
pub enum UpstreamPatchesError {
  RepositoryMissing,
  GetRepoRootPathFailed(paths::PathsError),
  PathNotUtf8,
  GetConfigFailed(config::GetConfigError),
  GetHeadRefFailed,
  GetHeadRefTargetFailed,
  GetHeadBranchNameFailed,
  GetUpstreamBranchNameFailed,
  FindUpstreamBranchReferenceFailed(git2::Error),
  GetUpstreamBranchOidFailed
}

pub fn upstream_patches(color: bool) -> Result<(), UpstreamPatchesError> {
  let repo = git::create_cwd_repo().map_err(|_| UpstreamPatchesError::RepositoryMissing)?;

  // get the start & end oids - e.g. origin/main & main
  let head_ref = repo.head().map_err(|_| UpstreamPatchesError::GetHeadRefFailed)?;
  let head_oid = head_ref.target().ok_or(UpstreamPatchesError::GetHeadRefTargetFailed)?;

  let head_branch_name = head_ref.name().ok_or(UpstreamPatchesError::GetHeadBranchNameFailed)?;
  let upstream_branch_name = git::branch_upstream_name(&repo, head_branch_name).map_err(|_| UpstreamPatchesError::GetUpstreamBranchNameFailed)?;

  let upstream_branch_ref = repo.find_reference(&upstream_branch_name).map_err(UpstreamPatchesError::FindUpstreamBranchReferenceFailed)?;
  let upstream_branch_oid = upstream_branch_ref.target().ok_or(UpstreamPatchesError::GetUpstreamBranchOidFailed)?;

  let mut rev_walk = git::get_revs(&repo, head_oid, upstream_branch_oid, git2::Sort::TOPOLOGICAL).unwrap().peekable();
  println!();
  if rev_walk.peek().is_none() { // empty
    println!("None. You are already up to date!");
  }
  rev_walk.for_each(|oid| {
    let sha = oid.unwrap();
    let patch_oid_str = format!("{:.6}", sha);
    let commit = repo.find_commit(sha).unwrap();
    let summary = commit.summary().unwrap_or("").to_string();
    let signature = commit.author();
    let display_name = signature.name().unwrap_or_else(|| signature.email().unwrap_or("Unknown"));

    if color {
      println!("* {:.6} {} {}", Yellow.paint(patch_oid_str), summary, Cyan.paint(display_name));
    } else {
      println!("* {:.6} {} {}", patch_oid_str, summary, signature);
    }
  });

  Ok(())
}
