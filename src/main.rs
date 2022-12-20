//* [#]  1) how to take command line arguments 
         // ? 1 so will be using this std::env::args
// will be having two arguments for a action and another for item    
// *#]  storing the data in structure based and using hashmaps
         //? so we will be hashmaps use std::collections::HashMap
         //? and then we will define how we want your data to look like 
// *[] implement a method so when we save it it will write the contents in a db file 
use std::env;
use std::collections::HashMap;
use std::fs;



struct Todo{
    map:HashMap<String,bool>,
}


impl Todo {
    fn insert(&mut self,key:String){
        self.map.insert(key,true);
    }
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k,v) in self.map {
            let record = format!("{}\t{}\n",k,v);
            content.push_str(&record);
        }
        fs::write("db.txt", content);
        

    }
}



fn main() {

    // let v1:Vec<String> = env::args().collect();
    // this will collet the arguments in a vector form 
    let mut action = env::args().nth(1)
        .expect("please specify action");
    let mut item = env::args().nth(2)
        .expect("please specify item");


    let mut todo = Todo{
        map:HashMap::new(),
    };
    if action == "add" {
        todo.insert(item);
        match todo.save(){
            Ok(_) => println!("success"),
            Err(why) => println!("error {}", why),
                }
    }

    
    

   

}
