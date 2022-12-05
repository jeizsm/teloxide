//! `teloxide-core` uses codegen in order to implement request payloads and
//! `Requester` trait.
//!
//! These are utilities for doing codegen inspired by/stolen from r-a's
//! [sourcegen].
//!
//! [sourcegen]: https://github.com/rust-lang/rust-analyzer/blob/master/crates/sourcegen

// TODO(waffle): does it make sense to extract these utilities (at least project
//               agnostic ones) in a standalone crate?

pub(crate) mod convert;
mod patch;
pub(crate) mod schema;

use std::{
    fs,
    path::{Path, PathBuf},
};

use aho_corasick::AhoCorasick;
use xshell::{cmd, Shell};

fn ensure_rustfmt(sh: &Shell) {
    // FIXME(waffle): find a better way to set toolchain
    let toolchain = "nightly-2022-09-23";

    let version = cmd!(sh, "rustup run {toolchain} rustfmt --version").read().unwrap_or_default();

    if !version.contains("nightly") {
        panic!(
            "Failed to run rustfmt from toolchain '{toolchain}'. Please run `rustup component add \
             rustfmt --toolchain {toolchain}` to install it.",
        );
    }
}

pub fn reformat(text: String) -> String {
    let toolchain = "nightly-2022-09-23";

    let sh = Shell::new().unwrap();
    ensure_rustfmt(&sh);
    let rustfmt_toml = project_root().join("../../rustfmt.toml");
    let mut stdout = cmd!(
        sh,
        "rustup run {toolchain} rustfmt --config-path {rustfmt_toml} --config fn_single_line=true"
    )
    .stdin(text)
    .read()
    .unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

pub fn add_hidden_preamble(generator: &'static str, mut text: String) -> String {
    let preamble = format!("// Generated by `{generator}`, do not edit by hand.\n\n");
    text.insert_str(0, &preamble);
    text
}

pub fn add_preamble(generator: &'static str, mut text: String) -> String {
    let preamble = format!("//! Generated by `{generator}`, do not edit by hand.\n\n");
    text.insert_str(0, &preamble);
    text
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
pub fn ensure_file_contents(file: &Path, contents: &str) {
    ensure_files_contents([(file, contents)])
}

pub fn ensure_files_contents<'a>(
    files_and_contents: impl IntoIterator<Item = (&'a Path, &'a str)>,
) {
    let mut err_count = 0;

    for (path, contents) in files_and_contents {
        let old_contents = fs::read_to_string(path).unwrap();

        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            // File is already up to date.
            continue;
        }

        err_count += 1;

        let display_path = path.strip_prefix(&project_root()).unwrap_or(path);
        eprintln!(
            "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
            display_path.display()
        );
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(path, contents.as_bytes()).unwrap();
    }

    let (s, were) = match err_count {
        // No erros, everything is up to date
        0 => return,
        // Singular
        1 => ("", "was"),
        // Plural
        _ => ("s", "were"),
    };

    if std::env::var("CI").is_ok() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }

    panic!("some file{s} {were} not up to date and has been updated, simply re-run the tests");
}

pub fn replace_block(path: &Path, title: &str, new: &str) -> String {
    let file = fs::read_to_string(path).unwrap();

    let start = format!("// START BLOCK {title}\n");
    let end = format!("// END BLOCK {title}\n");

    let mut starts = vec![];
    let mut ends = vec![];

    let searcher = AhoCorasick::new_auto_configured(&[start, end]);

    for finding in searcher.find_iter(&file) {
        match finding.pattern() {
            // start
            0 => starts.push(finding.start()..finding.end()),
            // end
            1 => ends.push(finding.start()..finding.end()),
            n => panic!("{n}"),
        }
    }

    let start_offset = match &*starts {
        [] => panic!("Coulnd't find start of block {title} in {p}", p = path.display()),
        [offset] => offset.end,
        [..] => panic!(),
    };

    let end_offset = match &*ends {
        [] => panic!("Coulnd't find end of block {title} in {p}", p = path.display()),
        [offset] => offset.start,
        [..] => panic!(),
    };

    if end_offset < start_offset {
        panic!("End of the {title} block is located before the start in {p}", p = path.display());
    }

    format!("{}{}{}", &file[..start_offset], new, &file[end_offset..])
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

/// Changes the first character in a string to uppercase.
pub fn to_uppercase(s: &str) -> String {
    let mut chars = s.chars();
    format!("{}{}", chars.next().unwrap().to_uppercase(), chars.as_str())
}

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let res = PathBuf::from(dir);
    assert!(res.join("CHANGELOG.md").exists());
    res
}

/// Returns minimal prefix of `l` such that `r` doesn't start with the prefix.
#[track_caller]
pub fn min_prefix<'a>(l: &'a str, r: &str) -> &'a str {
    l.char_indices()
        .zip(r.chars())
        .find(|((_, l), r)| l != r)
        .map(|((i, _), _)| &l[..=i])
        .unwrap_or_else(|| panic!("there is no different prefix for {l} and {r}"))
}
