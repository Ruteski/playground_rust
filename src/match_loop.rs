pub fn match_loop() {
   let mut s = Some(0);

   loop {
      match s {
         Some(i) => if i >= 20 {
            println!("Quit");
            s = None;
         } else {
            println!("{}", i);
            s = Some(i + 1);
         },
         _ => break,
      }
   }
}