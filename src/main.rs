#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

fn find_executable_in_path(name: &str) -> Option<PathBuf> {
    let path_os = env::var_os("PATH")?;
    for dir in env::split_paths(&path_os) {
        let candidate = dir.join(name);

        let metadata = match fs::metadata(&c
