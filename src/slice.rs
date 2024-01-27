pub fn slice() {
   let xs:[i32; 5] = [4,5,6,7,8];
   let ys = &xs[2..4];

   println!("{} - {}", ys[0], ys[1]);
   println!("{:?} - {:?}", xs, ys);// {:?} debbuging flag

   let s = "slice string"; // tipo: slice string
   let ss = String::from("string"); // tipo: string
   let sss = "conversao em string".to_string(); // converte um slice string pra string

   let slice = &ss[1..5]; // pegando um pedaço da string, um slice da string

   println!("{}", s);
   println!("{}", ss);
   println!("{}", sss);
   println!("slice da string - {}", slice);

   let h = String::from("Hello, ");
   let w = String::from("World!");
   let s = h + &w;

   println!("{}", s);

}