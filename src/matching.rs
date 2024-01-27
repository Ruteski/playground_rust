
//parecido com switch das outras linguagens
pub fn matching() {
   let x = 5;

   match x {
      1 => println!("um"),
      2 => println!("dois"),
      3 => println!("tres"),
      4 => println!("quatro"),
      5 => println!("cinco"),
      _ => println!("outros")
   }

   let n = 15;

   match n {
      1 => println!("um"),
      2|3|5|7|11 => println!("numero primo"),
      13..=19 => println!("a teen"),
      _ => println!("nao especial")
   }

   let tupla = (-5,-2);

   match tupla {
      (0,y) => println!("y: {}", y),
      (x, 0) => println!("x: {}", x),
      _ => println!("no match")
   }

   let tupla2 = (5,-5);

   //gards
   match tupla2 {
      (x, y) if x == y => println!("iguais"),
      (x, y) if x + y == 0 => println!("soma é igual a zero"),
      (x, _) if x % 2 == 0 => println!("x é par"),
      _ => println!("no match")
   }

   //bound - atribuir valor a variavel sem ownership
   let p = 5;

   match p {
      z @ 1..=12 => println!("z: {}", z) ,
      z @ 13..=19 => println!("z: {}", z) ,
      _ => println!("no match")
   }

   let k = 8;

   let s = match k {
      s @ 1..=12 => s,
      s @ 13..=19 =>s,
      _ => 0
   };

   println!("valor de s: {}", s);
}