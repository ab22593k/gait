# Wire

Wires part of other repository's source code into the repository in a declarative manner.

## Features

- **Declarative Synchronization**: Define external code dependencies in a `.gitwire` JSON file.
- **Multiple Checkout Methods**: Supports `shallow`, `shallow_no_sparse`, and `partial` git checkout strategies.
- **Efficient Caching**: Avoids redundant git pull operations by caching repositories, significantly improving performance for multiple configurations referencing the same remote.
- **Concurrent Execution**: Performs sync and check operations in parallel by default, with an option for single-threaded execution.
- **Verification**: `check` command to verify if synchronized code is identical to the original source.
- **Direct Operations**: `direct-sync` and `direct-check` commands for immediate synchronization or verification using command-line arguments.
