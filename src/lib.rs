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
// Using experimental experimental-declarative-modules feature so we can use pymodule_init
#[pymodule]
mod what {
    use super::*;

    #[pymodule_export]
    use super::FooBar;

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Arbitrary code to run at the module initialization
        println!("libwhat.so imported!");
        Ok(())
    }

}

// #[pymodule]
// fn what(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<FooBar>()?;
//     Ok(())
// }

