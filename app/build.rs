fn main() {
    println!("cargo:rustc-link-lib=dylib=clib_shared");
//    println!("cargo:rustc-link-lib=static=app_crate");
    println!("cargo:rustc-link-search=native=../../shared_ffi_test/clib-shared/cmake-build-debug/")
}
