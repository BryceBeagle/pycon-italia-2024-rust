use pyo3::prelude::*;

/// Really cool Rust object
#[pyclass]
struct FooBar;

#[pymethods]
impl FooBar {
    #[classattr]
    fn hello_world() -> String {
        // TODO: Why does this duplicate the emoji?
        // >>> FooBar.hello_world.encode("unicode-escape")
        // b'Hello \\U0001f980\\U0001f980'
        return "Hello 🦀\u{01F980}".to_string();
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn what(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FooBar>()?;
    Ok(())
}