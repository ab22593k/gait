use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::{Component, Path};
use std::process::Command;

use cause::Cause;
use cause::cause;

use super::ErrorType::{self, *};
use super::Parsed;

const DOT_GIT_WIRE: &str = ".gitwire";

pub fn parse_gitwire() -> Result<(String, Vec<Parsed>), Cause<ErrorType>> {
    let (root, file) = get_dotgitwire_file_path()?;
    Ok((root, parse_dotgitwire_file(file)?))
}

fn get_dotgitwire_file_path() -> Result<(String, String), Cause<ErrorType>> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| cause!(RepositoryRootPathCommand).src(e))?
        .stdout;

    let root_str = String::from_utf8(out).map_err(|e| cause!(RepositoryRootPathParse).src(e))?;
    let root = remove_line_ending(root_str);

    let file = format!("{root}/{DOT_GIT_WIRE}");
    if !Path::new(&file).exists() {
        return Err(cause!(
            DotGitWireFileOpen,
            "There is no .gitwire file in this repository"
        ));
    }
    Ok((root, file))
}

fn parse_dotgitwire_file(file: String) -> Result<Vec<Parsed>, Cause<ErrorType>> {
    let f = File::open(&file)
        .map_err(|e| cause!(DotGitWireFileOpen, "no .gitwire file read permission").src(e))?;
    let reader = BufReader::new(f);
    let parsed: Vec<Parsed> = serde_json::from_reader(reader)
        .map_err(|e| cause!(DotGitWireFileParse, ".gitwire file format is wrong").src(e))?;

    for item in &parsed {
        if !check_parsed_item_soundness(item) {
            Err(cause!(
                DotGitWireFileSoundness,
                ".gitwire file's `src` and `dst` must not include '.', '..', and '.git'."
            ))?
        }
    }

    let mut name_set: HashSet<&str> = HashSet::new();
    for p in &parsed {
        if let Some(ref name) = p.name
            && !name_set.insert(name.as_str())
        {
            Err(cause!(
                DotGitWireFileNameNotUnique,
                ".gitwire file's `name`s must be differ each other."
            ))?
        }
    }

    Ok(parsed)
}

fn remove_line_ending(string: String) -> String {
    string
        .strip_suffix("\r\n")
        .or(string.strip_suffix("\n"))
        .unwrap_or(string.as_ref())
        .into()
}

fn check_parsed_item_soundness(parsed: &Parsed) -> bool {
    let is_ok = |e: &Component| -> bool {
        match e {
            Component::Prefix(_) => true,
            Component::RootDir => true,
            Component::Normal(name) => name.ne(&OsStr::new(".git")),
            Component::ParentDir => false,
            Component::CurDir => false,
        }
    };
    let src_result_ok = Path::new(&parsed.src).components().all(|p| is_ok(&p));
    let dst_result_ok = Path::new(&parsed.dst).components().all(|p| is_ok(&p));
    src_result_ok && dst_result_ok
}
