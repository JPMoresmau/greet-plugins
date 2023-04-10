use std::{env, fs};

use anyhow::{anyhow, Result};
use greeter::{Greeter, GreeterData};
use wasmer::*;
use wasmer_compiler_llvm::LLVM;

wai_bindgen_wasmer::import!("greeter.wai");

/// Greet using all the plugins.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: i18n-greeter <name>"));
    }
    let compiler_config = LLVM::default();
    let engine = EngineBuilder::new(compiler_config).engine();

    let paths = fs::read_dir("./plugins").unwrap();

    for path in paths {
        let path = path?;
        let mut store = Store::new(&engine);

        let module = Module::from_file(&store, path.path())?;

        let imports = imports! {};
        let instance = Instance::new(&mut store, &module, &imports)?;
        let env = FunctionEnv::new(&mut store, GreeterData {});
        let greeter = Greeter::new(&mut store, &instance, env)?;

        let language = greeter.language(&mut store)?;
        println!("Language: {language}");
        let greeting = greeter.greet(&mut store, &args[1])?;
        println!("Greeting: {greeting}");
    }
    Ok(())
}
