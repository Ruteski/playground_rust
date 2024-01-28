#[derive(Debug)]
enum Exemplo {
   Float(f64),
   Int(i32),
   Text(String),
}

pub fn vetores() {
   let x = vec![1,2,3,4]; // sintaxe de criacao com macro vec![]
   let mut v: Vec<i32> = Vec::new();// utilizando o metodo new do Vec para criar

   // v.push(5);
   // v.push(6);
   // v.push(7);
   // v.push(8);

   for i in &v {
      println!("i: {}", i);
   }

   //v.push(9);

   println!("v: {:?} | len: {} | capacidade: {}", &v, v.len(), v.capacity());
   println!("v: {:?}", v.pop());

   let r = vec![
      Exemplo::Int(100),
      Exemplo::Float(12.67),
      Exemplo::Text(String::from("hello")),
   ];

   println!("{:?}", &r);

}