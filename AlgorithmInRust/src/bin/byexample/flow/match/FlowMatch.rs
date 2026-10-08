/*
    https://doc.rust-lang.org/rust-by-example/flow_control/match.html

    Rust prove pattern matching via match keyword, que pode
    ser usado como switch/case como em C/Java

    O match é exaustivo, todos os valores precisam ser avaliados
 */


fn main() {
    let value = 21;
    println!("Sobre o valor: {}", value);

    match value {
        1 => print!("Um"),
        2 | 3 | 5 | 7 | 11 => println!("Valor: {}", value),
        13 ..=20 => println!("Valor : {}. Entre 13, incluse e 20", value),
        _ => println!("Nenhum dos valores"),
    }
}