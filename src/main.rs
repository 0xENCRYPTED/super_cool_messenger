use std::str::FromStr;
use base64::Engine;
use crate::Mode::{Decrypt, Encrypt};

enum Mode {
    Encrypt,
    Decrypt
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Select mode\n1. Encrypt\n2. Decrypt");
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode)?;
    let mode = mode.trim().to_string();
    let mode = match u8::from_str(&mode)? {
        1 => Encrypt,
        2 => Decrypt,
        _ => panic!("Unknown mode inserted")
    };

    let key = match mode {
        Decrypt => {
            let mut private_self = String::new();
            println!("Insert your private key: ");
            std::io::stdin().read_line(&mut private_self)?;
            let private_self = base64::engine::general_purpose::STANDARD.decode(private_self.trim())?;

            private_self
        }
        Encrypt => {
            let mut public_other = String::new();
            println!("Insert other public key: ");
            std::io::stdin().read_line(&mut public_other)?;
            let public_other = base64::engine::general_purpose::STANDARD.decode(public_other.trim())?;

            public_other
        }
    };

    loop {
        println!("{}", match mode {
            Encrypt => {
                let mut message = String::new();
                println!("Enter message: ");
                std::io::stdin().read_line(&mut message)?;
                let message = message.trim().to_string();

                base64::engine::general_purpose::STANDARD.encode(ecies::encrypt(&key, message.as_bytes()).map_err(|_|"Unknown err")?)
            }
            Decrypt => {
                let mut message = String::new();
                println!("Enter encrypted message: ");
                std::io::stdin().read_line(&mut message)?;
                let message = base64::engine::general_purpose::STANDARD.decode(message.trim().to_string())?;

                String::from_utf8(ecies::decrypt(&key, &message).map_err(|_|"Unknown err")?)?
            }
        })
    }
}
