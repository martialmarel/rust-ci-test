fn main() {
    println!("Hello, World !");
    println!("Bonjour, Monde !");
    println!("Hola, Mundo !");

    let version = env!("CARGO_PKG_VERSION");
    println!("\r\nVersion: {}", version);
}
