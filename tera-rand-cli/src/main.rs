mod error;

use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use crate::error::TeraRandCliError;
use clap::{Parser, Subcommand};
use iso8601::Duration;
use tera::{Context, Tera};
use tera_rand::{
    random_bool, random_char, random_float32, random_float64, random_from_file, random_int32,
    random_int64, random_ipv4, random_ipv4_cidr, random_ipv6, random_ipv6_cidr, random_string,
    random_uint32, random_uint64, random_uuid,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// filepath of the Tera template to render
    #[arg(short, long)]
    file: PathBuf,
    /// number of rendered templates to output per `interval`. This is optional, but
    #[arg(short, long)]
    count: Option<u32>,
    #[arg(short, long)]
    interval: Option<Duration>,
}

fn main() {
    println!("Hello, world!");
    let cli: Cli = Cli::parse();
    let mut tera: Tera = Tera::default();

    tera.register_function("random_bool", random_bool);
    tera.register_function("random_char", random_char);
    tera.register_function("random_float32", random_float32);
    tera.register_function("random_float64", random_float64);
    tera.register_function("random_from_file", random_from_file);
    tera.register_function("random_int32", random_int32);
    tera.register_function("random_int64", random_int64);
    tera.register_function("random_ipv4", random_ipv4);
    tera.register_function("random_ipv4_cidr", random_ipv4_cidr);
    tera.register_function("random_ipv6", random_ipv6);
    tera.register_function("random_ipv6_cidr", random_ipv6_cidr);
    tera.register_function("random_string", random_string);
    tera.register_function("random_uint32", random_uint32);
    tera.register_function("random_uint64", random_uint64);
    tera.register_function("random_uuid", random_uuid);

    render_template(&mut tera, cli).expect("lol");
}

fn render_template(tera: &mut Tera, cli: Cli) -> anyhow::Result<()> {
    let context: Context = Context::new();

    match cli {
        Cli {
            file,
            count: Some(count),
            interval: Some(interval)
        } => {
            tera.add_template_file(file, Some("template"))?;
            let interval: core::time::Duration = interval.into();
            loop {
                let start: Instant = Instant::now();
                for _ in 0..count {
                    tera.render_to("template", &context, std::io::stdout())?;
                }
                if let Some(time_remaining) = interval.checked_sub(start.elapsed()) {
                    std::thread::sleep(time_remaining);
                }
            }
        },
        Cli {
            file,
            count: None,
            interval: None
        } => {
            tera.add_template_file(file, Some("template"))?;
            loop {
                tera.render_to("template", &context, std::io::stdout())?;
            }
        },
        _ => Err(TeraRandCliError::InvalidArguments.into())
    }
}
