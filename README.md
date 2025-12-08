# Foliar

## Development

[maturin](https://www.maturin.rs/) is used to build and manage the Python package. To run the package in your development environment, use:

```bash
maturin develop
```

### Code Quality

To lint the codebase, run:

```bash
cargo clippy --fix --allow-dirty -- -W clippy::pedantic
```

To format the codebase, run:

```bash
cargo fmt
```
