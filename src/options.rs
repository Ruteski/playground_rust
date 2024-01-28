// enum Option<T> {
//    Some(T),
//    None,
// }

fn divisao(x: f64, y:f64) -> Option<f64> {
   if y == 0.0  {
      None 
   } else {
      Some(x / y)
   }
}

pub fn optionss() {
   let res = divisao(5.0, 7.0);

   match res {
      Some(x) => println!("{:.7}", x),
      None => println!("nao pode dividir por zero")
   }
}