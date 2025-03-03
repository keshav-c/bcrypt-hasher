# bcsh (Bcrypt string hasher)

### Development commands

```sh
cargo run -- --help
cargo run -- -n 100 mypassword
cargo run -- -n 10
cargo run --release -- mypassword
```

### Usage

```sh
Usage:
  bcsh [options] <password>

Options:
  -n, --rounds <number>    Number of bcrypt rounds (4-31, default: X)
  -h, --help               Show this help message
```
