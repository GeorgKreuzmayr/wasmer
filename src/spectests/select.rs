// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/select.wast
#![allow(
    warnings,
    dead_code
)]
use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::spectest_importobject;
use wabt::wat2wasm;


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (param i32 i32 i32) (result i32)))
      (type (;1;) (func (param i64 i64 i32) (result i64)))
      (type (;2;) (func (param f32 f32 i32) (result f32)))
      (type (;3;) (func (param f64 f64 i32) (result f64)))
      (type (;4;) (func (param i32) (result i32)))
      (type (;5;) (func))
      (func (;0;) (type 0) (param i32 i32 i32) (result i32)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;1;) (type 1) (param i64 i64 i32) (result i64)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;2;) (type 2) (param f32 f32 i32) (result f32)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;3;) (type 3) (param f64 f64 i32) (result f64)
        get_local 0
        get_local 1
        get_local 2
        select)
      (func (;4;) (type 4) (param i32) (result i32)
        unreachable
        i32.const 0
        get_local 0
        select)
      (func (;5;) (type 4) (param i32) (result i32)
        i32.const 0
        unreachable
        get_local 0
        select)
      (func (;6;) (type 5)
        unreachable
        select
        unreachable
        i32.const 0
        select
        unreachable
        i32.const 0
        i32.const 0
        select
        unreachable
        f32.const 0x0p+0 (;=0;)
        i32.const 0
        select
        unreachable)
      (export \"select_i32\" (func 0))
      (export \"select_i64\" (func 1))
      (export \"select_f32\" (func 2))
      (export \"select_f64\" (func 3))
      (export \"select_trap_l\" (func 4))
      (export \"select_trap_r\" (func 5))
      (export \"select_unreached\" (func 6)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

// Line 31
fn c1_l31_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c1_l31_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, i32, i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i32, 2 as i32, 1 as i32, &vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 32
fn c2_l32_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c2_l32_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, i32, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i64, 1 as i64, 1 as i32, &vm_context);
    assert_eq!(result, 2 as i64);
}

// Line 33
fn c3_l33_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c3_l33_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1.0 as f32, 2.0 as f32, 1 as i32, &vm_context);
    assert_eq!(result, 1.0 as f32);
}

// Line 34
fn c4_l34_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c4_l34_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1.0 as f64, 2.0 as f64, 1 as i32, &vm_context);
    assert_eq!(result, 1.0 as f64);
}

// Line 36
fn c5_l36_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c5_l36_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, i32, i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i32, 2 as i32, 0 as i32, &vm_context);
    assert_eq!(result, 2 as i32);
}

// Line 37
fn c6_l37_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c6_l37_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, i32, i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, 1 as i32, 0 as i32, &vm_context);
    assert_eq!(result, 1 as i32);
}

// Line 38
fn c7_l38_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c7_l38_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, i32, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i64, 1 as i64, -1 as i32, &vm_context);
    assert_eq!(result, 2 as i64);
}

// Line 39
fn c8_l39_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c8_l39_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, i64, i32, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i64, 1 as i64, -252645136 as i32, &vm_context);
    assert_eq!(result, 2 as i64);
}

// Line 41
fn c9_l41_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c9_l41_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f32::NAN, 1.0 as f32, 1 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f32::NAN).is_sign_positive());
}

// Line 42
fn c10_l42_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c10_l42_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f32::NAN, 1.0 as f32, 1 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f32::NAN).is_sign_positive());
}

// Line 43
fn c11_l43_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c11_l43_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f32::NAN, 1.0 as f32, 0 as i32, &vm_context);
    assert_eq!(result, 1.0 as f32);
}

// Line 44
fn c12_l44_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c12_l44_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f32::NAN, 1.0 as f32, 0 as i32, &vm_context);
    assert_eq!(result, 1.0 as f32);
}

// Line 45
fn c13_l45_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c13_l45_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f32, std::f32::NAN, 1 as i32, &vm_context);
    assert_eq!(result, 2.0 as f32);
}

// Line 46
fn c14_l46_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c14_l46_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f32, std::f32::NAN, 1 as i32, &vm_context);
    assert_eq!(result, 2.0 as f32);
}

// Line 47
fn c15_l47_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c15_l47_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f32, std::f32::NAN, 0 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f32::NAN).is_sign_positive());
}

// Line 48
fn c16_l48_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c16_l48_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, f32, i32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f32, std::f32::NAN, 0 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f32::NAN).is_sign_positive());
}

// Line 50
fn c17_l50_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c17_l50_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f64::NAN, 1.0 as f64, 1 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f64::NAN).is_sign_positive());
}

// Line 51
fn c18_l51_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c18_l51_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f64::NAN, 1.0 as f64, 1 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f64::NAN).is_sign_positive());
}

// Line 52
fn c19_l52_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c19_l52_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f64::NAN, 1.0 as f64, 0 as i32, &vm_context);
    assert_eq!(result, 1.0 as f64);
}

// Line 53
fn c20_l53_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c20_l53_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(std::f64::NAN, 1.0 as f64, 0 as i32, &vm_context);
    assert_eq!(result, 1.0 as f64);
}

// Line 54
fn c21_l54_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c21_l54_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f64, std::f64::NAN, 1 as i32, &vm_context);
    assert_eq!(result, 2.0 as f64);
}

// Line 55
fn c22_l55_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c22_l55_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f64, std::f64::NAN, 1 as i32, &vm_context);
    assert_eq!(result, 2.0 as f64);
}

// Line 56
fn c23_l56_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c23_l56_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f64, std::f64::NAN, 0 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f64::NAN).is_sign_positive());
}

// Line 57
fn c24_l57_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c24_l57_action_invoke");
    let func_index = match result_object.module.info.exports.get("select_f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, f64, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2.0 as f64, std::f64::NAN, 0 as i32, &vm_context);
    assert!(result.is_nan());
            assert_eq!(result.is_sign_positive(), (std::f64::NAN).is_sign_positive());
}

// Line 59

// Line 60

// Line 61

// Line 62

// Line 65
#[test]
fn c29_l65_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 0, 1, 1, 65, 1, 27, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    c1_l31_action_invoke(&result_object, &vm_context);
    c2_l32_action_invoke(&result_object, &vm_context);
    c3_l33_action_invoke(&result_object, &vm_context);
    c4_l34_action_invoke(&result_object, &vm_context);
    c5_l36_action_invoke(&result_object, &vm_context);
    c6_l37_action_invoke(&result_object, &vm_context);
    c7_l38_action_invoke(&result_object, &vm_context);
    c8_l39_action_invoke(&result_object, &vm_context);
    c9_l41_action_invoke(&result_object, &vm_context);
    c10_l42_action_invoke(&result_object, &vm_context);
    c11_l43_action_invoke(&result_object, &vm_context);
    c12_l44_action_invoke(&result_object, &vm_context);
    c13_l45_action_invoke(&result_object, &vm_context);
    c14_l46_action_invoke(&result_object, &vm_context);
    c15_l47_action_invoke(&result_object, &vm_context);
    c16_l48_action_invoke(&result_object, &vm_context);
    c17_l50_action_invoke(&result_object, &vm_context);
    c18_l51_action_invoke(&result_object, &vm_context);
    c19_l52_action_invoke(&result_object, &vm_context);
    c20_l53_action_invoke(&result_object, &vm_context);
    c21_l54_action_invoke(&result_object, &vm_context);
    c22_l55_action_invoke(&result_object, &vm_context);
    c23_l56_action_invoke(&result_object, &vm_context);
    c24_l57_action_invoke(&result_object, &vm_context);
}
