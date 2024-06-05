use std::{io, u32};

use rand::Rng;

fn main() {
    let numero_secreto = rand::thread_rng()
        .gen_range(1, 100);
    
    println!("Adivinhe o número!");

    loop {
        let mut palpite = String::new();
        
        println!("Digite o seu palpite.");
        
        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");    

        let palpite: u32 = match palpite.trim()
        .parse() {
            Ok(numero) => numero,
            Err(_) => continue
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            std::cmp::Ordering::Greater => println!("MUITO ALTO!!!"),
            std::cmp::Ordering::Less => println!("MUITO BAIXO!!!"),
            std::cmp::Ordering::Equal => {
                println!("ACERTOU AMIGÃO!!!");
                break;            
            }
        }
    }
}
