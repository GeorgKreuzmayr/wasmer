//! A Wasm module can be compiled with multiple compilers.
//!
//! This example illustrates how to use the LLVM compiler.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example compiler-llvm-memory --release --features "llvm"
//! ```
//!
//! Ready?

use std::mem;
use wasmer::{imports, wat2wasm, Bytes, Instance, Module, NativeFunc, Pages, Store};
use wasmer_compiler_llvm::LLVM;
use wasmer_engine_universal::Universal;
use inkwell::context::Context;

use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
      r#"
(module
(type $mem_size_t (func (result i32)))
(type $get_at_t (func (param i32) (result i32)))
(type $set_at_t (func (param i32) (param i32)))
(type $t4 (func (param i32)))
(type $access_smth_t (func (result i32)))

(func $BM_memory_loop_std::__2::vector<int__std::__2::allocator<int>_>&_ (type $t4) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    local.get $l1
    i32.const 16
    i32.sub
    local.set $l1
    i32.const 0
    local.set $l2
    i32.const 0
    local.set $l3
    loop $L0
      local.get $l2
      local.set $l4
      local.get $l3
      local.set $l5
      loop $L1
        local.get $l1
        local.get $p0
        i32.load
        local.get $l4
        i32.add
        i32.load
        i32.const 10
        i32.add
        i32.store offset=12
        local.get $l4
        i32.const 400000
        i32.add
        local.set $l4
        local.get $l1
        i32.const 12
        i32.add
        local.set $l6
        local.get $l5
        i32.const 100000
        i32.add
        local.tee $l5
        i32.const 50000000
        i32.lt_u
        br_if $L1
      end
      local.get $l2
      i32.const 4
      i32.add
      local.set $l2
      local.get $l3
      i32.const 1
      i32.add
      local.tee $l3
      i32.const 100000
      i32.ne
      br_if $L0
    end)

(memory $mem 1)

(func $get_at (type $get_at_t) (param $idx i32) (result i32)
  (i32.load (local.get $idx)))

(func $set_at (type $set_at_t) (param $idx i32) (param $val i32)
  (i32.store (local.get $idx) (local.get $val)))

(func $mem_size (type $mem_size_t) (result i32)
  (memory.size))

(func $access_smth (type $access_smth_t) (result i32)
  (i32.load (i32.const 4)))

(export "get_at" (func $get_at))
(export "set_at" (func $set_at))
(export "mem_size" (func $mem_size))
(export "memory" (memory $mem))
(export "access_smth" (func $access_smth)))
"#
      .as_bytes(),
  )?;


    // Use LLVM compiler with the default settings
    let compiler = LLVM::default();

    // Create the store
    let store = Store::new(&Universal::new(compiler).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    // The module exports some utility functions, let's get them.
    //
    // These function will be used later in this example.
    let mem_size: NativeFunc<(), i32> = instance.exports.get_native_function("mem_size")?;
    let get_at: NativeFunc<i32, i32> = instance.exports.get_native_function("get_at")?;
    let set_at: NativeFunc<(i32, i32), ()> = instance.exports.get_native_function("set_at")?;
    let access_smth: NativeFunc<(), i32> = instance.exports.get_native_function("access_smth")?;
    let memory = instance.exports.get_memory("memory")?;

    // We now have an instance ready to be used.
    //
    // We will start by querying the most intersting information
    // about the memory: its size. There are mainly two ways of getting
    // this:
    // * the size as a number of `Page`s
    // * the size as a number of bytes
    //
    // The size in bytes can be found either by querying its pages or by
    // querying the memory directly.
    println!("Querying memory size...");
    assert_eq!(memory.size(), Pages::from(1));
    assert_eq!(memory.size().bytes(), Bytes::from(65536 as usize));
    assert_eq!(memory.data_size(), 65536);

    // Sometimes, the guest module may also export a function to let you
    // query the memory. Here we have a `mem_size` function, let's try it:
    let result = mem_size.call()?;
    println!("Memory size: {:?}", result);
    assert_eq!(Pages::from(result as u32), memory.size());

    // Now that we know the size of our memory, it's time to see how wa
    // can change this.
    //
    // A memory can be grown to allow storing more things into it. Let's
    // see how we can do that:
    println!("Growing memory...");
    // Here we are requesting two more pages for our memory.
    memory.grow(2)?;
    assert_eq!(memory.size(), Pages::from(3));
    assert_eq!(memory.data_size(), 65536 * 3);
    set_at.call(4, 10)?;
    // set_at.call(16, 10)?;

    let r = get_at.call(memory.data_size() as i32 / 4 + 1)?;
    println!("Value read: {:?}", r);
    let r2 = access_smth.call()?;
    println!("Value read by acc: {:?}", r2);

    Ok(())
}

#[test]
#[cfg(feature = "llvm")]
fn test_compiler_llvm_memory() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
