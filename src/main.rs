use std::io;


fn main() {
    println!("Bonvenon al la demonstrado de KLĜF (koloĝofo)");
    println!("Elektu opcionon");
    println!("[1] Aldoni opinion");
    println!("[2] Ŝercu opinion");
    println!("[3] Redaktu vian opinion");
    println!("[4] Forigu vian opinion");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Full buffer");
}
