fn main() {
    /*
        Em Rust, nao existe conversao automatica implicita
        (nao pode somar um i32 com um u8 diretamente).

        - Operador as: Forca a conversao (pode decepar bits se converter de um
        tipo maior para um menor)

        - Metodo try_into: Tenta converter e te avisa atraves de um Result se
        a conversao falharia ou se causaria perda de dados
     */

    let x: u32 = 1000;
    let y = x as u8;
    println!("Valor de y: {}", y);

    let z: Result<u8, _> = x.try_into();
    match z {
        Ok(num) => println!("Valor de num: {}", num),
        Err(e) => println!("Valor de num {} nao cabe em um u8. Erro {}", x, e),
    }
}