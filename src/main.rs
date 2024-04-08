mod battlebit;
mod filters;
mod toml;

use clap::{command, Parser};

macro_rules! prompt {
    ($message:expr, $function:expr $(, $args:expr)*) => {{
        use std::io::{stdin, stdout, Write};

        loop {
            print!("{} (Y/n): ", $message);
            stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");

            let response = input.trim().to_lowercase();

            match response.as_str() {
                "" | "y" | "yes" => {
                    $function($($args),*)?;
                    break;
                }
                "n" | "no" => {
                    println!("Operation canceled.");
                    break;
                }
                _ => {
                    println!("Invalid input. Please enter 'Y' or 'N'.");
                }
            }
        }
    }};
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help="The filepath of the TOML to import", conflicts_with = "output")]
    input: Option<String>,

    #[clap(short, long, help="The filepath to export the TOML to", conflicts_with = "input")]
    output: Option<String>,
    #[clap(short, long, help="Filters to include during an export", conflicts_with = "input")]
    filters: Option<Vec<String>>,
}

fn process_filters(filters: Vec<String>) -> Vec<String> {
    filters
        .iter()
        .flat_map(|f| {
            if f.contains(',') {
                f.split(',').map(|s| s.to_string()).collect()
            } else {
                vec![f.to_string()]
            }
        })
        .collect()
}

fn input_cmd(mut bbr: battlebit::State, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let input = match args.input {
        Some(file) => file,
        None => {
            return Err("No input file provided".into());
        }
    };

    let toml_str = std::fs::read_to_string(&input)?;
    bbr.from_toml_str(&toml_str)?;
    bbr.save_registry()?;

    tracing::info!("Successfully imported Battlebit configuration from `{}`", input);

    Ok(())
}

fn output_cmd(bbr: battlebit::State, args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let output = match args.output {
        Some(file) => file,
        None => {
            return Err("No output file provided".into());
        }
    };

    let filters = args
        .filters
        .as_ref()
        .map(|f| process_filters(f.clone()))
        .unwrap_or_default();

    let toml = if filters.len() > 0 {
        bbr.to_filtered_toml(filters::parse_filters(filters))
    } else {
        bbr.to_toml()
    };

    std::fs::write(&output, toml)?;

    tracing::info!("Successfully exported Battlebit configuration to `{}`", output);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ansi_term::enable_ansi_support().unwrap();
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let bbr = match battlebit::State::new() {
        Ok(bbr) => bbr,
        Err(e) => {
            tracing::error!("Failed to access Battlebit configuration: {}", e);
            return Ok(());
        }
    };

    if args.input.is_some() {
        prompt!(format!("Are you sure you want to import from `{}`?", args.input.clone().unwrap()), input_cmd, bbr, args);
    } else if args.output.is_some() {
        prompt!(format!("Are you sure you want to export to `{}`?", args.output.clone().unwrap()), output_cmd, bbr, args);
    } else {
        tracing::warn!("No command provided");
    }

    Ok(())
}