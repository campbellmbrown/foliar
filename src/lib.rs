use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::io::Write;

fn pretty_print_internal(obj: &Bound<'_, PyAny>, indent: usize, w: &mut dyn Write) -> PyResult<()> {
    const INDENT: usize = 4;
    const ONE_LEVEL_INDENT: &str = "    ";

    let indent_str = " ".repeat(indent);

    if obj.hasattr("__dataclass_fields__").unwrap_or(false) {
        let name = obj.getattr("__class__")?.getattr("__name__")?.str()?;
        let dataclass_fields = obj.getattr("__dataclass_fields__")?;
        let fields = dataclass_fields.cast::<PyDict>()?;
        if fields.is_empty() {
            write!(w, "{name}()")?;
        } else {
            writeln!(w, "{name}(")?;
            for (key, _) in fields.iter() {
                let key_str = key.str()?;
                let value = obj.getattr(&key_str)?;
                write!(w, "{indent_str}{ONE_LEVEL_INDENT}{key_str}=")?;
                pretty_print_internal(&value, indent + INDENT, w)?;
                writeln!(w, ",")?;
            }
            write!(w, "{indent_str})")?;
        }
    } else if obj.is_instance_of::<PyList>() {
        let list = obj.cast::<PyList>()?;
        if list.is_empty() {
            write!(w, "[]")?;
        } else {
            writeln!(w, "[")?;
            for item in list.iter() {
                write!(w, "{indent_str}{ONE_LEVEL_INDENT}")?;
                pretty_print_internal(&item, indent + INDENT, w)?;
                writeln!(w, ",")?;
            }
            write!(w, "{indent_str}]")?;
        }
    } else if obj.is_instance_of::<PyDict>() {
        let dict = obj.cast::<PyDict>()?;
        if dict.is_empty() {
            write!(w, "{{}}")?;
        } else {
            writeln!(w, "{{")?;
            for (key, value) in dict.iter() {
                let key_str = key.str()?;
                write!(w, "{indent_str}{ONE_LEVEL_INDENT}{key_str}: ")?;
                pretty_print_internal(&value, indent + INDENT, w)?;
                writeln!(w, ",")?;
            }
            write!(w, "{indent_str}}}")?;
        }
    } else {
        // Print as-is
        let repr = obj.repr()?.to_string();
        write!(w, "{repr}")?;
    }

    Ok(())
}

#[pyfunction]
fn pretty_print(obj: &Bound<'_, PyAny>) -> PyResult<()> {
    let mut stdout = std::io::stdout();
    pretty_print_internal(obj, 0, &mut stdout)?;
    writeln!(stdout)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pyo3::pymodule]
mod foliar {
    #[pymodule_export]
    use super::pretty_print;
}
