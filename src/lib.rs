mod pretty;

use std::io::Write;

use pyo3::prelude::*;

#[pyfunction]
fn pretty_print(obj: &Bound<'_, PyAny>) -> PyResult<()> {
    let mut stdout = std::io::stdout();
    pretty::print(obj, 0, &mut stdout)?;
    writeln!(stdout)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pyo3::pymodule]
mod foliar {
    #[pymodule_export]
    use super::pretty_print;
}
