# Amphitheatre’s official command line tool

`amp` is Amphitheatre on the command line. It brings develop, deploy and other
Amphitheatre concepts to the terminal next to where you are already working with
Kubernetes and your code.

[![License](https://img.shields.io/github/license/amphitheatre-app/cli)](https://github.com/amphitheatre-app/cli/blob/main/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/amphitheatre-app/cli)](https://github.com/amphitheatre-app/cli/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/amphitheatre-app/cli)](https://github.com/amphitheatre-app/cli/issues)

## Documentation

Docs are available at https://docs.amphitheatre.app/cli/ - we are still working
on refining it and contributions are welcome!

## Installation

### Pre-build binaries

Binaries for Windows, Linux and macOS are available from [Github release page](https://github.com/amphitheatre-app/cli/releases/latest).


### From source

To build Amphitheatre CLI from source, you will need to have Git, Rust and Cargo installed.

From a terminal, you can now run the following commands:

```
git clone https://github.com/amphitheatre-app/cli.git
cd cli
cargo build --release
```

Compilation will probably take a few minutes depending on your machine.
The binary will end up in `./target/release/amp`.  You can move it in your `$PATH` to have the `amp` command available globally:

```
cp target/release/amp ~/.cargo/bin/amp
```

### Install the latest development build through git

To get the latest bug fixes and features, you can install the development version from git. However, this is not fully tested. That means you're probably going to have more bugs despite having the latest bug fixes.

```
cargo install --git https://github.com/amphitheatre-app/cli.git amp
```

This will download the CLI from the master branch, and install it in Cargo's global binary directory (`~/.cargo/bin/` by default).

Run `amp --help` for a list of all the available commands. Furthermore, you can run `amp <command> --help` to get help with a specific command.

## Contributing

If anything feels off, or if you feel that some functionality is missing, please
check out the [contributing page](https://docs.amphitheatre.app/contributing/).
There you will find instructions for sharing your feedback, building the tool
locally, and submitting pull requests to the project.

## License

Copyright (c) The Amphitheatre Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
