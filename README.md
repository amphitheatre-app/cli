# Amphitheatre’s official command line tool

`amp` is Amphitheatre on the command line. It brings develop, deploy and other
Amphitheatre concepts to the terminal next to where you are already working with
Kubernetes and your code.

[![License](https://img.shields.io/github/license/amphitheatre-app/cli)](https://github.com/amphitheatre-app/cli/blob/master/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/amphitheatre-app/cli)](https://github.com/amphitheatre-app/cli/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/amphitheatre-app/cli)](https://github.com/amphitheatre-app/cli/issues)

## Documentation

Docs are available at https://docs.amphitheatre.app/cli/ - we are still working
on refining it and contributions are welcome!

## Installation

Binaries for Windows, Linux and macOS are available [from Github](https://github.com/amphitheatre-app/cli/releases/latest).

Compiling from this repository also works similarly:

```
git clone https://github.com/amphitheatre-app/cli.git
cd cli
cargo build --release
```

Compilation will probably take a few minutes depending on your machine.
The binary will end up in `./target/release/amp`.

## Contributing

If anything feels off, or if you feel that some functionality is missing, please
check out the [contributing page](https://docs.amphitheatre.app/contributing/).
There you will find instructions for sharing your feedback, building the tool
locally, and submitting pull requests to the project.

## License

Copyright 2023 The Amphitheatre Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
