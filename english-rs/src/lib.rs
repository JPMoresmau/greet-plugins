wai_bindgen_rust::export!("greeter.wai");
wai_bindgen_rust::import!("host.wai");

struct Greeter;

impl crate::greeter::Greeter for Greeter {
    /// The language we greet in.
    fn language() -> String {
        String::from("English")
    }

    /// Greet the given name.
    fn greet(name: String) -> String {
        let hour = host::hour();
        if hour < 12 {
            format!("Good morning, {name}!")
        } else if hour < 18 {
            format!("Good afternoon, {name}!")
        } else {
            format!("Good evening, {name}!")
        }
    }
}
