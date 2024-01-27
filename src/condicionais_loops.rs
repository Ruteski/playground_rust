pub fn condicionais_loops() {
   let c = 25;

   let n = if c == 10 {
      50
   } else {
      65
   };

   println!("n = {}", n);

   let mut d = 0;

   loop {
      println!("valor de d: {}", d);
      d += 1;

      if d >= 10 {
         break;
      }
   }

   // //label no loop
   // 'a:loop {
   //    println!("loop a");
   //    'b:loop {
   //       println!("loop b");
   //       'c:loop {
   //          println!("loop c");
   //          break 'b;
   //       }
   //    }
   // }

   let mut n = loop {
      break 10;
   };

   println!("o valor de n é {}", n);

   while n <= 20 {
      println!("valor de n: {}", n);
      n += 1;
   }

   let g = vec![1,2,3,4,5,6];

   for i in g {
      println!("i: {}", i)
   }
}