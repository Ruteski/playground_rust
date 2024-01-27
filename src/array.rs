pub fn array() {
   //array
   // let arr = [1, 2, 3, 4];
   // let a1 = arr[2];

   //=== ARRAY
   let xs: [i32; 5] = [5, 6, 2, 8, 7]; //xs: [i32;5] array de inteiros, com 5 elementos
   println!(
      "-> valor na posicao 4: {} \n-> tamanho do array: {} \n-> tamanho que ocupa em memoria: {} bits",
      xs[3],
      xs.len(),
      std::mem::size_of_val(&xs) // ou poderia ter importado use std::mem e utilizado > mem::size_of_val,
   );
}