use std::fs::File;

//usado para checagem de erro
// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }

pub fn enum_result() {
   // let f = 24.67;
   // let i = f as u8;//casting de um float64 em um integer8
   // let c = i as char;

   // println!("valor de f: {}", f);
   // println!("valor de i: {}", i);
   // println!("valor de c: {}", c);

   let f = File::open("teste.txt");
   let s = match f {
      Ok(file) => file,
      Err(error) => {
         panic!("Houston, we have a problem... in opening a file: {}", error)//macro panic pra sair do programa
      }
   };
}