struct Object {
   width: u32,
   heigh: u32,
}

fn area(obj: &Object) -> u32 {
   obj.width * obj.heigh  // para retornar o valor, ou usa o return ou deixa sem o ";" no final da linha onde vai ser o retorno
}

pub fn structs() {
   let obj = Object {
      width: 35,
      heigh: 35,
   };

   println!("{}x{} com área de {}", obj.width, obj.heigh, area(&obj));
}