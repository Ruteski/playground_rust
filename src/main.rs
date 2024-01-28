//mod tupla;
//mod array;
//mod slice;
//mod stack_heap;
//mod ownership;
//mod funcao_com_ponteiro_e_referencia;
//mod structs;
//mod metodos_funcoes_relacionadas;
//mod condicionais_loops;
//mod matching;
//mod enums;
//mod options;
//mod vetores;
//mod hashmap;
//mod if_let;
//mod match_loop;
//mod while_let;
//mod enum_result;

struct Point {
   x: f64,
   y: f64,
}

impl Point {
   fn distance_from_origin(&self) -> f64 {
       (self.x.powi(2) + self.y.powi(2)).sqrt()
   }
}

fn main() {
   /* TIPOS DE VARIAVEIS
   println!("Hello, Ruteski! I like Rust!");
   let mut x: i32 = 5; //u32 usize32 nao aceita valores negativos, i32 aceita valores negativos
   x = -3;

   //inteiros: i8, u8, i16, u16, i32, u32, i64, u64 || i = size - u = usize
   //float : f32(single precision) 12.4, f64(dupla precisao) 12.45

   let a = 1 + 20;
   let s = 30 - 20;
   let m = 5 * 10;
   let d = 4 / 6;
   let r = 49 % 2;

   //bool: true/false

   //char
   let c = 'c';
   let z: char = 'z';
   */

   //tupla::tupla();    
   //array::array();
   //slice::slice();
   //stack_heap::stack_heap();
   //ownership::ownership();
   //funcao_com_ponteiro_e_referencia::func_ponteiro_referencia();
   //structs::structs();
   //metodos_funcoes_relacionadas::metodos_funcoes_relacionadas();
   //condicionais_loops::condicionais_loops();
   //matching::matching();
   //enums::enums();
   //options::optionss();
   //vetores::vetores();
   //hashmap::hasmap();
   //if_let::if_let();
   //match_loop::match_loop();
   //while_let::while_let();
   //enum_result::enum_result();

   let p = Point { x: 3.0, y: 4.0 };
   println!("A distância do ponto ({}, {}) da origem é {}", p.x, p.y, p.distance_from_origin());

   let x = 5;
   let y = 6;
   let z = {
       let a = 3;
       let b = 4;
       a + b
   };
   println!("O resultado da operação é: {}", x + y + z);

}
