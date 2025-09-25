use std::borrow::Cow;
use std::path::Path;
use std::process::Command;

use cause::Cause;
use cause::cause;
use regex::Regex;
use temp_dir::TempDir;

use super::ErrorType;
use super::ErrorType::*;
use super::Method;
use super::Parsed;

pub fn fetch_target_to_tempdir(prefix: &str, parsed: &Parsed) -> Result<TempDir, Cause<ErrorType>> {
    let tempdir = TempDir::with_prefix(prefix).map_err(|e| cause!(TempDirCreation).src(e))?;

    std::env::set_current_dir(tempdir.path())
        .map_err(|e| cause!(GitCheckoutChangeDirectory).src(e))?;

    git_clone(prefix, tempdir.path(), parsed)?;

    let method = match parsed.mtd.as_ref() {
        Some(Method::Partial) => git_checkout_partial,
        Some(Method::ShallowNoSparse) => git_checkout_shallow_no_sparse,
        Some(Method::Shallow) => git_checkout_shallow_with_sparse,
        None => git_checkout_shallow_with_sparse,
    };

    method(prefix, tempdir.path(), parsed)?;

    Ok(tempdir)
}

fn git_clone(prefix: &str, path: &Path, parsed: &Parsed) -> Result<(), Cause<ErrorType>> {
    println!("  - {prefix}clone --no-checkout: {}", parsed.url);

    let out = Command::new("git")
        .args([
            "-C",
            path.to_str().unwrap(),
            "clone",
            "--depth",
            "1",
            "--filter=blob:none",
            "--no-checkout",
            "--progress",
            parsed.url.as_ref(),
            "./",
        ])
        .output()
        .map_err(|e| cause!(GitCloneCommand).src(e))?;

    handle_git_output(out, "git clone", GitCloneCommandExitStatus)
}

fn git_checkout_partial(
    prefix: &str,
    path: &Path,
    parsed: &Parsed,
) -> Result<(), Cause<ErrorType>> {
    let rev = identify_commit_hash(path, parsed)?;
    let rev = if let Some(r) = rev {
        println!("  - {prefix}checkout partial: {} ({})", r, parsed.rev);
        r
    } else {
        println!("  - {prefix}checkout partial: {}", parsed.rev);
        parsed.rev.to_owned()
    };

    let out = Command::new("git")
        .args([
            "-C",
            path.to_str().unwrap(),
            "checkout",
            "--progress",
            rev.as_ref(),
            "--",
            parsed.src.as_ref(),
        ])
        .output()
        .map_err(|e| cause!(GitCheckoutCommand).src(e))?;

    handle_git_output(out, "git checkout", GitCheckoutCommandExitStatus)
}

fn git_checkout_shallow_no_sparse(
    prefix: &str,
    path: &Path,
    parsed: &Parsed,
) -> Result<(), Cause<ErrorType>> {
    git_checkout_shallow_core(prefix, path, parsed, false)
}

fn git_checkout_shallow_with_sparse(
    prefix: &str,
    path: &Path,
    parsed: &Parsed,
) -> Result<(), Cause<ErrorType>> {
    git_checkout_shallow_core(prefix, path, parsed, true)
}

fn git_checkout_shallow_core(
    prefix: &str,
    path: &Path,
    parsed: &Parsed,
    use_sparse: bool,
) -> Result<(), Cause<ErrorType>> {
    let rev = identify_commit_hash(path, parsed)?;
    let no_sparse = if use_sparse { "" } else { " (no sparse)" };
    let rev = if let Some(r) = rev {
        println!(
            "  - {prefix}checkout shallow{}: {} ({})",
            no_sparse, r, parsed.rev
        );
        r
    } else {
        println!("  - {prefix}checkout shallow{}: {}", no_sparse, parsed.rev);
        parsed.rev.to_owned()
    };

    if use_sparse {
        // Make a kind of absolute path from repository root for sparse checkout.
        let sparse_path: Cow<'_, str> = if parsed.src.starts_with("/") {
            parsed.src.as_str().into()
        } else {
            format!("/{}", &parsed.src).into()
        };

        let out = Command::new("git")
            .args([
                "-C",
                path.to_str().unwrap(),
                "sparse-checkout",
                "set",
                "--no-cone",
                &sparse_path,
            ])
            .output();

        if out.is_err() || !out.unwrap().status.success() {
            // sparse-checkout command is optional, even if it failed,
            // subsequent sequence will be performed without any problem.
            println!("    - {prefix}Could not activate sparse-checkout feature.");
            println!("    - {prefix}Your git client might not support this feature.");
        }
    }

    let out = Command::new("git")
        .args([
            "-C",
            path.to_str().unwrap(),
            "fetch",
            "--depth",
            "1",
            "--progress",
            "origin",
            rev.as_ref(),
        ])
        .output()
        .map_err(|e| cause!(GitFetchCommand).src(e))?;

    if !out.status.success() {
        let error = String::from_utf8(out.stderr)
            .unwrap_or("Could not get even a error output of git checkout command".into());
        return Err(cause!(GitFetchCommandExitStatus, error));
    }

    let out = Command::new("git")
        .args([
            "-C",
            path.to_str().unwrap(),
            "checkout",
            "--progress",
            "FETCH_HEAD",
        ])
        .output()
        .map_err(|e| cause!(GitCheckoutCommand).src(e))?;

    handle_git_output(out, "git checkout", GitCheckoutCommandExitStatus)
}

fn handle_git_output(
    out: std::process::Output,
    command_name: &str,
    error_variant: ErrorType,
) -> Result<(), Cause<ErrorType>> {
    if out.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8(out.stderr).unwrap_or(format!(
            "Could not get even a error output of {command_name} command"
        ));
        Err(cause!(error_variant, error))
    }
}

fn identify_commit_hash(path: &Path, parsed: &Parsed) -> Result<Option<String>, Cause<ErrorType>> {
    let out = Command::new("git")
        .args([
            "-C",
            path.to_str().unwrap(),
            "ls-remote",
            "--heads",
            "--tags",
            parsed.url.as_ref(),
        ])
        .output()
        .map_err(|e| cause!(GitLsRemoteCommand).src(e))?;

    if !out.status.success() {
        let error = String::from_utf8(out.stderr)
            .unwrap_or("Could not get even a error output of git ls-remote command".into());
        return Err(cause!(GitLsRemoteCommandExitStatus).msg(error));
    }

    let stdout =
        String::from_utf8(out.stdout).map_err(|e| cause!(GitLsRemoteCommandStdoutDecode).src(e))?;
    let lines = stdout.lines();

    let re_in_line = Regex::new(&format!(
        "^((?:[0-9a-fA-F]){{40}})\\s+(.*{})(\\^\\{{\\}})?$",
        regex::escape(parsed.rev.as_ref())
    ))
    .map_err(|e| cause!(GitLsRemoteCommandStdoutRegex).src(e))?;

    let matched = lines.filter_map(|l| {
        let cap = re_in_line.captures(l)?;
        let hash = cap.get(1)?.as_str().to_owned();
        let name = cap.get(2)?.as_str().to_owned();

        // Check whether the name is same as `parsed.rev` without doubt,
        // since current regex match method might have some ambiguity.
        // (e.g. if `.` included in 'parsed.rev')
        if !name.contains(&parsed.rev) {
            return None;
        }

        // - A shorter name is supposed that it's more likely exactly matched.
        // - Having '^{}' at the end should be selected if the name is same as another.
        let wrongness = (name.len() * 100) + if cap.get(3).is_some() { 0 } else { 1 };

        Some((hash, name, wrongness))
    });
    let identified = matched.min_by(|l, r| l.2.cmp(&r.2));

    if let Some((rev, _, _)) = identified {
        Ok(Some(rev))
    } else {
        // There is no items among refs/heads and refs/tags.
        // `parsed.rev` must be a commit hash value or at least part of that.
        Ok(None)
    }
}
