use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyList;

fn pretty_print_internal(obj: &Bound<'_, PyAny>, indent: usize) -> PyResult<()> {
    let indent_str = " ".repeat(indent);

    if obj.hasattr("__dataclass_fields__").unwrap_or(false) {
        let name = obj
            .getattr("__class__")?
            .getattr("__name__")?
            .str()?
            .to_string();
        let dataclass_fields = obj.getattr("__dataclass_fields__")?;
        let fields = dataclass_fields.cast::<PyDict>()?;
        // If length is 0, just print the name
        // Otherwise, print the name and the fields
        if fields.len() == 0 {
            print!("{name}()");
        } else {
            println!("{name}(");
            for (key, _) in fields.iter() {
                let key_str = key.str()?.to_string();
                let value = obj.getattr(&key_str)?;
                print!("{}{}{}=", indent_str, " ".repeat(4), key_str);
                pretty_print_internal(&value, indent + 4)?;
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
                print!("{}{}", indent_str, " ".repeat(4));
                pretty_print_internal(&item, indent + 4)?;
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
                print!(
                    "{}{}{}: ",
                    indent_str,
                    " ".repeat(4),
                    key.str()?
                );
                pretty_print_internal(&value, indent + 4)?;
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
