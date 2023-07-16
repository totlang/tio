use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    /// The .tot file to operate on
    #[arg(global = true)]
    file: String,
    /// Use verbose output
    #[arg(short, long, global = true, default_value_t = false)]
    verbose: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Verify the syntax of a .tot file
    Check,
    /// Output a new file with all tot expressions pre-evaluated
    Bake {
        /// The output format to generate
        #[arg(long = "bake-type", short = 't', value_enum, default_value_t = BakeType::Tot)]
        bake_type: BakeType,
        /// The output path. Defaults to next to the input file, possibly with .baked.tot appended
        #[arg(long, short)]
        output: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum BakeType {
    Tot,
    Json,
    Yaml,
    Toml,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init();

    match args.command {
        Command::Check => {
            log::debug!("Checking {}", args.file);
            tot::parser::parse(std::fs::read_to_string(args.file)?.as_str())?;
        }
        Command::Bake { bake_type, output } => {
            println!(
                "File: {}, Type: {bake_type:?}, Output: {}",
                args.file,
                output.unwrap_or_default()
            );
        }
    }

    Ok(())
}
