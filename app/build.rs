use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    //https://doc.rust-lang.org/cargo/reference/environment-variables.html
//    for (x,y) in env::vars(){
//        println!("{:?} {:?}",x,y);
//    }
    let ld_path = env::var("LD_LIBRARY_PATH").unwrap();

    println!("cargo:rustc-link-lib=dylib=clib_shared");
    println!("cargo:rustc-link-search=native=../../shared_ffi_test/clib-shared/cmake-build-debug/");
//    println!("cargo:rustc-link-lib=dylib=app_crate");
//    println!("cargo:rustc-link-search=native=./,{}",
//             ld_path.replace(":",",")
//    );
}
