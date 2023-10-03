//! Kinda pointless program - wraps cbindgen in a simple CLI.
//! Also wraps `cc` crate for compiling the disc_union binary and linking it with the generated shared lib.

use std::path::PathBuf;

use cbindgen::Config;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GenerateHeader {
        /// Directory for input files - should be a valid cargo project.
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Output file.
        #[arg(short, long, default_value = "library.h")]
        output: PathBuf,
    },
    CompileBinary {
        /// Target id.
        #[arg(short, long, default_value = current_platform::CURRENT_PLATFORM)]
        target: String,

        /// Opt level.
        #[arg(short = 'l', long, default_value_t = 0)]
        opt_level: u32,

        /// Host.
        #[arg(long, default_value = current_platform::CURRENT_PLATFORM)]
        host: String,

        /// Shared object to link
        #[arg(
            short,
            long,
            default_value = "target/debug/libdiscriminated_union_generator.so"
        )]
        object: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::GenerateHeader { dir, output } => {
            let mut config = Config::default();
            config.language = cbindgen::Language::C;
            cbindgen::generate_with_config(dir, config)
                .unwrap()
                .write_to_file(output);
        }
        Command::CompileBinary {
            target,
            opt_level,
            host,
            object,
        } => {
            cc::Build::new()
                .file("disc_union/main.c")
                .include(".")
                .target(&target)
                .opt_level(opt_level)
                .host(&host)
                .object(object)
                .compile("disc_union");
        }
    }
}
