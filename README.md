# Foliar

[![PyPI - Version](https://img.shields.io/pypi/v/foliar?style=for-the-badge)](https://pypi.org/project/foliar/)
![License](https://img.shields.io/github/license/campbellmbrown/foliar?style=for-the-badge)
![Pipeline](https://img.shields.io/github/actions/workflow/status/campbellmbrown/foliar/CI.yml?branch=main&style=for-the-badge&logo=github)
![Release](https://img.shields.io/github/v/release/campbellmbrown/foliar?style=for-the-badge&logo=github)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json&style=for-the-badge)

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
