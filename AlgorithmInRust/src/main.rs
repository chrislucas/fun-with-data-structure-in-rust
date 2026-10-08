use std::mem::size_of_val;

fn main() {
    let character: char = 'c';
    let integer: i32 = 10;
    let boolean: bool = false;

    println!("Size of character: {} bytes", size_of_val(&character));
    println!("Size of integer: {} bytes", size_of_val(&integer));
    println!("Size of boolean: {} bytes", size_of_val(&boolean));

    println!("Maior i8 possivel: {}", i8::MAX);
    println!("Menor i8 possivel: {}", i8::MIN);
    println!("Maior i16 possivel: {}", i16::MAX);
    println!("Menor i16 possivel: {}", i16::MIN);
}
