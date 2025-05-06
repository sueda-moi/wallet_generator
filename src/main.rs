use solana_sdk::signature::{Keypair, Signer};
use bs58;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};


#[derive(Serialize, Deserialize)]
struct Wallet {
    public_key: String,
    secret_key: String, // Base58 encoded
}


fn load_wallet(path: &str) -> Keypair {

    // read the wallet file
    let mut file = File::open(path).expect("could not open file");
    let mut data  = String ::new();
    file.read_to_string(&mut data).expect(msg("could not read file"));

    // deserialize the wallet
    let wallet: Wallet = serde_json::from_str(&data).expect(msg("could not deserialize wallet"));

    //baseS58 -> bytes -> Keypair
    let secret_key = bsS8::decode(wallet.secret_key).into_vec().expect("could not decode secret key");

    Keypair::from_bytes(&secret_bytes).expect(msg("could not create keypair from bytes"))
}


fn main() {
    // // Generate a new keypair
    // let keypair = Keypair::new();

    // let public_key = keypair.pubkey().to_string();
    // let secret_key = bs58:: encode(keypair.to_bytes()).into_string(); 

    // let wallet = Wallet {
    //     public_key,
    //     secret_key,
    // };

    // //save the wallet to a file
    // let json = serde_json :: to_string_pretty(&wallet).expect("serialization failed");
    // let mut file = File :: create("wallet.json").expect("could not create file");
    // file.write_all(json.as_bytes()).expect("could not write to file");


    // println!("you have generated a new keypair");
    // println!("Public Key: {}", wallet.public_key);
    // println!("Secret Key: {}", wallet.secret_key);
    // println!("Wallet saved to wallet.json");
    // println!("You can use this wallet to interact with the Solana blockchain.");
    // println!("Keep your secret key safe and never share it with anyone.");

    let keypair = load_wallet("wallet.json");
    println!("Public Key: {}", keypair.pubkey());
    println!("load wallet successfully");
}
