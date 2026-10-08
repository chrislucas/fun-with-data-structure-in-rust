/*
    https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    O if let em Rust é um atalho para um match que lida apenas com um
    padrao especifico e ignora o resto
 */

fn main() {
    let value: Option<i32> = Some(10);
    /*
        Em Rust, Some é uma variante do enum Option usado para representar
        que um valor está presente.

        A linguagem Rust nao usa null para valores ausentes. Em vez disso ela usa
        o tipo generico Option<T>

        O Option<T> tem apenas duas opcoses chamadas variantes
            - Some(T): Contem um valor valido do tipo T
            - None
     */
    if let Some(v) = value {
        println!("{}", v);
    } else {
        println!("None");
    }
}