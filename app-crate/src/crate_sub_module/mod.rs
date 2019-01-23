#[no_mangle]
pub extern "C" fn crate_func (){
    println!("crate_func::exec");
}

pub fn inner()
{
    println!("crate_func::inner");

}
