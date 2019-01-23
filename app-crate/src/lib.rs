#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod crate_sub_module;
//pub use crate_sub_module::*;


pub fn just_function(){
    println!("just function call");
}