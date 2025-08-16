/// Uses with git2

use git2::{
    Repository, Cred, RemoteCallbacks, 
    FetchOptions, build::RepoBuilder, Error
};
use std::path::Path;
use std::fs;
use crate::other::confirm_action::confirm;

pub fn clone_https(repo_url: &str, path: &Path, depth: Option<u32>) -> Result<Repository, Error> {
    let callbacks = RemoteCallbacks::new();

    // Set up fetch options
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // Apply shallow clone if depth is specified
    if let Some(d) = depth {
        fetch_options.depth(d as i32);
    }

    // Use RepoBuilder directly
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let repo = builder.clone(repo_url, path)?;
    Ok(repo)
}

pub fn pull_https(repo: &Repository) -> Result<(), Error> {
    let head_ref = repo.head()?;
    let branch_name = head_ref.shorthand().ok_or_else(|| Error::from_str("Invalid branch"))?;

    // No credentials needed
    let callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[branch_name], Some(&mut fetch_options), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        let mut ref_to_update = repo.find_reference(head_ref.name().unwrap())?;
        ref_to_update.set_target(fetch_commit.id(), "Fast-forward")?;
        repo.set_head(head_ref.name().unwrap())?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        log::info!("Fast-forward merge completed");
    } else {
        return Err(Error::from_str("No fast-forward merge, manual merge required"));
    }

    Ok(())
}


pub fn pull_but_reclone_on_fail(repo_url: &str, repo_path: &Path, depth: Option<u32>) -> Result<Repository, Error> {
    // Try opening the repo if it exists
    if let Ok(repo) = Repository::open(repo_path) {
        // Try to pull
        match pull_https(&repo) {
            Ok(_) => return Ok(repo),
            Err(err) => {
                log::warn!("Pull failed (maybe not fast-forward): {}.", err);

                let user_confirm = confirm("Forwarding latest commit to cache failed. Delete cache and retry?");

                let home_dir = dirs::home_dir().ok_or_else(|| Error::from_str("Failed to get home directory"))?;
                let cache_root = home_dir.join(".eiipm/cache");

                if user_confirm {
                    if !repo_path.starts_with(cache_root.as_path()) {
                        return Err(Error::from_str(&format!("Refusing to delete outside cache: {}", repo_path.display())));
                    }

                    fs::remove_dir_all(repo_path).map_err(|e| Error::from_str(&format!("Failed to remove dir: {}", e)))?;
                } else {
                    // user refused, so just return the repo as-is
                    return Ok(repo);
                }
            }
        }
    }

    // Either repo didn't exist or we removed it, so clone fresh
    clone_https(repo_url, repo_path, depth)
}

/// Checks if the current branch is behind its upstream.
/// Returns `Ok(true)` if the upstream has commits the local branch doesn't have.
pub fn is_upstream_ahead(repo_path: &str) -> Result<bool, Error> {
    let repo = Repository::open(repo_path)?;
    
    // Get the current branch
    let head_ref = repo.head()?;
    let branch_name = head_ref.shorthand()
        .ok_or_else(|| Error::from_str("Invalid branch name"))?;
    
    // Set up fetch options with authentication callbacks
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    // Fetch from origin
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[branch_name], Some(&mut fetch_options), None)?;
    
    // Resolve upstream
    let local_branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let upstream_branch = local_branch.upstream()?;
    
    let local_oid = local_branch.get().target().ok_or_else(|| Error::from_str("Local branch has no commit"))?;
    let upstream_oid = upstream_branch.get().target().ok_or_else(|| Error::from_str("Upstream branch has no commit"))?;
    
    let (_ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid)?;
    
    Ok(behind > 0)
}