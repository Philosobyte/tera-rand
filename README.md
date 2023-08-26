# tera-rand

[![build status]][actions]
[![latest version]][crates.io]
[![docs]][`tera-rand` documentation]
[![rustc version 1.72+]][rust 1.72]

## tera-rand
`tera-rand` is a library of random data generation functions for the [Tera] template engine. 

To use `tera-rand` in your project, include the following in your `Cargo.toml`:
```toml
[dependencies]
tera-rand = "0.1.2"
```

Please see [`tera-rand` documentation] for examples on using `tera-rand` functions.

## tera-rand-cli

`tera-rand-cli` is a command-line tool for generating a feed of random data from a [Tera] template. 
This random data can be useful for tasks such as simulating traffic or populate data stores.

### Installation

You can install a `tera-rand-cli` binary from crates.io using `cargo install tera-rand-cli@0.1.1`. 

Alternatively, if you would like to build from source, ensure you have Rust installed at version
1.72 or higher. Then, checkout this repository and run`cargo build --release` from the root 
project directory. The binary should be located under directory `target/release`.

### Examples
Let's say this Tera template is located in a file at `cpu_util.json`:
```json
{"hostname": "{{ random_string() }}", "cpu_util": {{ random_uint32(start=0, end=100) }}}
```
If we run the following command:
```
tera-rand-cli -f cpu_util.json
```
we should see the template in `cpu_util.json` being rendered as quickly as possible into
standard output:
```jsonl
{"hostname": "hZ6dguUP", "cpu_util": 4}
{"hostname": "wgRDsuEv", "cpu_util": 76}
{"hostname": "v2EHobFJ", "cpu_util": 85}
{"hostname": "gwREN077", "cpu_util": 79}
(etc.)
```
To slow down the rate of data generated, we can provide arguments to the `batch_size` and
`batch_interval` options. Here, we tell tera-rand-cli to generate only 5 records every second:
```
tera-rand-cli -f cpu_util.json --batch-size 5 --batch-interval PT1S
```
To exit after generating after a certain number of records, use `--record-limit`. Similarly,
`--time-limit` tells the program to exit after a certain amount of time. If both options are
used, the program will exit as soon as just one of the exit conditions becomes true.
This tool is built on top of the Tera templating library, so all of its advanced capabilities
are available for constructing these templates. Here is a slightly more complex example of a
Tera template making use of random functions. This template initializes a `count` variable to
a random integer between 1 and 10, and then generates exactly `count` number of random integers
inside the array named `src_ports`:
```json
{
  "protocol": "{{ random_from_file(path="tera-rand-cli/resources/test/protocol.txt") }}",
  {% set count = random_uint32(start=1, end=10) -%}
  "count": {{ count }},
  "timestamp": {{ now() }},
  "flow_id": {{ random_uuid() }},
  "src_hostname": "{{ random_string(length=10) }}",
  "src_addr": "{{ random_ipv4() }}",
  "src_ports": [
    {% for i in range(end=count) -%}
      {{ random_uint32(start=49152, end=65535) }}{% if not loop.last %}, {% endif %}
    {%- endfor %}
  ],
  "src_app": "{{ random_string() }}",
  "dst_hostname": "{{ random_string(length=10) }}",
  "dst_addr":  "{{ random_ipv4() }}",
  "dst_port": {{ random_uint32(end=49151) }},
  "dst_app": "{{ random_string() }}"
}
```
The rendered output for that template could look like this:
```json
{
  "protocol": "UDP",
  "count": 8,
  "timestamp": 2023-08-25T21:50:20.836769600-04:00,
  "flow_id": 3944799d-1f60-40fc-9b0d-35c02ab017ec,
  "src_hostname": "trEi25xe44",
  "src_addr": "213.203.1.172",
  "src_ports": [
    58382, 51005, 63169, 59766, 64632, 52953, 55543, 63626
  ],
  "src_app": "YEl34jzn",
  "dst_hostname": "lTnsk5uVZC",
  "dst_addr":  "28.85.27.180",
  "dst_port": 29036,
  "dst_app": "wybCthJU"
}
```
See Tera's documentation for a more in-depth look at templates:
<https://keats.github.io/tera/docs/#getting-started>

### Use cases
We can redirect this output to, say, a curl command to simulate traffic on a REST endpoint:
```
tera-rand-cli -f cpu_util.json | curl -H "Content-Type: application/json" -X POST -d @- http://localhost:80
```
or we could produce to a Kafka topic:
```
tera-rand-cli -f cpu_util.json | kafka-console-producer --bootstrap-server localhost:9092 --topic cpu-util
```
This tool is intended to help in scenarios where
1. generated records should preferably be distinguishable from each other, i.e. not the exact
   same record a thousand times, or
2. the schema might change often enough or there are a large enough number of data types that
   maintaining a set of templates would be easier than maintaining an equivalent data generator
   implemented directly in code.
   While this tool might be convenient for benchmarking, it is not intended to be as fast as
   possible. It is slower than a tool which defines templates or schemas at compile time.
   [Tera]: https://github.com/Keats/tera

[build status]: https://img.shields.io/github/actions/workflow/status/philosobyte/tera-rand/ci.yml?branch=main
[actions]: https://github.com/philosobyte/tera-rand/actions?query=branch%3Amain
[latest version]: https://img.shields.io/crates/v/tera_rand.svg
[crates.io]: https://crates.io/crates/tera-rand
[docs]: https://docs.rs/tera-rand/badge.svg
[rustc version 1.72+]: https://img.shields.io/badge/rustc-1.72+-lightgray.svg
[rust 1.72]: https://blog.rust-lang.org/2023/08/24/Rust-1.72.0.html
[`tera-rand` documentation]: https://docs.rs/tera-rand
[`tera-rand-cli` documentation]: https://docs.rs/tera-rand-cli

[Tera]: https://github.com/Keats/tera
