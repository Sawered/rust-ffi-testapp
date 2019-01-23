mod sub_module;
extern crate app_crate;
//use sub_module::submodule_func;

#[no_mangle]
pub extern "C" fn root_func(){
    println!("root_func:exec");
}

fn main() {
    println!("Hello, world!");
    app_crate::crate_sub_module::inner();
    app_crate::just_function();
    unsafe {
        sub_module::run_shared_func();
        sub_module::run_imported_func();
    }
}
