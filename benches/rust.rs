// $ cargo bench --features full --bench rust
//
// Syn only, useful for profiling:
// $ RUSTFLAGS='--cfg syn_only' cargo build --release --features full --bench rust

#![cfg_attr(not(syn_only), feature(rustc_private))]

#[macro_use]
#[path = "../tests/macros/mod.rs"]
mod macros;

#[path = "../tests/repo/mod.rs"]
mod repo;

use std::fs;
use std::time::{Duration, Instant};

#[cfg(not(syn_only))]
mod tokenstream_parse {
    use proc_macro2::TokenStream;
    use std::str::FromStr;

    pub fn bench(content: &str) -> Result<(), ()> {
        TokenStream::from_str(content).map(drop).map_err(drop)
    }
}

mod syn_parse {
    pub fn bench(content: &str) -> Result<(), ()> {
        syn::parse_file(content).map(drop).map_err(drop)
    }
}

#[cfg(not(syn_only))]
mod libsyntax_parse {
    extern crate rustc_data_structures;
    extern crate rustc_parse;
    extern crate rustc_span;
    extern crate syntax;

    use rustc_data_structures::sync::Lrc;
    use rustc_span::FileName;
    use syntax::edition::Edition;
    use syntax::errors::{emitter::Emitter, Diagnostic, Handler};
    use syntax::sess::ParseSess;
    use syntax::source_map::{FilePathMapping, SourceMap};

    pub fn bench(content: &str) -> Result<(), ()> {
        struct SilentEmitter;

        impl Emitter for SilentEmitter {
            fn emit_diagnostic(&mut self, _diag: &Diagnostic) {}
            fn source_map(&self) -> Option<&Lrc<SourceMap>> {
                None
            }
        }

        syntax::with_globals(Edition::Edition2018, || {
            let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
            let emitter = Box::new(SilentEmitter);
            let handler = Handler::with_emitter(false, None, emitter);
            let sess = ParseSess::with_span_handler(handler, cm);
            if let Err(mut diagnostic) = rustc_parse::parse_crate_from_source_str(
                FileName::Custom("bench".to_owned()),
                content.to_owned(),
                &sess,
            ) {
                diagnostic.cancel();
                return Err(());
            };
            Ok(())
        })
    }
}

#[cfg(not(syn_only))]
mod read_from_disk {
    pub fn bench(content: &str) -> Result<(), ()> {
        let _ = content;
        Ok(())
    }
}

fn exec(mut codepath: impl FnMut(&str) -> Result<(), ()>) -> Duration {
    let begin = Instant::now();
    let mut success = 0;
    let mut total = 0;

    walkdir::WalkDir::new("tests/rust/src")
        .into_iter()
        .filter_entry(repo::base_dir_filter)
        .for_each(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                return;
            }
            let content = fs::read_to_string(path).unwrap();
            let ok = codepath(&content).is_ok();
            success += ok as usize;
            total += 1;
            if !ok {
                eprintln!("FAIL {}", path.display());
            }
        });

    assert_eq!(success, total);
    begin.elapsed()
}

fn main() {
    repo::clone_rust();

    macro_rules! testcases {
        ($($(#[$cfg:meta])* $name:path,)*) => {
            vec![
                $(
                    $(#[$cfg])*
                    (stringify!($name), $name as fn(&str) -> Result<(), ()>),
                )*
            ]
        };
    }

    #[cfg(not(syn_only))]
    {
        let mut lines = 0;
        let mut files = 0;
        exec(|content| {
            lines += content.lines().count();
            files += 1;
            Ok(())
        });
        eprintln!("\n{} lines in {} files", lines, files);
    }

    for (name, f) in testcases!(
        #[cfg(not(syn_only))]
        read_from_disk::bench,
        #[cfg(not(syn_only))]
        tokenstream_parse::bench,
        syn_parse::bench,
        #[cfg(not(syn_only))]
        libsyntax_parse::bench,
    ) {
        eprint!("{:20}", format!("{}:", name));
        let elapsed = exec(f);
        eprintln!(
            "elapsed={}.{:03}s",
            elapsed.as_secs(),
            elapsed.subsec_millis(),
        );
    }
    eprintln!();
}
