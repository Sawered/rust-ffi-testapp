mod sub_module;

//use sub_module::submodule_func;

#[no_mangle]
pub extern "C" fn root_func(){
    println!("root_func:exec");
}

fn main() {
    println!("Hello, world!");
    unsafe {
        sub_module::run_shared_func();
        sub_module::run_imported_func();
    }
}
