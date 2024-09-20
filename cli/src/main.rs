use clap::Parser;
use colored::Colorize;
use diplomat_core::hir;
use std::path::{Path, PathBuf};
mod backend;

pub fn gen(
    entry: &Path,
    out_folder: &Path,
    library_config: Option<&Path>,
    silent: bool,
) -> std::io::Result<()> {
    if !entry.exists() {
        eprintln!(
            "{}{}\n{}",
            "Error: ".red().bold(),
            if entry.file_name().map(|e| e == "lib.rs").unwrap_or_default() {
                "Could not find the lib.rs file to process."
            } else {
                "The entry file does not exist."
            },
            format!("{}", std::env::current_dir().unwrap().join(entry).display()).red()
        );
        std::process::exit(1);
    }

    // The HIR backends used to be named "c2", "js2", etc
    let mut attr_validator = hir::BasicAttributeValidator::new("java");
    attr_validator.support = backend::attr_support();

    let module = syn_inline_mod::parse_and_inline_modules(entry);
    let tcx = hir::TypeContext::from_syn(&module, attr_validator).unwrap_or_else(|e| {
        for (ctx, err) in e {
            eprintln!("Lowering error in {ctx}: {err}");
        }
        std::process::exit(1);
    });

    let (files, errors) = backend::run(&tcx, library_config, out_folder);

    let errors = errors.take_all();
    if !errors.is_empty() {
        eprintln!("Found errors whilst:");
        for error in errors {
            eprintln!("\t{}: {}", error.0, error.1);
        }
        eprintln!("Not generating files due to errors");
        // Eventually this should use eyre or something
        std::process::exit(1);
    }

    if !silent {
        println!("{}", "Generating bindings:".green().bold());
    }
    for (subpath, text) in files.take_files() {
        let out_path = out_folder.join(subpath);
        if !silent {
            println!("{}", format!("  {}", out_path.display()).dimmed());
        }
        std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();
        std::fs::write(&out_path, text)?;
    }

    Ok(())
}

/// diplomat-tool CLI options, as parsed by [clap-derive].
#[derive(Debug, Parser)]
#[clap(
    name = "diplomat-tool",
    about = "Generate bindings to a target language"
)]
struct Opt {
    /// The folder that stores the bindings.
    #[clap(value_parser)]
    out_folder: PathBuf,

    #[clap(short = 'u', long)]
    docs_base_urls: Vec<String>,

    /// The path to the lib.rs file.
    #[clap(short, long, value_parser, default_value = "src/lib.rs")]
    entry: PathBuf,

    /// The path to an optional config file to override code generation defaults.
    /// This is currently used by kotlin and demo_gen.
    #[clap(short, long, value_parser)]
    library_config: Option<PathBuf>,

    #[clap(short = 's', long)]
    silent: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::parse();

    gen(
        &opt.entry,
        &opt.out_folder,
        opt.library_config.as_deref(),
        opt.silent,
    )
}
