#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
extern "C" fn crate_func (){
    println!("crate_func::exec");
}


pub fn just_function(){
    println!("just function call");
}