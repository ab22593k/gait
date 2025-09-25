use std::path::Path;
use std::sync::Arc;

use cause::Cause;
use cause::cause;
use folder_compare::FolderCompare;
use temp_dir::TempDir;

use crate::common;
use crate::common::ErrorType;
use crate::common::ErrorType::*;
use crate::common::Parsed;
use crate::common::Target;
use crate::common::sequence::Operation;

#[derive(Debug)]
struct CheckOperation {}

impl Operation for CheckOperation {
    fn operate(
        &self,
        prefix: &str,
        parsed: &Parsed,
        rootdir: &str,
        tempdir: &TempDir,
    ) -> Result<bool, Cause<ErrorType>> {
        compare_with_temp(prefix, parsed, rootdir, tempdir.path())
    }
}

pub fn check(target: Target, mode: common::sequence::Mode) -> Result<bool, Cause<ErrorType>> {
    println!("git-wire check started\n");
    let operation = Arc::new(CheckOperation {});
    let result = common::sequence::sequence(target, operation, mode)?;
    Ok(result)
}

fn compare_with_temp(
    prefix: &str,
    parsed: &Parsed,
    root: &str,
    temp: &Path,
) -> Result<bool, Cause<ErrorType>> {
    println!("  - {prefix}compare `src` and `dst`");

    let temp_root = temp;
    let temp = temp.join(parsed.src.as_str());
    let root = Path::new(root).join(parsed.dst.as_str());

    let fc1 =
        FolderCompare::new(&temp, &root, &vec![]).map_err(|_| cause!(CheckDifferenceExecution))?;
    let fc2 =
        FolderCompare::new(&root, &temp, &vec![]).map_err(|_| cause!(CheckDifferenceExecution))?;

    let mut result = true;

    use colored::*;

    if !fc1.new_files.is_empty() {
        let temp_root = temp_root
            .to_str()
            .ok_or_else(|| cause!(CheckDifferenceStringReplace))?;
        for file in fc1.new_files {
            let file = file
                .to_str()
                .ok_or_else(|| cause!(CheckDifferenceStringReplace))?;
            let file = file.replace(temp_root, "");
            println!(
                "{}",
                format!("    {prefix}! file {file:?} does not exist").red()
            );
        }
        result = false;
    }
    if !fc2.new_files.is_empty() {
        for file in fc2.new_files {
            println!(
                "{}",
                format!("    {prefix}! file {file:?} does not exist on original").red()
            );
        }
        result = false;
    }
    if !fc2.changed_files.is_empty() {
        for file in fc2.changed_files {
            println!(
                "{}",
                format!("    {prefix}! file {file:?} is not identical to original").red()
            );
        }
        result = false;
    }

    Ok(result)
}
