// fn take(v: Vec<i32>) {
//    println!("We took v: {}", v[10] + v[100]);
// }

fn add(a: i32, b: i32) {
   println!("soma de a + b = {}", a + b);
}

fn ret(v: Vec<i32>) -> Vec<i32> {
   println!("We took v: {}", v[10] + v[100]);
   v
}

pub fn ownership() {
/*   let mut v = Vec::new();

   for i in 1..1000 {
      v.push(i);
   }

   take(v);// tecnica de moving, movendo o ownership de v para a funcao take. String e Vector nao possui copy trait

   println!("{:?}", v);
   */

  // movendo valores que possuem copy trait, o exemplo acima Vec nao possui 
  let a = 32;
  let b = 45;
  add(a, b);
  println!("valor de a: {} - valor de b: {}", a, b);

   // retornando o ownership para a funcao main/ownership
   let mut v = Vec::new();

   for i in 1..1000 {
      v.push(i);
   }

   // transfiro o ownership da funcao ownership para a ret e depois retorno ele para a ownership
   v = ret(v);// retorno o ownership da funcao ret para a ownership

   println!("{:?}", v);   
}