use std::fmt;

#[derive(Debug)] //trait
struct Object {
   width: u32,
   heigh: u32,
}
//implementation block - metodos e funcoes
impl Object {
   fn area(&self) -> u32 {
      self.width * self.heigh  // para retornar o valor, ou usa o return ou deixa sem o ";" no final da linha onde vai ser o retorno
   }

   fn show(&self) {
      println!("{}x{} com área de {}", self.width, self.heigh, self.area());   
   }
}

//related
impl Object {
   //funcao relacionada/associada
   fn new(width: u32, heigh:u32) -> Object {
      Object {
         width,
         heigh,
      }
   }
}

impl fmt::Display for Object {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({} {})", self.width, self.heigh)
   }
}

pub fn metodos_funcoes_relacionadas() {
   let obj = Object {
      width: 35,
      heigh: 35,
   };

   let obj2 = Object::new(25, 39);

   //println!("{}x{} com área de {}", obj.width, obj.heigh, obj.area());
   //println!("{}x{} com área de {}", obj2.width, obj2.heigh, obj2.area());

   obj.show();
   obj2.show();
   println!("{:?}", obj);// pra poder printar assim o obj, foi adicionada a trait debug na struct
   println!("{}", obj);// pra poder usar assim, foi implementado o display para o objeto

}