use std::{ffi::OsStr, path::PathBuf, process::exit};
use clap::Parser;
use cargo_exe_v2::*;


/// Help text for the subcommand.
///
/// NOTE: This must be given in an attribute macro, rather than as a doc
///     comment, so that Clap does not ruin the rest of the help text to match
///     it being "long".
const ABOUT: &str = "A small tool to print the path to the executable binary \
built by Cargo.

Designed to be run in shell scripts that operate on executables during
development — for example, to build and immediately run in a debugger.";

/// The warning given when the program is run directly.
const STANDALONE_WARN: &str = "{bin} {version}
This program is intended to be invoked as a Cargo subcommand:
        cargo exev2 [...]

For exhausting technical reasons, in order to run it alone, it needs to
be run with the `exev2` subcommand:
        cargo-exe-v2 \x1B[4mexev2\x1B[m [...]";


type Status = i32;

const STATUS_OK: Status = 0;
const STATUS_NO_BIN: Status = 1;
const STATUS_NO_PROJECT: Status = 2;


/// Return a path to a Cargo manifest given input, which may already be a path
///     to a manifest file, or may a path to a directory containing one.
///
/// If no input is given, return a path to a manifest in the current directory.
//  TODO: Follow the filesystem upwards until a manifest is found.
fn find_target(input: Option<impl AsRef<OsStr>>) -> PathBuf {
    let mut path = PathBuf::from(match &input {
        Some(s) => s.as_ref(),
        None => ".".as_ref(),
    });

    if path.is_dir() {
        path.push(FILE_MANIFEST);
    }

    path
}


#[derive(Parser)]
#[clap(
bin_name = "cargo",
disable_help_subcommand(true),
disable_version_flag(true),
help_template(STANDALONE_WARN),
version,
)]
enum Cargo {
    #[clap(about = ABOUT, version)]
    ExeV2 {
        /// Find the most recently modified executable in `target/**`.
        #[clap(long, short)]
        latest: bool,

        /// Look in `target/release/` instead of `target/debug/`.
        #[clap(long, short)]
        release: bool,

        /// A path to a project directory or a `Cargo.toml` file. If this is
        /// not provided, the current directory will be searched.
        #[clap(verbatim_doc_comment)]
        path: Option<String>,

        /// Look for this subpath rather than reading the manifest.
        #[clap(short = 'f', name = "PATH")]
        names_override: Vec<String>,
    }
}


fn main() {
    let Cargo::ExeV2 { latest, release, path, names_override } = Cargo::parse();

    let mode: Mode = if latest {
        Mode::Latest
    } else if release {
        Mode::Release
    } else {
        Mode::Debug
    };

    let path_manifest: PathBuf = find_target(path);

    let filenames = if names_override.is_empty() {
        names_bin(&path_manifest)
    } else {
        Ok(names_override)
    };

    match filenames {
        Err(..) => {
            eprintln!("This does not appear to be a valid Cargo project.");
            exit(STATUS_NO_PROJECT);
        }
        Ok(names) => if names.is_empty() {
            eprintln!("No output executables found.");
            exit(STATUS_NO_BIN);
        } else {
            let mut found = 0;
            let mut path_target: PathBuf = path_project(&path_manifest).into();
            path_target.push(DIR_TARGET);

            for name in names {
                if let Some(path) = mode.make_path(&path_target, name) {
                    println!("{}", path.display());
                    found += 1;
                }
            }

            if found == 0 {
                eprintln!("No output executables found.");
                exit(STATUS_NO_BIN);
            } else {
                exit(STATUS_OK);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target() {
        fn check(input: Option<&str>, expect: &str) {
            let output = find_target(input).display().to_string();

            assert_eq!(
                expect, output,
                "Input {input:?} returns the wrong path.\
                \n  Expected: {expect:?}\
                \n  Received: {output:?}"
            );
        }

        check(None, "./Cargo.toml");
        check(Some("."), "./Cargo.toml");
        check(Some("/"), "/Cargo.toml");
        check(Some("./Cargo.toml"), "./Cargo.toml");
        check(Some("/Cargo.toml"), "/Cargo.toml");
        check(Some("Cargo.toml"), "Cargo.toml");
    }
}
