use std::{env, fs};

use anyhow::{anyhow, Result};
use chrono::{self, Timelike};
use wasmtime::component::*;
use wasmtime::*;

wasmtime::component::bindgen!("greeter");

struct MyState {}

impl GreetImports for MyState {
    fn hour(&mut self) -> wasmtime::Result<u32> {
        let now = chrono::Local::now();
        Ok(now.hour())
    }
}

/// Greet using all the plugins.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: i18n-greeter <name>"));
    }
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = wasmtime::component::Linker::new(&engine);

    let paths = fs::read_dir("./plugins").unwrap();

    for path in paths {
        let path = path?;
        let component = Component::from_file(&engine, path.path())?;

        Greet::add_to_linker(&mut linker, |state: &mut MyState| state)?;

        let mut store = Store::new(&engine, MyState {});

        let (bindings, _) = Greet::instantiate(&mut store, &component, &linker)?;

        let language = bindings.greeter.call_language(&mut store)?;
        println!("Language: {language}");
        let greeting = bindings.greeter.call_greet(&mut store, &args[1])?;
        println!("Greeting: {greeting}");
    }
    Ok(())
}
