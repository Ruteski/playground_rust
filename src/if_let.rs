pub fn if_let() {
   let s = Some('c');

   match s {
      Some(i) => println!("{}", i),
      //_ => println!("no match"),
      _ => {},
   }
   // ou \/

   if let Some(i) = s {
      println!("{}", i)
   } else {
      {}
   }
}