wai_bindgen_rust::export!("greeter.wai");

struct Greeter;

impl crate::greeter::Greeter for Greeter {
    /// The language we greet in.
    fn language() -> String {
        String::from("English")
    }

    /// Greet the given name.
    fn greet(name: String) -> String {
        format!("Hello, {name}!")
    }
}
