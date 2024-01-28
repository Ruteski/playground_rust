use std::collections::HashMap;

pub fn hasmap() {
   let mut hm = HashMap::new();

   hm.insert(String::from("random"), 12);
   hm.insert(String::from("strings"), 49);

   for (k,v) in &hm {
      println!("key: {}  valor: {}", k, v);
   }

   hm.remove(&String::from("random"));

   match hm.get(&String::from("random")) {
      Some(&n) => println!("{}", n),
      _ => println!("no match"),
   }

}