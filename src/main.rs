use anyhow::Result;
use clap::{ArgEnum, Parser};
use erased_serde::{Deserializer, Serializer};
use serde_json;
use serde_transcode;
use serde_yaml;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Format {
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Format to convert from
    #[clap(short, long, arg_enum)]
    source: Format,

    /// Format to convert to
    #[clap(short, long, arg_enum)]
    dest: Format,
}

fn reader(f: Format) -> Box<dyn Deserializer<'static>> {
    match f {
        Format::Json => {
            let json = Box::new(serde_json::Deserializer::from_reader(std::io::stdin()));
            let json = Box::leak(json);
            Box::new(<dyn Deserializer>::erase(json))
        }
        Format::Yaml => {
            let yaml = serde_yaml::Deserializer::from_reader(std::io::stdin());
            Box::new(<dyn Deserializer>::erase(yaml))
        }
    }
}

fn writer(f: Format) -> Box<dyn Serializer> {
    match f {
        Format::Json => {
            let ser = Box::new(serde_json::Serializer::pretty(std::io::stdout()));
            let ser = Box::leak(ser);
            Box::new(<dyn Serializer>::erase(ser))
        }
        Format::Yaml => {
            let ser = Box::new(serde_yaml::Serializer::new(std::io::stdout()));
            let ser = Box::leak(ser);
            Box::new(<dyn Serializer>::erase(ser))
        }
    }
}

fn run(args: Args) -> Result<()> {
    let reader = reader(args.source);
    let writer: &mut dyn Serializer = &mut writer(args.dest);

    serde_transcode::transcode(reader, writer)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
