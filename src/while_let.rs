pub fn while_let() {
   let mut s = Some(0);

   while let Some(i) = s {
      if i > 20 {
         println!("Quit");
         s = None;
      } else {
         println!("{}", i);
         s = Some(i + 1);
      }
   }
}