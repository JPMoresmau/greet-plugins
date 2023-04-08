use std::{env, fs};

use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use wasmer::*;
use wasmer_compiler_llvm::LLVM;

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
        let module = Module::from_file(&engine, path.path())?;
        let mut runtime = Runtime::new(&engine, &module)?;
        let language = runtime.language()?;
        println!("Language: {language}");
        let greeting = runtime.greet(&args[1])?;
        println!("Greeting: {greeting}");
    }
    Ok(())
}

/// Keep all necessary runtime information in one place.
struct Runtime {
    store: Store,
    instance: Instance,
    /// Pointer to currently unused memory.
    pointer: usize,
    language: TypedFunction<i32, ()>,
    greet: TypedFunction<(i32, i32, i32), ()>,
}

impl Runtime {
    /// Create a new Runtime.
    fn new(engine: &Engine, module: &Module) -> Result<Self> {
        let mut store = Store::new(engine);
        let imports = imports! {};
        let instance = Instance::new(&mut store, module, &imports)?;
        let language = instance
            .exports
            .get_typed_function(&store, "language")
            .or(Err(anyhow::format_err!(
                "`language` was not an exported function"
            )))?;
        let greet = instance
            .exports
            .get_typed_function(&store, "greet")
            .or(Err(anyhow::format_err!(
                "`greet` was not an exported function"
            )))?;

        Ok(Self {
            store,
            instance,
            pointer: 0,
            language,
            greet,
        })
    }

    fn memory(&self) -> Result<&Memory> {
        self.instance
            .exports
            .get_memory("memory")
            .or(Err(anyhow::format_err!("failed to find `memory` export")))
    }

    /// Get a new pointer to store the given size in memory.
    /// Grows memory if needed.
    fn new_pointer(&mut self, size: usize) -> Result<i32> {
        let current = self.pointer;
        self.pointer += size;
        let memory = self
            .instance
            .exports
            .get_memory("memory")
            .or(Err(anyhow::format_err!("failed to find `memory` export")))?;
        while self.pointer > memory.view(&self.store).data_size().try_into()? {
            memory.grow(&mut self.store, 1)?;
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
        self.memory()?
            .view(&self.store)
            .read(offset as u64, &mut contents)?;
        Ok(String::from_utf8(contents)?)
    }

    /// Read bounds from memory.
    fn read_bounds(&self, offset: i32) -> Result<(i32, i32)> {
        let mut buffer = [0u8; 8];
        self.memory()?
            .view(&self.store)
            .read(offset as u64, &mut buffer)?;
        let start = (&buffer[0..4]).read_i32::<LittleEndian>()?;
        let length = (&buffer[4..]).read_i32::<LittleEndian>()?;
        Ok((start, length))
    }

    /// Write string into memory.
    fn write_string(&mut self, str: &str) -> Result<(i32, i32)> {
        let data = str.as_bytes();
        let offset = self.new_pointer(data.len())?;
        self.memory()?
            .view(&self.store)
            .write(offset as u64, data)?;
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
        self.greet.call(&mut self.store, offset, start, length)?;
        let (offset, length) = self.read_bounds(offset)?;
        let s = self.read_string(offset, length)?;
        self.reset_pointer();
        Ok(s)
    }
}
