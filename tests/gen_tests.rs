
use llvm_ir::Module;
use llvm_ir::ToLLVM;

use std::path::{Path, PathBuf};

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

const LL_DIR: &str = "tests/basic_bc/";

// Test against bitcode compiled with the same version of LLVM
fn llvm_ll_dir() -> PathBuf {
    if cfg!(feature = "llvm-8") {
        Path::new(LL_DIR).join("llvm8")
    } else if cfg!(feature = "llvm-9") {
        Path::new(LL_DIR).join("llvm9")
    } else if cfg!(feature = "llvm-10") {
        Path::new(LL_DIR).join("llvm10")
    } else if cfg!(feature = "llvm-11") {
        Path::new(LL_DIR).join("llvm11")
    } else if cfg!(feature = "llvm-12") {
        Path::new(LL_DIR).join("llvm12")
    } else if cfg!(feature = "llvm-13") {
        Path::new(LL_DIR).join("llvm13")
    } else if cfg!(feature = "llvm-14") {
        Path::new(LL_DIR).join("llvm14")
    } else {
        unimplemented!("new llvm version?")
    }
}

fn rust_ll_dir() -> PathBuf {
    Path::new(LL_DIR).join("rust")
}


#[test]
fn hello_gen() {
    init_logging();
    let path = llvm_ll_dir().join("hello.ll");
    let module = Module::from_ir_path(&path).expect("Failed to parse module");
}