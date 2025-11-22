use std::ops::Index;
use std::str::FromStr;
use base64::Engine;
use crate::Mode::{Bisexual, Decrypt, Encrypt};

#[derive(Debug)]
enum Mode {
    Encrypt,
    Decrypt,
    Bisexual,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Select mode\n1. Encrypt\n2. Decrypt\n3. Bixesual");
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode)?;
    let mode = mode.trim().to_string();
    let mode = match u8::from_str(&mode)? {
        1 => Encrypt,
        2 => Decrypt,
        3 => Bisexual,
        _ => panic!("Unknown mode inserted")
    };

    println!("Mode selected : {mode:?}");



    let (self_priv, other_pub):(Option<Vec<u8>>, Option<Vec<u8>>) = match mode {
        Decrypt => {
            let mut private_self = String::new();
            println!("Insert your private key: ");
            std::io::stdin().read_line(&mut private_self)?;
            let private_self = base64::engine::general_purpose::STANDARD.decode(private_self.trim())?;

            println!("Accepted");

            (Some(private_self), None)
        }
        Encrypt => {
            let mut public_other = String::new();
            println!("Insert other public key: ");
            std::io::stdin().read_line(&mut public_other)?;
            let public_other = base64::engine::general_purpose::STANDARD.decode(public_other.trim())?;

            println!("Accepted");

            (None, Some(public_other))
        }
        Bisexual => {
            let mut private_self = String::new();
            println!("Insert your private key: ");
            std::io::stdin().read_line(&mut private_self)?;
            let private_self = base64::engine::general_purpose::STANDARD.decode(private_self.trim())?;

            println!("Accepted");

            let mut public_other = String::new();
            println!("Insert other public key: ");
            std::io::stdin().read_line(&mut public_other)?;
            let public_other = base64::engine::general_purpose::STANDARD.decode(public_other.trim())?;

            println!("Accepted");
            println!("You are in a bi-mode now.\nPrefix your message with '#' symbol to encrypt or '?' to decrypt");
            (Some(private_self), Some(public_other))
        }
    };

    loop {
        println!("{}", match mode {
            Encrypt => {
                match other_pub {
                    Some(ref other_pub) => {
                        let mut message = String::new();
                        println!("Enter message: ");
                        std::io::stdin().read_line(&mut message)?;
                        let message = message.trim().to_string();

                        base64::engine::general_purpose::STANDARD.encode(ecies::encrypt(other_pub, message.as_bytes()).unwrap())
                    }
                    None=>"ti eblan".to_string()
                }


            }
            Decrypt => {
                match self_priv {
                    Some(ref self_priv) => {
                        let mut message = String::new();
                        println!("Enter encrypted message: ");
                        std::io::stdin().read_line(&mut message)?;
                        let message = base64::engine::general_purpose::STANDARD.decode(message.trim().to_string())?;

                        String::from_utf8(ecies::decrypt(self_priv, &message).unwrap())?
                    }
                    None=>"ti eblan".to_string()
                }

            }

            Bisexual => {
                // VNEZAPNO STALO POHUI NA PROVERKI


                let mut message = String::new();
                std::io::stdin().read_line(&mut message)?;

                match message.chars().nth(0).unwrap() {
                    '?'=>{
                        let message = base64::engine::general_purpose::STANDARD.decode(message[1..].trim().to_string())?;
                        println!("Decrypted message:");
                        String::from_utf8(ecies::decrypt(&self_priv.clone().unwrap(), &message).unwrap())?

                    }
                    '#'=>{
                        println!("Encrypted message:");
                        base64::engine::general_purpose::STANDARD.encode(ecies::encrypt(&other_pub.clone().unwrap(), message[1..].as_bytes()).unwrap())
                    }
                    _=>{"ti eblan".to_string()}
                }
            }
        })
    }
}
