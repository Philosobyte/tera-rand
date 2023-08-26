//! # tera-rand-cli
//!
//! A CLI tool for generating a feed of random data from a [Tera] template.
//!
//! # Examples
//! Let's say this Tera template is located in a file at `cpu_util.json`:
//! ```json
//! {"hostname": "{{ random_string() }}", "cpu_util": {{ random_uint32(start=0, end=100) }}}
//! ```
//!
//! If we run the following command:
//! ```
//! tera-rand-cli -f cpu_util.json
//! ```
//!
//! we should see the template in `cpu_util.json` being rendered as quickly as possible into
//! standard output:
//! ```jsonl
//! {"hostname": "hZ6dguUP", "cpu_util": 4}
//! {"hostname": "wgRDsuEv", "cpu_util": 76}
//! {"hostname": "v2EHobFJ", "cpu_util": 85}
//! {"hostname": "gwREN077", "cpu_util": 79}
//! (etc.)
//! ```
//!
//! To slow down the rate of data generated, we can provide arguments to the `batch_size` and
//! `batch_interval` options. Here, we tell tera-rand-cli to generate only 5 records every second:
//! ```
//! tera-rand-cli -f cpu_util.json --batch-size 5 --batch-interval PT1S
//! ```
//!
//! To exit after generating after a certain number of records, use `--record-limit`. Similarly,
//! `--time-limit` tells the program to exit after a certain amount of time. If both options are
//! used, the program will exit as soon as just one of the exit conditions becomes true.
//!
//! This tool is built on top of the Tera templating library, so all of its advanced capabilities
//! are available for constructing these templates. Here is a slightly more complex example of a
//! Tera template making use of random functions. This template initializes a `count` variable to
//! a random integer between 1 and 10, and then generates exactly `count` number of random integers
//! inside the array named `src_ports`:
//!
//! ```json
//! {
//!   "protocol": "{{ random_from_file(path="tera-rand-cli/resources/test/protocol.txt") }}",
//!   {% set count = random_uint32(start=1, end=10) -%}
//!   "count": {{ count }},
//!   "timestamp": {{ now() }},
//!   "flow_id": {{ random_uuid() }},
//!   "src_hostname": "{{ random_string(length=10) }}",
//!   "src_addr": "{{ random_ipv4() }}",
//!   "src_ports": [
//!     {% for i in range(end=count) -%}
//!       {{ random_uint32(start=49152, end=65535) }}{% if not loop.last %}, {% endif %}
//!     {%- endfor %}
//!   ],
//!   "src_app": "{{ random_string() }}",
//!
//!   "dst_hostname": "{{ random_string(length=10) }}",
//!   "dst_addr":  "{{ random_ipv4() }}",
//!   "dst_port": {{ random_uint32(end=49151) }},
//!   "dst_app": "{{ random_string() }}"
//! }
//! ```
//!
//! The rendered output for that template could look like this:
//! ```json
//! {
//!   "protocol": "UDP",
//!   "count": 8,
//!   "timestamp": 2023-08-25T21:50:20.836769600-04:00,
//!   "flow_id": 3944799d-1f60-40fc-9b0d-35c02ab017ec,
//!   "src_hostname": "trEi25xe44",
//!   "src_addr": "213.203.1.172",
//!   "src_ports": [
//!     58382, 51005, 63169, 59766, 64632, 52953, 55543, 63626
//!   ],
//!   "src_app": "YEl34jzn",
//!
//!   "dst_hostname": "lTnsk5uVZC",
//!   "dst_addr":  "28.85.27.180",
//!   "dst_port": 29036,
//!   "dst_app": "wybCthJU"
//! }
//! ```
//! See Tera's documentation for a more in-depth look at templates:
//! <https://keats.github.io/tera/docs/#getting-started>
//!
//! ## Use cases
//!
//! We can redirect this output to, say, a curl command to simulate traffic on a REST endpoint:
//! ```
//! tera-rand-cli -f cpu_util.json | curl -H "Content-Type: application/json" -X POST -d @- http://localhost:80
//! ```
//!
//! or we could produce to a Kafka topic:
//! ```
//! tera-rand-cli -f cpu_util.json | kafka-console-producer --bootstrap-server localhost:9092 --topic cpu-util
//! ```
//!
//! This tool is intended to help in scenarios where
//! 1. generated records should preferably be distinguishable from each other, i.e. not the exact
//!    same record a thousand times, or
//! 2. the schema might change often enough or there are a large enough number of data types that
//!    maintaining a set of templates would be easier than maintaining an equivalent data generator
//!    implemented directly in code.
//!
//! While this tool might be convenient for benchmarking, it is not intended to be as fast as
//! possible. It is slower than a tool which defines templates or schemas at compile time.
//!
//! [Tera]: https://github.com/Keats/tera
#![warn(missing_debug_implementations, missing_docs)]

mod error;

use std::path::PathBuf;
use std::time::Instant;

use crate::error::TeraRandCliError;
use clap::Parser;
use iso8601::Duration;
use tera::{Context, Tera};
use tera_rand::{
    random_bool, random_char, random_float32, random_float64, random_from_file, random_int32,
    random_int64, random_ipv4, random_ipv4_cidr, random_ipv6, random_ipv6_cidr, random_string,
    random_uint32, random_uint64, random_uuid,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// filepath of the Tera template to render.
    #[arg(short, long)]
    file: PathBuf,
    /// number of times to render and output the template per `batch_interval`. This is optional,
    /// but if an argument is provided for `batch_size`, then an argument should be provided for
    /// `batch_interval`, as well.
    #[arg(long)]
    batch_size: Option<u32>,
    /// how often to render and output the template. This is optional, but if an argument is
    /// provided for `batch_size`, then an argument should be provided for
    /// `batch_interval`, as well.
    #[arg(long)]
    batch_interval: Option<Duration>,
    /// the maximum ISO 8601 duration for which this program should render and output the template.
    /// The program exits when it reaches either `time_limit` or `record_limit`, whichever comes
    /// first.
    #[arg(short, long)]
    time_limit: Option<Duration>,
    /// the maximum number of times this program should render and output the template.
    /// The program exits when it reaches either `record_limit` or `time_limit`, whichever comes
    /// first.
    #[arg(short, long)]
    record_limit: Option<u32>,
}

fn main() {
    let cli_args: CliArgs = CliArgs::parse();
    let mut tera: Tera = Tera::default();

    register_tera_rand_functions(&mut tera);
    render_template(&mut tera, cli_args).unwrap_or_else(|e| {
        eprintln!("Encountered a fatal error: {e:?}");
        std::process::exit(1)
    });
}

fn register_tera_rand_functions(tera: &mut Tera) {
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
}

/// Use the Tera instance passed in to render the template provided by the user via the command
/// line. Depending on the command line options, this function may run in an infinite loop.
fn render_template(tera: &mut Tera, cli_args: CliArgs) -> anyhow::Result<()> {
    let context: Context = Context::new();

    // the base logic when just filename is specified is just "render a template in an infinite
    // loop". It is so simple that each cli argument has a proportionally large impact on the logic.
    // So, instead of trying to check options on the fly, just lay out each possible, valid
    // combination of cli arguments individually. We may have to rethink this if the number of
    // arguments grows (and thus the number of combinations).

    // batch_size and batch_interval go hand in hand. In this outer match block, do not allow one
    // of the two arguments to be specified without the other.
    match cli_args {
        CliArgs {
            file,
            batch_size: None,
            batch_interval: None,
            record_limit: total_records,
            time_limit: total_duration,
        } => {
            tera.add_template_file(file, Some("template"))?;
            match (total_records, total_duration) {
                (None, None) => loop {
                    tera.render_to("template", &context, std::io::stdout())?;
                },
                (Some(total_records), None) => {
                    for _ in 0..total_records {
                        tera.render_to("template", &context, std::io::stdout())?;
                    }
                    Ok(())
                }
                (None, Some(total_duration)) => {
                    let total_duration: core::time::Duration = total_duration.into();
                    let program_start_time: Instant = Instant::now();

                    while total_duration
                        .checked_sub(program_start_time.elapsed())
                        .is_some()
                    {
                        tera.render_to("template", &context, std::io::stdout())?;
                    }
                    Ok(())
                }
                (Some(total_records), Some(total_duration)) => {
                    let total_duration: core::time::Duration = total_duration.into();
                    let program_start_time: Instant = Instant::now();
                    let mut records_remaining: u32 = total_records;

                    while total_duration
                        .checked_sub(program_start_time.elapsed())
                        .is_some()
                        && records_remaining > 0
                    {
                        tera.render_to("template", &context, std::io::stdout())?;
                        records_remaining -= 1;
                    }
                    Ok(())
                }
            }
        }
        CliArgs {
            file,
            batch_size: Some(batch_size),
            batch_interval: Some(batch_interval),
            record_limit: total_records,
            time_limit: total_duration,
        } => {
            tera.add_template_file(file, Some("template"))?;
            let batch_interval: core::time::Duration = batch_interval.into();

            match (total_records, total_duration) {
                (None, None) => {
                    loop {
                        let loop_start_time: Instant = Instant::now();
                        // render a batch
                        for _ in 0..batch_size {
                            tera.render_to("template", &context, std::io::stdout())?;
                        }
                        // sleep off the time left
                        if let Some(time_remaining) =
                            batch_interval.checked_sub(loop_start_time.elapsed())
                        {
                            std::thread::sleep(time_remaining);
                        }
                    }
                }
                (Some(total_records), None) => {
                    let mut remaining_records: u32 = total_records;

                    // produce until we've hit our record limit
                    while remaining_records > 0u32 {
                        let loop_start_time: Instant = Instant::now();

                        let current_batch_size: u32 = if remaining_records > batch_size {
                            batch_size
                        } else {
                            remaining_records
                        };
                        // render a batch
                        for _ in 0..current_batch_size {
                            tera.render_to("template", &context, std::io::stdout())?;
                        }

                        remaining_records -= current_batch_size;
                        // sleep off the time left
                        if let Some(time_remaining) =
                            batch_interval.checked_sub(loop_start_time.elapsed())
                        {
                            std::thread::sleep(time_remaining);
                        }
                    }
                    Ok(())
                }
                (None, Some(total_duration)) => {
                    let total_duration: core::time::Duration = total_duration.into();
                    let program_start_time: Instant = Instant::now();

                    // produce until we've hit our time limit
                    while total_duration
                        .checked_sub(program_start_time.elapsed())
                        .is_some()
                    {
                        let loop_start_time: Instant = Instant::now();
                        // render a batch
                        for _ in 0..batch_size {
                            tera.render_to("template", &context, std::io::stdout())?;
                        }
                        // sleep off the time left
                        if let Some(time_remaining) =
                            batch_interval.checked_sub(loop_start_time.elapsed())
                        {
                            std::thread::sleep(time_remaining);
                        }
                    }
                    Ok(())
                }
                (Some(total_records), Some(total_duration)) => {
                    let mut records_remaining: u32 = total_records;
                    let total_duration: core::time::Duration = total_duration.into();
                    let program_start_time: Instant = Instant::now();

                    // produce until we've hit our record limit or our time limit,
                    // whichever comes first
                    while records_remaining > 0u32
                        && total_duration
                            .checked_sub(program_start_time.elapsed())
                            .is_some()
                    {
                        let loop_start_time: Instant = Instant::now();

                        let current_batch_size: u32 = if records_remaining > batch_size {
                            batch_size
                        } else {
                            records_remaining
                        };
                        // render a batch
                        for _ in 0..current_batch_size {
                            tera.render_to("template", &context, std::io::stdout())?;
                        }

                        records_remaining -= current_batch_size;
                        // sleep off the time left
                        if let Some(time_remaining) =
                            batch_interval.checked_sub(loop_start_time.elapsed())
                        {
                            std::thread::sleep(time_remaining);
                        }
                    }
                    Ok(())
                }
            }
        }
        _ => Err(TeraRandCliError::InvalidBatchArguments.into()),
    }
}
