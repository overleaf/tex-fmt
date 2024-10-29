//! tex-fmt
//! An extremely fast LaTeX formatter written in Rust

#![warn(missing_docs)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]

use clap::Parser;
use std::fs;
use std::process::ExitCode;

mod cli;
mod comments;
mod format;
mod ignore;
mod indent;
mod logging;
mod read;
mod regexes;
mod subs;
mod verbatim;
mod wrap;
mod write;
use crate::cli::*;
use crate::format::*;
use crate::logging::*;
use crate::read::*;
use crate::write::*;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

/// Line ending for unix
const LINE_END: &str = "\n";

#[wasm_bindgen]
pub fn main() {
    let mut args = Cli::parse();
    init_logger(args.log_level());

    let mut logs = Vec::<Log>::new();
    let mut exit_code = args.resolve(&mut logs);

    if exit_code == 0 {
        if args.stdin {
            if let Some((file, text)) = read_stdin(&mut logs) {
                let new_text = format_file(&text, &file);
                exit_code = process_output(
                    &args, &file, &text, &new_text, exit_code, &mut logs,
                );
            } else {
                exit_code = 1;
            }
        } else {
            for file in &args.files {
                if let Some((file, text)) = read(file, &mut logs) {
                    let new_text = format_file(&text, &file);
                    exit_code = process_output(
                        &args, &file, &text, &new_text, exit_code, &mut logs,
                    );
                } else {
                    exit_code = 1;
                };
            }
        }
    }

    print_logs(&mut logs);
//     ExitCode::from(exit_code)
}
