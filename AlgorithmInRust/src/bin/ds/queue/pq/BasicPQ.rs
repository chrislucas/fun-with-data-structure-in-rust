/*
    Método 2: Múltiplos Arquivos Executáveis (src/bin)

    Se você quer que cada arquivo seja um programa independente com sua própria função fn main(),
    use a pasta especial src/bin:
    
    1 - Crie uma pasta chamada bin dentro de src (src/bin/).
    2 - Adicione seu arquivo executável lá dentro, por exemplo, src/bin/outro_programa.rs:

    existe uma implementacao de pilha, fila e file de prioridade ?
    https://share.google/aimode/C9oOuxGsuQZOM7PCq

    - Fila de Prioridade (Priority Queue)
        - A biblioteca padrao do Rust possui uma implementacao nativa de File de
        prioridades chamada BinaryHeap<T>

        - Por padrao, a BinaryHeap funciona como uma MaxHeap
 */

use std::collections::BinaryHeap;


fn main() {
    /*
        Em Rust, let declara uma variavel imutavel por padrao, enquanto
        o let mut declara uma variavel mutavel

        let: O valor atribuido nao pode ser alterado apos a inicializacao.
        Tentar reatribuir um novo valor gera um erro de compilacao

        let mut: a palavra chave mut (de mutable) avisa o compolador que
        o valor da variavel podera ser modificado posteriiormente no codigo
     */

    let mut pq: BinaryHeap<i32> = BinaryHeap::new();
    pq.push(10);
    pq.push(40);
    pq.push(30);
    println!("Hello, world!");
}