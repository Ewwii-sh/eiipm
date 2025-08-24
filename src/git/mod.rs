//! Minimal fetch & checkout with git2

use git2::{Error, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

/// Initialize a repo at `path` and fetch a specific commit from origin.
///
/// Equivalent to:
/// ```bash
/// git init
/// git fetch --depth 1 origin <commit>
/// git checkout FETCH_HEAD
/// ```
pub fn init_and_fetch(
    repo_url: &str,
    path: &Path,
    commit: &str,
    fetch_depth: i32,
) -> Result<Repository, Error> {
    // initialize new git repository
    let repo = Repository::init(path)?;

    repo.remote("origin", repo_url)?;

    // prepare fetch options (shallow, depth=1)
    let callbacks = RemoteCallbacks::new();
    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);
    fetch_opts.depth(fetch_depth);

    // Fetch the given commit
    {
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[commit], Some(&mut fetch_opts), None)?;
    }

    {
        // Point HEAD to FETCH_HEAD
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let commit = repo.reference_to_annotated_commit(&fetch_head)?;

        repo.set_head_detached(commit.id())?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    }

    Ok(repo)
}

/// Fetch the latest commit from `origin/<commit>`, checkout it,
/// and discard all previous history (like a shallow reset).
pub fn update_to_latest(repo_path: &Path, commit: &str, fetch_depth: i32) -> Result<(), Error> {
    let repo = Repository::open(repo_path)?;

    // Prepare fetch options (shallow)
    let callbacks = RemoteCallbacks::new();
    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);
    fetch_opts.depth(fetch_depth);

    // Fetch from origin
    {
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[commit], Some(&mut fetch_opts), None)?;
    }

    // Point HEAD to FETCH_HEAD
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let commit_obj = repo.find_commit(commit.id())?;

    // Reset hard to that commit
    repo.reset(
        commit_obj.as_object(),
        git2::ResetType::Hard,
        Some(git2::build::CheckoutBuilder::default().force()),
    )?;

    let _ = repo.cleanup_state();

    Ok(())
}
