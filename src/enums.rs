#![allow(dead_code)] // remove o warning do arquivo inteiro


#[derive(Debug)]
struct Point {
   x: u32,
   y: u32,
}

#[derive(Debug)]
enum Direction {
   // (x, y)
   /*Up(u32, u32),
   Down(u32, u32),
   Left(u32, u32),
   Right(u32, u32),*/

   // ou assim \/

   Up(Point),//up é uma tupla do tipo Point
   Down(Point),
   Left(Point),
   Right(Point),
}

#[derive(Debug)]
enum Keys {
   UpKey(String),
   DownKey(String),
   LeftKey(String),
   RightKey(String),
}

impl Keys {
   fn destruct(&self) -> &String {
      match *self {
         Keys::UpKey(ref s) => s,
         Keys::DownKey(ref s) => s,
         Keys::LeftKey(ref s) => s,
         Keys::RightKey(ref s) => s,
      }
   }
}

impl Direction {
   fn match_direction(&self) -> Keys {
      match *self {
         Direction::Up(_) => Keys::UpKey(String::from("pressionou W")),
         Direction::Down(_) => Keys::DownKey(String::from("pressionou S")),
         Direction::Left(_) => Keys::LeftKey(String::from("pressionou A")),
         Direction::Right(_) => Keys::RightKey(String::from("pressionou D")),
      }
   }
}

enum Shape {
   Retangulo {width: u32, height: u32},
   Quadrado (u32),
   Circulo (f64),
}

impl Shape {
   fn area(&self) -> f64 {
      match *self {
          Shape::Retangulo { width, height } => (width * height) as f64,
          Shape::Quadrado ( ref s ) => (s * s) as f64,
          Shape::Circulo ( ref r ) => 3.14 * (r * r),
      }
   }
}

pub fn enums() {

   let u = Direction::Up(Point { x: 0, y: 1 });
   let k = u.match_direction();
   let x = k.destruct();
   println!("k(tipo de dado de k): {:?}", k);
   println!("x: {:?}", x);

   // let d = Direction::Down(Point { x: 0, y: -1 });
   // let l = Direction::Left(Point { x: -1, y: 0 });
   // let r = Direction::Right(Point { x: 1, y: 0 });

   let u = 10;
   let v = &u;
   let ref z = u;

   println!("{}", z == v);

   let r = Shape::Retangulo { width: 10, height: 7 };
   let q = Shape::Quadrado(10);
   let c = Shape::Circulo(4.5);

   let ar = r.area();
   let aq = q.area();
   let ac = c.area();

   println!("area do retangulo: {}", ar);
   println!("area do quadrado: {}", aq);
   println!("area do circulo: {}", ac);
}
