//* [#]  1) how to take command line arguments 
         // ? 1 so will be using this std::env::args
// will be having two arguments for a action and another for item    
// *[]  storing the data in structure based and using hashmaps 
use std::env;


fn main() {

    // let v1:Vec<String> = env::args().collect();
    // this will collet the arguments in a vector form 
    let mut first = env::args().nth(1)
        .expect("please specify action");
    let mut second = env::args().nth(2)
        .expect("please specify item");

    println!("{}", first);
    println!("{}", second);


}
