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
    let branch_name = head_ref
        .shorthand()
        .ok_or_else(|| Error::from_str("Invalid branch"))?;

    // Fetch remote branch
    let callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&[branch_name], Some(&mut fetch_options), None)?;

    // Find fetched commit
    let fetch_ref = repo.find_reference(&format!("refs/remotes/origin/{}", branch_name))?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_ref)?;

    // Merge analysis
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.0.is_fast_forward() {
        // Fast-forward
        let mut ref_to_update = repo.find_reference(head_ref.name().unwrap())?;
        ref_to_update.set_target(fetch_commit.id(), "Fast-forward")?;
        repo.set_head(head_ref.name().unwrap())?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        log::info!("Fast-forward merge completed");
    } else {
        // Real merge
        log::info!("Fast-forward not possible, performing merge...");

        let head_commit = repo.reference_to_annotated_commit(&head_ref)?;
        let head_tree = repo.find_commit(head_commit.id())?.tree()?;
        let fetch_tree = repo.find_commit(fetch_commit.id())?.tree()?;

        let ancestor_commit = repo.merge_base(head_commit.id(), fetch_commit.id())
            .and_then(|oid| repo.find_commit(oid))?;
        let ancestor_tree = ancestor_commit.tree()?;


        let mut idx = repo.merge_trees(&ancestor_tree, &head_tree, &fetch_tree, None)?;
        if idx.has_conflicts() {
            return Err(Error::from_str("Merge conflicts detected. Please resolve manually."));
        }

        // Write the merged tree
        let result_tree_id = idx.write_tree_to(repo)?;
        let result_tree = repo.find_tree(result_tree_id)?;

        // Create merge commit
        let sig = repo.signature()?;
        let head_commit_obj = repo.find_commit(head_commit.id())?;
        let fetch_commit_obj = repo.find_commit(fetch_commit.id())?;

        repo.commit(
            Some(head_ref.name().unwrap()), // update current branch
            &sig,
            &sig,
            "Merge commit from pull",
            &result_tree,
            &[&head_commit_obj, &fetch_commit_obj],
        )?;

        // Checkout updated HEAD
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        log::info!("Merge completed successfully");
    }

    Ok(())
}

pub fn pull_with_rebase_or_reclone(
    repo_url: &str,
    repo_path: &Path,
    depth: Option<u32>,
) -> Result<Repository, Error> {
    // Try opening the repo if it exists
    if let Ok(repo) = Repository::open(repo_path) {
        // First attempt regular pull
        match pull_https(&repo) {
            Ok(_) => return Ok(repo),
            Err(pull_err) => {
                log::warn!("Pull failed: {}", pull_err);

                // Ask user if they want to try a rebase instead
                if confirm("Pull failed. Attempt rebase on top of remote?") {
                    match rebase_local_on_remote(&repo) {
                        Ok(_) => return Ok(repo),
                        Err(rebase_err) => {
                            log::warn!("Rebase failed: {}", rebase_err);
                        }
                    }
                }

                // If rebase was refused or failed, ask to reclone
                if confirm("Rebase failed or was skipped. Delete and reclone?") {
                    let home_dir = dirs::home_dir()
                        .ok_or_else(|| Error::from_str("Failed to get home directory"))?;
                    let cache_root = home_dir.join(".eiipm/cache");

                    if !repo_path.starts_with(cache_root.as_path()) {
                        return Err(Error::from_str(&format!(
                            "Refusing to delete outside cache: {}",
                            repo_path.display()
                        )));
                    }

                    fs::remove_dir_all(repo_path)
                        .map_err(|e| Error::from_str(&format!("Failed to remove dir: {}", e)))?;
                } else {
                    // User refused reclone, return the repo as-is
                    return Ok(repo);
                }
            }
        }
    }

    // Either repo didn't exist or was removed, so clone fresh
    clone_https(repo_url, repo_path, depth)
}

pub fn rebase_local_on_remote(repo: &Repository) -> Result<(), Error> {
    let head_ref = repo.head()?;

    let branch_name = head_ref
        .shorthand()
        .ok_or_else(|| Error::from_str("Invalid branch"))?;

    // Find fetched commit
    let fetch_ref = repo.find_reference(&format!("refs/remotes/origin/{}", branch_name))?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_ref)?;

    // Attempt rebase
    log::info!("Fast-forward not possible, trying rebase...");
    let head_commit = repo.reference_to_annotated_commit(&head_ref)?;
    let mut rebase = repo.rebase(
        Some(&head_commit),
        Some(&fetch_commit),
        None,
        None,
    )?;

    loop {
        match rebase.next() {
            Some(res) => {
                res.map_err(|e| Error::from_str(&format!("Rebase operation failed: {}", e)))?;
                rebase
                    .commit(None, &repo.signature()?, None)
                    .map_err(|e| Error::from_str(&format!("Commit during rebase failed: {}", e)))?;
            }
            None => break,
        }
    }

    match rebase.finish(None) {
        Ok(_) => {
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            log::info!("Rebase completed successfully");
        }
        Err(e) => {
            // Conflict occurred
            return Err(Error::from_str(&format!(
                "Rebase failed due to conflict: {}",
                e
            )));
        }
    }

    Ok(())
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