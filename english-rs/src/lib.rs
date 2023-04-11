use greeter::Greeter;


wit_bindgen::generate!("greeter");

struct MyGreeter;

impl Greeter for MyGreeter {

    /// The language we greet in.
    fn language() -> String {
        String::from("English")
    }

    /// Greet the given name.
    fn greet(name: String) -> String {
        let hour = hour();
        if hour < 12 {
            format!("Good morning, {name}!")
        } else if hour < 18 {
            format!("Good afternoon, {name}!")
        } else {
            format!("Good evening, {name}!")
        }
    }
}

export_greet!(MyGreeter);
