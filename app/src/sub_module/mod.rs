
#[no_mangle]
extern "C" fn submodule_func(){
    println!("submodule_func:exec");
}
extern "C"{
    pub fn run_shared_func();
    pub fn run_imported_func();
}