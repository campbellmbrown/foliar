use foliar::pretty::print;
use pyo3::prelude::*;
use std::io::Cursor;

#[test]
fn test_list() -> PyResult<()> {
    Python::initialize();
    Python::attach(|py| {
        let lst = pyo3::types::PyList::new(py, [1, 2, 3])?;
        let mut buf = Cursor::new(Vec::new());

        let obj = lst.as_any();
        print(obj, 0, &mut buf)?;
        let out = String::from_utf8(buf.into_inner())?;

        let expected = r#"[
    1,
    2,
    3,
]"#;
        assert_eq!(out, expected);

        Ok(())
    })
}
