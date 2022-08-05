//! A Wasm module can be compiled with multiple compilers.
//!
//! This example illustrates how to use the LLVM compiler.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example compiler-llvm --release --features "llvm"
//! ```
//!
//! Ready?
//! 
//! 
/*
#include <stdio.h>

int test() {
  int n, i, *ptr, sum = 0;
  n = 100;

  ptr = (int*) malloc(n * sizeof(int));
 
  printf("Enter elements: ");
  for(i = 0; i < n; ++i) {
    *(ptr + i) = i;
    sum += *(ptr + i);
  }

  for(i = 0; i < n; ++i) {
    sum += *(ptr + i);
  }
  free(ptr);

  return sum;
}
*/


use wasmer::{imports, wat2wasm, Instance, Module, Store, Value, Val, Global, Memory, MemoryType};
use wasmer_compiler_llvm::LLVM;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
        r#"
        (module
          (type $t0 (func (result i32)))
          (type $t1 (func (param i32 i32) (result i32)))
          (type $t2 (func (param i32 i32 i32) (result i32)))
          (type $t3 (func (param i32) (result i32)))
          (type $t4 (func (param i32 i32 i32 i32 i32) (result i32)))
          (type $t5 (func (param i32)))
          (import "env" "__linear_memory" (memory $env.__linear_memory 1))
          (import "env" "__stack_pointer" (global $env.__stack_pointer (mut i32)))
          (import "env" "printf" (func $env.printf (type $t1)))
          (import "env" "fflush" (func $env.fflush (type $t3)))
          (import "env" "malloc" (func $env.malloc (type $t3)))
          (import "env" "free" (func $env.free (type $t5)))
          (func $f4 (type $t0) (result i32)
            (local $l0 i32)
            i32.const 0
            local.set $l0
            local.get $l0
            return)
          (func $f5 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            i32.const 0
            local.set $l5
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            local.get $l5
            return)
          (func $f6 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
            (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
            global.get $env.__stack_pointer
            local.set $l3
            i32.const 16
            local.set $l4
            local.get $l3
            local.get $l4
            i32.sub
            local.set $l5
            i32.const -1
            local.set $l6
            local.get $l5
            local.get $p0
            i32.store offset=12
            local.get $l5
            local.get $p1
            i32.store offset=8
            local.get $l5
            local.get $p2
            i32.store offset=4
            local.get $l6
            return)
          (func $f7 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            i32.const 0
            local.set $l5
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            local.get $l5
            return)
          (func $f8 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            i32.const 0
            local.set $l5
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            local.get $l5
            return)
          (func $f9 (type $t3) (param $p0 i32) (result i32)
            (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
            global.get $env.__stack_pointer
            local.set $l1
            i32.const 16
            local.set $l2
            local.get $l1
            local.get $l2
            i32.sub
            local.set $l3
            i32.const 0
            local.set $l4
            local.get $l3
            local.get $p0
            i32.store offset=12
            local.get $l4
            return)
          (func $f10 (type $t3) (param $p0 i32) (result i32)
            (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
            global.get $env.__stack_pointer
            local.set $l1
            i32.const 16
            local.set $l2
            local.get $l1
            local.get $l2
            i32.sub
            local.set $l3
            i32.const -1
            local.set $l4
            local.get $l3
            local.get $p0
            i32.store offset=12
            local.get $l4
            return)
          (func $f11 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            i32.const -1
            local.set $l5
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            local.get $l5
            return)
          (func $f12 (type $t0) (result i32)
            (local $l0 i32)
            i32.const 65536
            local.set $l0
            local.get $l0
            return)
          (func $f13 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
            (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
            global.get $env.__stack_pointer
            local.set $l3
            i32.const 16
            local.set $l4
            local.get $l3
            local.get $l4
            i32.sub
            local.set $l5
            i32.const 0
            local.set $l6
            local.get $l5
            local.get $p0
            i32.store offset=12
            local.get $l5
            local.get $p1
            i32.store offset=8
            local.get $l5
            local.get $p2
            i32.store offset=4
            local.get $l6
            return)
          (func $f14 (type $t4) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (result i32)
            (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
            global.get $env.__stack_pointer
            local.set $l5
            i32.const 32
            local.set $l6
            local.get $l5
            local.get $l6
            i32.sub
            local.set $l7
            i32.const 0
            local.set $l8
            local.get $l7
            local.get $p0
            i32.store offset=28
            local.get $l7
            local.get $p1
            i32.store offset=24
            local.get $l7
            local.get $p2
            i32.store offset=20
            local.get $l7
            local.get $p3
            i32.store offset=16
            local.get $l7
            local.get $p4
            i32.store offset=12
            local.get $l8
            return)
          (func $f15 (type $t0) (result i32)
            (local $l0 i32)
            i32.const 0
            local.set $l0
            local.get $l0
            return)
          (func $f16 (type $t0) (result i32)
            (local $l0 i32)
            i32.const 0
            local.set $l0
            local.get $l0
            return)
          (func $f17 (type $t0) (result i32)
            (local $l0 i32)
            i32.const 0
            local.set $l0
            local.get $l0
            return)
          (func $f18 (type $t0) (result i32)
            (local $l0 i32)
            i32.const 0
            local.set $l0
            local.get $l0
            return)
          (func $f19 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            i32.const 0
            local.set $l5
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            local.get $l5
            return)
          (func $f20 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            i32.const 0
            local.set $l5
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            local.get $l5
            return)
          (func $f21 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
            (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
            global.get $env.__stack_pointer
            local.set $l3
            i32.const 16
            local.set $l4
            local.get $l3
            local.get $l4
            i32.sub
            local.set $l5
            i32.const 0
            local.set $l6
            local.get $l5
            local.get $p0
            i32.store offset=12
            local.get $l5
            local.get $p1
            i32.store offset=8
            local.get $l5
            local.get $p2
            i32.store offset=4
            local.get $l6
            return)
          (func $f22 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            local.get $l4
            global.set $env.__stack_pointer
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            i32.const 1
            local.set $l5
            i32.const 0
            local.set $l6
            local.get $l5
            local.get $l6
            call $env.printf
            drop
            i32.const 0
            local.set $l7
            local.get $l7
            i32.load
            local.set $l8
            local.get $l8
            call $env.fflush
            drop
            unreachable
            unreachable)
          (func $f23 (type $t0) (result i32)
            (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
            i32.const 19
            local.set $l0
            i32.const 0
            local.set $l1
            local.get $l0
            local.get $l1
            call $env.printf
            drop
            i32.const 0
            local.set $l2
            i32.const 0
            local.set $l3
            local.get $l3
            i32.load
            local.set $l4
            local.get $l4
            call $env.fflush
            drop
            local.get $l2
            return)
          (func $f24 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
            (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32)
            global.get $env.__stack_pointer
            local.set $l2
            i32.const 16
            local.set $l3
            local.get $l2
            local.get $l3
            i32.sub
            local.set $l4
            local.get $l4
            global.set $env.__stack_pointer
            local.get $l4
            local.get $p0
            i32.store offset=12
            local.get $l4
            local.get $p1
            i32.store offset=8
            i32.const 38
            local.set $l5
            i32.const 0
            local.set $l6
            local.get $l5
            local.get $l6
            call $env.printf
            drop
            i32.const 0
            local.set $l7
            local.get $l7
            i32.load
            local.set $l8
            local.get $l8
            call $env.fflush
            drop
            unreachable
            unreachable)
          (func $f25 (type $t3) (param $p0 i32) (result i32)
            (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32)
            global.get $env.__stack_pointer
            local.set $l1
            i32.const 16
            local.set $l2
            local.get $l1
            local.get $l2
            i32.sub
            local.set $l3
            local.get $l3
            global.set $env.__stack_pointer
            local.get $l3
            local.get $p0
            i32.store offset=12
            i32.const 55
            local.set $l4
            i32.const 0
            local.set $l5
            local.get $l4
            local.get $l5
            call $env.printf
            drop
            i32.const 0
            local.set $l6
            local.get $l6
            i32.load
            local.set $l7
            local.get $l7
            call $env.fflush
            drop
            unreachable
            unreachable)
          (func $f26 (type $t0) (result i32)
            (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32) (local $l10 i32) (local $l11 i32) (local $l12 i32) (local $l13 i32) (local $l14 i32) (local $l15 i32) (local $l16 i32) (local $l17 i32) (local $l18 i32) (local $l19 i32) (local $l20 i32) (local $l21 i32) (local $l22 i32) (local $l23 i32) (local $l24 i32) (local $l25 i32) (local $l26 i32) (local $l27 i32) (local $l28 i32) (local $l29 i32) (local $l30 i32) (local $l31 i32) (local $l32 i32) (local $l33 i32) (local $l34 i32) (local $l35 i32) (local $l36 i32) (local $l37 i32) (local $l38 i32) (local $l39 i32) (local $l40 i32) (local $l41 i32) (local $l42 i32) (local $l43 i32) (local $l44 i32) (local $l45 i32) (local $l46 i32) (local $l47 i32) (local $l48 i32) (local $l49 i32) (local $l50 i32) (local $l51 i32) (local $l52 i32) (local $l53 i32) (local $l54 i32) (local $l55 i32) (local $l56 i32) (local $l57 i32) (local $l58 i32)
            global.get $env.__stack_pointer
            local.set $l0
            i32.const 16
            local.set $l1
            local.get $l0
            local.get $l1
            i32.sub
            local.set $l2
            local.get $l2
            global.set $env.__stack_pointer
            i32.const 100
            local.set $l3
            i32.const 0
            local.set $l4
            local.get $l2
            local.get $l4
            i32.store
            local.get $l2
            local.get $l3
            i32.store offset=12
            local.get $l2
            i32.load offset=12
            local.set $l5
            i32.const 2
            local.set $l6
            local.get $l5
            local.get $l6
            i32.shl
            local.set $l7
            local.get $l7
            call $env.malloc
            local.set $l8
            local.get $l2
            local.get $l8
            i32.store offset=4
            i32.const 74
            local.set $l9
            i32.const 0
            local.set $l10
            local.get $l9
            local.get $l10
            call $env.printf
            drop
            i32.const 0
            local.set $l11
            local.get $l2
            local.get $l11
            i32.store offset=8
            block $B0
              loop $L1
                local.get $l2
                i32.load offset=8
                local.set $l12
                local.get $l2
                i32.load offset=12
                local.set $l13
                local.get $l12
                local.set $l14
                local.get $l13
                local.set $l15
                local.get $l14
                local.get $l15
                i32.lt_s
                local.set $l16
                i32.const 1
                local.set $l17
                local.get $l16
                local.get $l17
                i32.and
                local.set $l18
                local.get $l18
                i32.eqz
                br_if $B0
                local.get $l2
                i32.load offset=8
                local.set $l19
                local.get $l2
                i32.load offset=4
                local.set $l20
                local.get $l2
                i32.load offset=8
                local.set $l21
                i32.const 2
                local.set $l22
                local.get $l21
                local.get $l22
                i32.shl
                local.set $l23
                local.get $l20
                local.get $l23
                i32.add
                local.set $l24
                local.get $l24
                local.get $l19
                i32.store
                local.get $l2
                i32.load offset=4
                local.set $l25
                local.get $l2
                i32.load offset=8
                local.set $l26
                i32.const 2
                local.set $l27
                local.get $l26
                local.get $l27
                i32.shl
                local.set $l28
                local.get $l25
                local.get $l28
                i32.add
                local.set $l29
                local.get $l29
                i32.load
                local.set $l30
                local.get $l2
                i32.load
                local.set $l31
                local.get $l31
                local.get $l30
                i32.add
                local.set $l32
                local.get $l2
                local.get $l32
                i32.store
                local.get $l2
                i32.load offset=8
                local.set $l33
                i32.const 1
                local.set $l34
                local.get $l33
                local.get $l34
                i32.add
                local.set $l35
                local.get $l2
                local.get $l35
                i32.store offset=8
                br $L1
              end
            end
            i32.const 0
            local.set $l36
            local.get $l2
            local.get $l36
            i32.store offset=8
            block $B2
              loop $L3
                local.get $l2
                i32.load offset=8
                local.set $l37
                local.get $l2
                i32.load offset=12
                local.set $l38
                local.get $l37
                local.set $l39
                local.get $l38
                local.set $l40
                local.get $l39
                local.get $l40
                i32.lt_s
                local.set $l41
                i32.const 1
                local.set $l42
                local.get $l41
                local.get $l42
                i32.and
                local.set $l43
                local.get $l43
                i32.eqz
                br_if $B2
                local.get $l2
                i32.load offset=4
                local.set $l44
                local.get $l2
                i32.load offset=8
                local.set $l45
                i32.const 2
                local.set $l46
                local.get $l45
                local.get $l46
                i32.shl
                local.set $l47
                local.get $l44
                local.get $l47
                i32.add
                local.set $l48
                local.get $l48
                i32.load
                local.set $l49
                local.get $l2
                i32.load
                local.set $l50
                local.get $l50
                local.get $l49
                i32.add
                local.set $l51
                local.get $l2
                local.get $l51
                i32.store
                local.get $l2
                i32.load offset=8
                local.set $l52
                i32.const 1
                local.set $l53
                local.get $l52
                local.get $l53
                i32.add
                local.set $l54
                local.get $l2
                local.get $l54
                i32.store offset=8
                br $L3
              end
            end
            local.get $l2
            i32.load offset=4
            local.set $l55
            local.get $l55
            call $env.free
            local.get $l2
            i32.load
            local.set $l56
            i32.const 16
            local.set $l57
            local.get $l2
            local.get $l57
            i32.add
            local.set $l58
            local.get $l58
            global.set $env.__stack_pointer
            local.get $l56
            return)
          (data $d0 (i32.const 0) "\00")
          (data $d1 (i32.const 1) "[WASIENV] dlopen\0a\00")
          (data $d2 (i32.const 19) "[WASIENV] dlerror\0a\00")
          (data $d3 (i32.const 38) "[WASIENV] dlsym\0a\00")
          (data $d4 (i32.const 55) "[WASIENV] dlclose\0a\00")
          (data $d5 (i32.const 74) "Enter elements: \00")
        (export "test_fn" (func $f26)))
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
    let g1 = Global::new(&store, Val::I64(0));
    // let g2 = Global::new(&store, Val::I64(0));
    // let g3 = Memory::new(&store, )
    let m = Memory::new(&store, MemoryType::new(1, None, false)).unwrap();


    let import_object = imports! {
        "dog" => {
            "happy" => g1,
        },
        "env" => {
          "__linear_memory" => m,
      },
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    let mem_loop = instance.exports.get_function("test_fn")?;

    println!("Calling `mem_loop` function...");
    // Let's call the `mem_loop` exported function. The parameters are a
    // slice of `Value`s. The results are a boxed slice of `Value`s.
    let results = mem_loop.call(&[])?;

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::I32(3)]);

    Ok(())
}

#[test]
#[cfg(feature = "llvm")]
fn test_compiler_llvm() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
