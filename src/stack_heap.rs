// formas de armazenamento de dados

// Em Rust, como em muitas outras linguagens de programação, há duas regiões principais de memória que são usadas para armazenar dados enquanto o programa está sendo executado: a "pilha" (stack) e a "pilha dinâmica" ou "heap".

// A pilha é uma região de memória que é alocada automaticamente pelo sistema operacional quando o programa é iniciado. Ela é usada para armazenar variáveis ​​que são criadas localmente dentro de funções, e também é usada para armazenar o contexto de chamadas de função. A pilha é geralmente mais rápida para alocar e liberar do que a heap, mas ela tem um tamanho limitado e não pode ser redimensionada enquanto o programa está sendo executado.

// A heap, por outro lado, é uma região de memória que pode ser alocada e liberada dinamicamente enquanto o programa está sendo executado. Ela é usada para armazenar dados que podem ser acessados ​​globalmente ou que precisam persistir por um período de tempo mais longo do que a duração de uma única chamada de função. A heap é mais lenta para alocar e liberar do que a pilha, mas ela pode ser redimensionada enquanto o programa está sendo executado e não tem um tamanho fixo.

// literal type é armazenado na stack

/*
Ownership é um conceito-chave em Rust que se refere à maneira como a linguagem gerencia a memória. O objetivo do sistema de ownership é garantir que a memória seja usada de maneira segura e eficiente, sem ter que confiar em garbage collection ou em outros mecanismos de coleta de lixo.

Em Rust, cada valor tem um "dono", que é responsável por gerenciar a vida útil desse valor. Quando o dono de um valor sai de escopo, o valor é automaticamente liberado da memória. Isso significa que você não precisa se preocupar em gerenciar explicitamente a memória em Rust, o que pode ajudar a evitar erros comuns de memória, como buffer overflows e dangling pointers.



O sistema de ownership em Rust é baseado em três regras principais:



Cada valor tem um único dono, que é responsável por liberar a memória quando o valor não é mais necessário.

Quando um valor é atribuído a uma nova variável, o novo dono é responsável por liberar a memória quando o valor não é mais necessário.

Não é possível ter mais de um dono para um determinado valor ao mesmo tempo.

Essas regras são aplicadas pelo compilador Rust de forma a garantir que a memória seja gerenciada de maneira segura e eficiente. Por exemplo, se você tentar atribuir um valor para duas variáveis ​​diferentes ao mesmo tempo, o compilador emitirá um erro, pois isso violaria a regra 3 acima.
*/

pub fn stack_heap() {
   let x = 1;
   let y = x;// y possui um copy do valor de x

   println!("{}", y);
   println!("{}", x);

   let a = String::from("string a");
   let b = a.clone();

   println!("valor de a - {}", a);
   println!("valor de b clonado de a - {}", b);

   let c = String::from("string c");
   let d = &c;

   println!("valor de c - {}", c);
   println!("valor de d pegando como referencia o valor de c - {}", d);
}