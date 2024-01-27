pub fn tupla() {
   // let t: (i32, f64, char) = (42, 6.12, 'j');
   // let (z, y, x) = t; // destructuring - desestruturar
   // let (_, _, g) = t;

   // println!("{}", t.2);//buscar o valor de x, o 2 é o index na tupla

   //=== TUPLAS
   let t = (1, 'a', false);
   let _f = (2, t);

   println!("{} - {} - {}", t.0, t.1, t.2);
   println!("{:?}", t);
   println!("{:?}", _f);
   println!("{:#?}", _f);
}