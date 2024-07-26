# SHAPES-rs

[![Shapes-rs](https://github.com/weso/shapes-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/weso/shapes-rs/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/weso/shapes-rs/status.svg)](https://deps.rs/repo/github/weso/shapes-rs)

This repo contains an RDF data shapes library implemented in Rust.
The implementation supports [ShEx](http://shex.io/), [SHACL](https://www.w3.org/TR/shacl/), [DCTap](https://www.dublincore.org/specifications/dctap/) and conversions between different RDF data modeling formalisms.

The code can be used as a Rust library but it also contains a binary called `rdfsx` which can be used as an RDF playground.

We provide binaries for Linux, Windows, Mac and Docker (see [releases](https://github.com/weso/shapes-rs/releases)), as well as Python bindings.

- [List of issues](https://github.com/weso/shapes-rs)
- [Discussion](https://github.com/weso/shapes-rs/discussions/landing)
- [FAQ](https://github.com/weso/shapes-rs/wiki/FAQ)
- [How to guides](https://github.com/weso/shapes-rs/wiki/Howto-guides)
- [Roadmap](https://github.com/weso/shapes-rs/issues/1)

## Installation

### Official releases

You can download a binary from the [latest release](https://github.com/weso/shapes-rs/releases/latest) page. There you will also find the compiled packages for the installation on your system using a package manager.

#### Ubuntu

Download the binary from [https://github.com/weso/shapes-rs/releases] and install the `.deb` package running the following commands after replacing X.X.X by the latest version:

```
wget https://github.com/weso/shapes-rs/releases/download/X.X.X/rdfsx_vX.X.X_amd64.deb
sudo dpkg -i rdfsx_vX.X.X_amd64.deb
```

#### Windows

The binary can be downloaded from [https://github.com/weso/shapes-rs/releases]

#### Mac

The binary is available at: [https://github.com/weso/shapes-rs/releases]

<details markdown="block">
<summary>Compiling from source</summary>

### Compiling from source

`shapes-rs` has been implemented in Rust and is compiled using [cargo](https://doc.rust-lang.org/cargo/). The command `cargo run` can be used to compile and run locally the code.

For example:

```
cargo run -- validate --data examples/user.ttl --schema examples/user.shex --shapemap examples/user.sm 
```

### Compiling from source and installing the binary (Debian)

Install `cargo deb` (only the first time)

```
cargo install cargo-deb
```

Create the `.deb` package by:

```
cargo deb
```

And run:

```
sudo dpkg -i target/debian/shapes-rs_0.0.11-1_amd64.deb
```

## Docker

The library is also published as a Docker image.

</details>

## Usage

### Some examples

The folder `examples` contains several example files with ShEx schemas and RDF data.

### Validate a simple RDF file with a ShEx schema using a ShapeMap

```sh
rdfsx validate --data examples/user.ttl --schema examples/user.shex --shapemap examples/user.sm
```

We maintain a Wiki page with some common [Usage scenarios and How-to guides](https://github.com/weso/shapes-rs/wiki/Howto-guides).

### Debugging information

It is possible to change the debug level information with:

```sh
export RUST_LOG=value
```

where `value` can be `debug` to show more verbose information or `info` to show basic information.

## Command line usage

```sh
$ rdfsx --help
Usage: sx [OPTIONS] [COMMAND]
Commands:
  schema
  validate
  data
  node
  help
          Print this message or the help of the given subcommand(s)
```

### Obtaining information about a schema

```sh
Usage: rdfsx schema [OPTIONS] --schema <Schema file name>

Options:
  -s, --schema <Schema file name>
          
  -f, --schema-format <Schema format>
          [default: shexj] [possible values: internal, shexc, shexj]
  -r, --result-schema-format <Result schema format>
          [default: shexj] [possible values: internal, shexc, shexj]
  -h, --help
          Print help
```

### Obtaining information about RDF data

```sh
Usage: rdfsx data [OPTIONS] --data <RDF data path>

Options:
  -d, --data <RDF data path>           
  -t, --data-format <RDF Data format>  [default: turtle] [possible values: turtle]
  -h, --help                           Print help
```

### Obtaining information about a node in RDF data

This command can be useful to obtain the neighbourhood of a node.

```sh
Usage: rdfsx node [OPTIONS] --node <NODE>

Options:
  -n, --node <NODE>
          
  -d, --data <RDF data path>
          
  -t, --data-format <RDF Data format>
          [default: turtle] [possible values: turtle]
  -e, --endpoint <Endpoint with RDF data>
          
  -m, --show-node-mode <Show Node Mode>
          [default: outgoing] [possible values: outgoing, incoming, both]
  -h, --help
          Print help  
```

For example, the following command shows the neighbourhood of node `wd:Q80` in the Wikidata endpoint.

```sh
rdfsx node -e wikidata -n wd:Q80
```

### Validating an RDF node against some data

Example: Assuming there a ShEx file in `examples/user.shex` and an RDF turtle file in `examples/user.ttl` we can ask to validate node `:a` with shape label `:User` using:

```sh
rdfsx validate -s examples/user.shex -d examples/user.ttl -n :a -l :User
```

If there is a shapemap in `examples/user.sm`, we can validate using:

```sh
rdfsx validate -s examples/user.shex -d examples/user.ttl -m examples/user.sm
```

The full help for the `validate` subcommand is:

```sh
Usage: rdfsx validate [OPTIONS] --schema <Schema file name> --data <RDF data path>

Options:
  -s, --schema <Schema file name>
          
  -f, --schema-format <Schema format>
          [default: shexc] [possible values: internal, shexc, shexj]
  -m, --shapemap <ShapeMap file name>
          
      --shapemap-format <ShapeMap format>
          [default: compact] [possible values: compact, internal]
      --result-shapemap-format <Result shapemap format>
          [default: compact] [possible values: compact, internal]
  -n, --node <NODE>
          
  -l, --shape-label <shape label (default = START)>
          
  -d, --data <RDF data path>
          
  -t, --data-format <RDF Data format>
          [default: turtle] [possible values: turtle]
      --max-steps <max steps to run>
          [default: 100]
  -h, --help
          Print help
```

### Validating an RDF node against some SHACL Shape

```sh
rdfsx validate-shacl --shapes examples/simple_shacl.ttl --data examples/simple.ttl
```

## Main modules

The repo is divided in the following modules:

- [iri_s](https://github.com/weso/shapes-rs/tree/master/iri_s) defines simple IRIs.
- [srdf](https://github.com/weso/shapes-rs/tree/master/srdf) simple RDF model which will be used for validation.
- [prefixmap](https://github.com/weso/shapes-rs/tree/master/prefixmap) Prefix maps implementation.
- [shapemap](https://github.com/weso/shapes-rs/tree/master/shapemap) ShapeMap implementation.
- [shex_ast](https://github.com/weso/shapes-rs/tree/master/shex_ast) defines the ShEx Abstract syntax
- [shex_compact](https://github.com/weso/shapes-rs/tree/master/shex_compact) contains the code required to handle ShEx compact syntax.
- [shex_validation](https://github.com/weso/shapes-rs/tree/master/shex_validation) contains the code required to validate RDF using ShEx.
- [shex_testsuite](https://github.com/weso/shapes-rs/tree/master/shex_testsuite) contains the code required to run the ShEx testsuite.
- [shacl_ast](https://github.com/weso/shapes-rs/tree/master/shacl_ast) defines the SHACL core Abstract syntax.

## Publishing the crates

```sh
cargo workspaces publish 
```

## Worskpaces

The project is using cargo workspaces wihch can be installed with:

```
cargo install cargo-workspaces
```

## Unit-testing

In order to test all the sub-projects

```
cargo test --all
```

Testing one specific subproject:

```
cargo test -p shex_validation
```

## Using the ShEx test-suite

The ShEx testsuite is included in a git submodule. In order to obtain it, it is necessary to do:

```sh
git submodule update --init --recursive
cargo run -p shex_testsuite
```

```sh
Usage: shex_testsuite [OPTIONS]

Options:
  -m, --manifest <Manifest FILE (.jsonld)>
          Name of Manifest file [default: shex_testsuite/shexTest/validation/manifest.jsonld]
  -c, --config <Config file>
          [default: shex_testsuite/config.yml]
  -x, --run_mode <MANIFEST_RUN_MODE>
          [default: collect-errors] [possible values: collect-errors, fail-first-error]
  -f, --manifest_mode <MANIFEST_MODE>
          [possible values: schemas, validation, negative-syntax, negative-structure]
  -p, --print_result_mode <PRINT_RESULT_MODE>
          [default: basic] [possible values: basic, failed, passed, not-implemented, all]
  -e, --entry <Entry names>
          
  -t, --trait <Trait names>
          
  -h, --help
          Print help
  -V, --version
          Print version
```

### Validation tests

```
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/validation/manifest.jsonld validation 
```

### Schemas tests

```sh
cargo run -p shex_testsuite -- -m shex_testsuite/shexTest/schemas/manifest.jsonld -f schemas -p failed
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

### Contribution

Unless you explicitly state otherwise,
any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
