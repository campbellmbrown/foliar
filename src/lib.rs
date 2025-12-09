use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyList;

fn pretty_print_internal(obj: &Bound<'_, PyAny>, indent: usize) -> PyResult<()> {
    const INDENT: usize = 4;
    const ONE_LEVEL_INDENT: &str = "    ";

    let indent_str = " ".repeat(indent);

    if obj.hasattr("__dataclass_fields__").unwrap_or(false) {
        let name = obj.getattr("__class__")?.getattr("__name__")?.str()?;
        let dataclass_fields = obj.getattr("__dataclass_fields__")?;
        let fields = dataclass_fields.cast::<PyDict>()?;
        if fields.len() == 0 {
            print!("{name}()");
        } else {
            println!("{name}(");
            for (key, _) in fields.iter() {
                let key_str = key.str()?;
                let value = obj.getattr(&key_str)?;
                print!("{indent_str}{ONE_LEVEL_INDENT}{key_str}=");
                pretty_print_internal(&value, indent + INDENT)?;
                println!(",");
            }
            print!("{indent_str})");
        }
    } else if obj.is_instance_of::<PyList>() {
        let list = obj.cast::<pyo3::types::PyList>()?;
        if list.len() == 0 {
            print!("[]");
        } else {
            println!("[");
            for item in list.iter() {
                print!("{indent_str}{ONE_LEVEL_INDENT}");
                pretty_print_internal(&item, indent + INDENT)?;
                println!(",");
            }
            print!("{indent_str}]");
        }
    } else if obj.is_instance_of::<PyDict>() {
        let dict = obj.cast::<PyDict>()?;
        if dict.len() == 0 {
            print!("{{}}");
        } else {
            println!("{{");
            for (key, value) in dict.iter() {
                let key_str = key.str()?;
                print!("{indent_str}{ONE_LEVEL_INDENT}{key_str}: ",);
                pretty_print_internal(&value, indent + INDENT)?;
                println!(",");
            }
            print!("{indent_str}}}");
        }
    } else {
        // Print as-is
        let repr = obj.repr()?.to_string();
        print!("{repr}");
    }

    Ok(())
}

#[pyfunction]
fn pretty_print(obj: &Bound<'_, PyAny>) -> PyResult<()> {
    pretty_print_internal(obj, 0)?;
    println!();
    Ok(())
}

/// A Python module implemented in Rust.
#[pyo3::pymodule]
mod foliar {
    #[pymodule_export]
    use super::pretty_print;
}
