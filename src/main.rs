extern crate rand;
extern crate bip39;

fn main() {
	
	 use bip39::{Mnemonic, Language};
	
	 let mut rng = rand::thread_rng();
 let m = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();
}
