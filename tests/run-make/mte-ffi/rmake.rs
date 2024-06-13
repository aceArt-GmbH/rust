// Tests that MTE tags and values stored in the top byte of a pointer (TBI) are
// preserved across FFI boundaries (C <-> Rust).
// This test does not require MTE: whilst the test will use MTE if available, if it is not,
// arbitrary tag bits are set using TBI.

//@ only-aarch64
//@ only-linux
//@ only-gnu
//@ run-pass

use run_make_support::{cc, dynamic_lib_name, extra_c_flags, run, rustc, target};

fn main() {
    run_test("int");
    run_test("float");
    run_test("string");
    run_test("function");
}

fn run_test(variant: &str) {
    let flags = {
        let mut flags = extra_c_flags();
        flags.push("-march=armv8.5-a+memtag");
        flags
    };
    print!("{variant} test...");
    rustc()
        .input(format!("foo_{variant}.rs"))
        .target(target())
        .linker("aarch64-linux-gnu-gcc")
        .run();
    cc().input(format!("bar_{variant}.c"))
        .input(dynamic_lib_name("foo"))
        .out_exe("test")
        .args(&flags)
        .run();
    run("test");
    println!("\tpassed");
}
