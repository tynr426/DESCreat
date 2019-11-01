
extern crate des;
use des::Des;
use des::block_cipher_trait::{BlockCipher,generic_array::GenericArray};
//use des::{decrypt, encrypt};
fn main() {
    let key: [u8; 8] = [115, 104, 97, 105, 112, 101, 120, 112];
     let  msg_array ="你哈".as_bytes();
   // let mut message = GenericArray::from("你哈".as_bytes());
     
    let g_key= GenericArray::from(key);
     let instance=Des::new(&g_key);
    let mut cipher = instance.encrypt(&msg_array);
    //let message = instance.decrypt(cipher);
    // println!("Hello, world!");
}

fn des003() {
    // let key: [u8; 8] = [115, 104, 97, 105, 112, 101, 120, 112];
    // let message = "你哈".as_bytes();
    // let cipher = encrypt(&message, &key);
    // let message1 = decrypt(&cipher, &key);
    // println!("{:?},{:?},{:?}!", message, cipher, message1);
    // for byte in message1 {
    //     print!("{:x}", byte);
    // }
}