use std::io::Write;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

const INDENT: usize = 4;
const ONE_LEVEL_INDENT: &str = "    ";

const PYTHON_DUNDER_DATACLASS_FIELDS: &str = "__dataclass_fields__";
const PYTHON_DUNDER_CLASS: &str = "__class__";
const PYTHON_DUNDER_NAME: &str = "__name__";

/// Pretty-print a Python object to the given writer with the specified indentation.
///
/// # Arguments
/// - `obj`: The Python object to print.
/// - `indent`: The current indentation level.
/// - `w`: The writer to which the output will be written.
///
/// # Returns
/// A `PyResult<()>` indicating success or failure.
///
/// # Errors
/// Returns an error if writing to `w` fails or if interacting with the python object fails.
pub fn print(obj: &Bound<'_, PyAny>, indent: usize, w: &mut dyn Write) -> PyResult<()> {
    if obj.hasattr(PYTHON_DUNDER_DATACLASS_FIELDS).unwrap_or(false) {
        print_dataclass(obj, indent, w)?;
    } else if obj.is_instance_of::<PyList>() {
        print_list(obj.cast::<PyList>()?, indent, w)?;
    } else if obj.is_instance_of::<PyDict>() {
        print_dict(obj.cast::<PyDict>()?, indent, w)?;
    } else {
        // Print as-is
        let repr = obj.repr()?;
        write!(w, "{repr}")?;
    }

    Ok(())
}

fn print_dataclass(obj: &Bound<'_, PyAny>, indent: usize, w: &mut dyn Write) -> PyResult<()> {
    let name = obj
        .getattr(PYTHON_DUNDER_CLASS)?
        .getattr(PYTHON_DUNDER_NAME)?
        .str()?;
    let dataclass_fields = obj.getattr(PYTHON_DUNDER_DATACLASS_FIELDS)?;
    let fields = dataclass_fields.cast::<PyDict>()?;

    if fields.is_empty() {
        write!(w, "{name}()")?;
    } else {
        let indent_str = " ".repeat(indent);

        writeln!(w, "{name}(")?;
        for (key, _) in fields.iter() {
            let key_str = key.str()?;
            let value = obj.getattr(&key_str)?;
            write!(w, "{indent_str}{ONE_LEVEL_INDENT}{key_str}=")?;
            print(&value, indent + INDENT, w)?;
            writeln!(w, ",")?;
        }
        write!(w, "{indent_str})")?;
    }

    Ok(())
}

fn print_list(obj: &Bound<'_, PyList>, indent: usize, w: &mut dyn Write) -> PyResult<()> {
    if obj.is_empty() {
        write!(w, "[]")?;
    } else {
        let indent_str = " ".repeat(indent);

        writeln!(w, "[")?;
        for item in obj.iter() {
            write!(w, "{indent_str}{ONE_LEVEL_INDENT}")?;
            print(&item, indent + INDENT, w)?;
            writeln!(w, ",")?;
        }
        write!(w, "{indent_str}]")?;
    }

    Ok(())
}

fn print_dict(obj: &Bound<'_, PyDict>, indent: usize, w: &mut dyn Write) -> PyResult<()> {
    if obj.is_empty() {
        write!(w, "{{}}")?;
    } else {
        let indent_str = " ".repeat(indent);

        writeln!(w, "{{")?;
        for (key, value) in obj.iter() {
            write!(w, "{indent_str}{ONE_LEVEL_INDENT}")?;
            print(&key, indent + INDENT, w)?;
            write!(w, ": ")?;
            print(&value, indent + INDENT, w)?;
            writeln!(w, ",")?;
        }
        write!(w, "{indent_str}}}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_list() -> PyResult<()> {
        Python::initialize();
        Python::attach(|py| {
            let lst = pyo3::types::PyList::new(py, [1, 2, 3])?;
            let mut buf = Vec::new();

            let obj = lst.as_any();
            print(obj, 0, &mut buf)?;
            let out = String::from_utf8(buf)?;

            let expected = r#"[
    1,
    2,
    3,
]"#;
            assert_eq!(out, expected);
            Ok(())
        })
    }
}
