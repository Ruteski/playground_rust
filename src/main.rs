fn main() {
    // println!("Hello, Ruteski! I like Rust!");
    // let mut x: i32 = 5; //u32 usize32 nao aceita valores negativos, i32 aceita valores negativos
    // x = -3;

    //inteiros: i8, u8, i16, u16, i32, u32, i64, u64 || i = size - u = usize
    //float : f32(single precision) 12.4, f64(dupla precisao) 12.45

    // let a = 1 + 20;
    // let s = 30 - 20;
    // let m = 5 * 10;
    // let d = 4 / 6;
    // let r = 49 % 2;

    // bool: true/false

    // char
    // let c = 'c';
    // let z: char = 'z';

    //tupla
    // let t: (i32, f64, char) = (42, 6.12, 'j');
    // let (z, y, x) = t; // destructuring - desestruturar
    // let (_, _, g) = t;
    //t.2; //buscar o valor de x, o 2 é o index na tupla

    //array
    // let arr = [1, 2, 3, 4];
    // let a1 = arr[2];

    //=== TUPLAS
    // let t = (1, 'a', false);
    // let _f = (2, t);

    // println!("{} - {} - {}", t.0, t.1, t.2);
    // println!("{:?}", t);
    // println!("{:?}", _f);
    // println!("{:#?}", _f);

    //=== ARRAY
    let xs: [i32; 5] = [5, 6, 2, 8, 7]; //xs: [i32;5] array de inteiros, com 5 elementos
    println!(
        "-> valor na posicao 4: {} \n-> tamanho do array: {} \n-> tamanho que ocupa em memoria: {} bits",
        xs[3],
        xs.len(),
        std::mem::size_of_val(&xs) // ou poderia ter importado use std::mem e utilizado > mem::size_of_val
    );
}
