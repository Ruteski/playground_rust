fn burrow1(v: &Vec<i32>) {
   println!("{}", (*v)[10] + (*v)[15]);
}
fn burrow2(v: &Vec<i32>) {
   println!("{}", v[0] + v[998]);
}

pub fn func_ponteiro_referencia() {
   let mut v = Vec::new();

   for i in 1..1000 {
      v.push(i);
   }

   burrow1(&v);
   burrow2(&v);
   println!("{:?}", v);
}