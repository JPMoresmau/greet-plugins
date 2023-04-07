use std::{env, fs};

use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use wasmtime::*;

/// Greet using all the plugins.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: i18n-greeter <name>"));
    }
    let engine = Engine::default();
    let linker = Linker::new(&engine);

    let paths = fs::read_dir("./plugins").unwrap();

    for path in paths {
        let path = path?;
        let module = Module::from_file(&engine, path.path())?;
        let mut runtime = Runtime::new(&engine, &linker, &module)?;
        let language = runtime.language()?;
        println!("Language: {language}");
        let greeting = runtime.greet(&args[1])?;
        println!("Greeting: {greeting}");
    }
    Ok(())
}

/// Keep all necessary runtime information in one place.
struct Runtime {
    store: Store<()>,
    memory: Memory,
    /// Pointer to currently unused memory.
    pointer: usize,
    language: TypedFunc<i32, ()>,
    greet: TypedFunc<(i32, i32, i32), ()>,
}

impl Runtime {
    /// Create a new Runtime.
    fn new(engine: &Engine, linker: &Linker<()>, module: &Module) -> Result<Self> {
        let mut store = Store::new(engine, ());

        let instance = linker.instantiate(&mut store, module)?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(anyhow::format_err!("failed to find `memory` export"))?;
        let language = instance
            .get_func(&mut store, "language")
            .ok_or(anyhow::format_err!(
                "`language` was not an exported function"
            ))?
            .typed::<i32, (), _>(&store)?;
        let greet = instance
            .get_func(&mut store, "greet")
            .ok_or(anyhow::format_err!("`greet` was not an exported function"))?
            .typed::<(i32, i32, i32), (), _>(&store)?;

        Ok(Self {
            store,
            memory,
            pointer: 0,
            language,
            greet,
        })
    }

    /// Get a new pointer to store the given size in memory.
    /// Grows memory if needed.
    fn new_pointer(&mut self, size: usize) -> Result<i32> {
        let current = self.pointer;
        self.pointer += size;
        while self.pointer > self.memory.data_size(&self.store) {
            self.memory.grow(&mut self.store, 1)?;
        }
        Ok(current as i32)
    }

    /// Reset pointer, so memory can get overwritten.
    fn reset_pointer(&mut self) {
        self.pointer = 0;
    }

    /// Read string from memory.
    fn read_string(&self, offset: i32, length: i32) -> Result<String> {
        let mut contents = vec![0; length as usize];
        self.memory
            .read(&self.store, offset as usize, &mut contents)?;
        Ok(String::from_utf8(contents)?)
    }

    /// Read bounds from memory.
    fn read_bounds(&self, offset: i32) -> Result<(i32, i32)> {
        let mut buffer = [0u8; 8];
        self.memory
            .read(&self.store, offset as usize, &mut buffer)?;
        let start = (&buffer[0..4]).read_i32::<LittleEndian>()?;
        let length = (&buffer[4..]).read_i32::<LittleEndian>()?;
        Ok((start, length))
    }

    /// Write string into memory.
    fn write_string(&mut self, str: &str) -> Result<(i32, i32)> {
        let data = str.as_bytes();
        let offset = self.new_pointer(data.len())?;
        self.memory.write(&mut self.store, offset as usize, data)?;
        Ok((offset, str.len() as i32))
    }

    /// Call language function.
    fn language(&mut self) -> Result<String> {
        let offset = self.new_pointer(16)?;
        self.language.call(&mut self.store, offset)?;
        let (offset, length) = self.read_bounds(offset)?;
        let s = self.read_string(offset, length)?;
        self.reset_pointer();
        Ok(s)
    }

    /// Call greet function.
    fn greet(&mut self, name: &str) -> Result<String> {
        let offset = self.new_pointer(16)?;
        let (start, length) = self.write_string(name)?;
        self.greet.call(&mut self.store, (offset, start, length))?;
        let (offset, length) = self.read_bounds(offset)?;
        let s = self.read_string(offset, length)?;
        self.reset_pointer();
        Ok(s)
    }
}
