# Foliar

[![PyPI - Version](https://img.shields.io/pypi/v/foliar?style=for-the-badge)](https://pypi.org/project/foliar/)
[![License](https://img.shields.io/github/license/campbellmbrown/foliar?style=for-the-badge)](LICENSE)
[![Deploy](https://img.shields.io/github/actions/workflow/status/campbellmbrown/foliar/deploy.yaml?style=for-the-badge&logo=github&label=Deploy)](https://github.com/campbellmbrown/foliar/actions/workflows/deploy.yaml)
[![Develop](https://img.shields.io/github/actions/workflow/status/campbellmbrown/foliar/develop.yaml?branch=main&style=for-the-badge&logo=github&label=Develop)](https://github.com/campbellmbrown/foliar/actions/workflows/develop.yaml)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json&style=for-the-badge)

## Overview

> **foliar** [foh-lee-er]
>
> _adjective_: of, relating to, or having the nature of a leaf or leaves.

Are you sick of your Python prints looking like a tangled mess of nested dictionaries and lists, all on one line? Foliar serves as a simple tool to pretty-print complex Python data structures, making them easier to read at a glance.

For example, the following print is difficult to read:

```
ClassA(simple_bool=True, class_list=[ClassB(simple_int=42, simple_str='Hello,'), ClassB(simple_int=7, simple_str='world!')], simple_dict={'key1': 1, 'key2': 2})
```

With foliar, the same print becomes much more readable:

```
ClassA(
    simple_bool=True,
    class_list=[
        ClassB(
            simple_int=42,
            simple_str='Hello,',
        ),
        ClassB(
            simple_int=7,
            simple_str='world!',
        ),
    ],
    simple_dict={
        'key1': 1,
        'key2': 2,
    },
)
```

## Installation

Foliar can be installed via pip:

```bash
pip install foliar
```

## Usage

To use foliar, simply import it and create a `Pretty` object:

```python
from foliar import Pretty

pretty = Pretty()
pretty.print("Hello, World!")
```

You can customize the number of spaces to use for indentation by passing the `indent` parameter (defaults to 4):

```python
pretty = Pretty(indent=2)
```

## Known Issues

Pylint sometimes complains "*No name 'Pretty' in module 'foliar'*" ([E0611:no-name-in-module](https://pylint.readthedocs.io/en/latest/user_guide/messages/error/no-name-in-module.html)).
This is because [Pylint doesn't lint Rust-compiled extensions by default](https://pylint.pycqa.org/en/v2.11.1/technical_reference/c_extensions.html).
This can be fixed by using the [`extension-pkg-allow-list` option](https://pylint.readthedocs.io/en/latest/user_guide/messages/error/no-member.html) in your chosen Pylint configuration.

## Development

[maturin](https://www.maturin.rs/) is used to build and manage the Python package. To run the package in your development environment, use:

```bash
maturin develop
```

### Code Quality

To lint Rust code, run:

```bash
cargo clippy --fix --allow-dirty -- -W clippy::pedantic
```

To format Rust code, run:

```bash
cargo fmt
```

To lint Python code, run:

```bash
ruff check
```

To format Python code, run:
```bash
ruff format
```

To type check Python code, run:

```bash
mypy .
```
