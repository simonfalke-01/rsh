use std::io;

fn main() {
    let pi: f32 = 3.141592;
    let mut input = String::new();
    println!("{pi:.5}", pi = pi);
    println!("Pls type smth");
    io::stdin().read_line(&mut input).expect("Broe we have a issue contact local indian tech support");
    println!("{input}");
    println!("Dumbass this is not finished");
}
