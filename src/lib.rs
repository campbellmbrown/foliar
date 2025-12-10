mod pretty;

use std::io::Write;

use crate::pretty::{Config, print};

use pyo3::prelude::*;

#[pyclass]
pub struct Pretty {
    config: Config,
}

#[pymethods]
impl Pretty {
    #[new]
    #[pyo3(signature = (indent = 4))]
    fn new(indent: usize) -> Self {
        Self {
            config: Config { indent },
        }
    }

    /// The number of spaces to use for indentation. Default is 4.
    #[getter]
    fn indent(&self) -> usize {
        self.config.indent
    }

    #[setter]
    fn set_indent(&mut self, indent: usize) {
        self.config.indent = indent;
    }

    /// Pretty-print a Python object to standard output.
    ///
    /// Args:
    ///     obj: The Python object to pretty-print.
    fn print(&mut self, obj: &Bound<'_, PyAny>) -> PyResult<()> {
        let mut stdout = std::io::stdout();
        print(obj, &self.config, 0, &mut stdout)?;
        writeln!(stdout)?;
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pyo3::pymodule]
mod foliar {
    #[pymodule_export]
    use super::Pretty;
}
