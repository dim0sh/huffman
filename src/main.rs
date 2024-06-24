mod encrypt;
mod huffman;
use encrypt::{encrypt, decrypt};
use huffman::HuffmanTree;
use std::fs;
use bincode;
use std::{cmp::Reverse, iter::once_with};
fn main() {
    // let salt: Vec<u8> = vec![1, 2, 3, 4];


    // let original_file = fs::read_to_string("./bible.txt").unwrap();
    // let input = original_file.clone().into_bytes();
    // //needs to be even length or desalt will have off by one error in step calculation (sometimes)
    // let start = std::time::Instant::now();
    // let output = encrypt(input, salt.clone());

    // //

    // //

    // let duration = start.elapsed();
    // println!("Encrypt Duration: {:?}", duration);
    // let serialized_out = bincode::serialize(&output).unwrap();
    // fs::write("./bible_encrypted.txt", serialized_out).unwrap();
    
    // let start = std::time::Instant::now();
    // let output = decrypt(output, salt.clone());
    // let duration = start.elapsed();
    // println!("Decrypt Duration: {:?}", duration);
    // assert!(original_file == String::from_utf8(output).unwrap(), "Decrypted file does not match original file");

    // let input = fs::read("./bible_encrypted.txt").unwrap();
    // let deserialized_input:Vec<u8> = bincode::deserialize(&input).unwrap();
    // let start = std::time::Instant::now();
    // let output = decrypt(deserialized_input, salt);
    // let duration = start.elapsed();
    // println!("Decrypt Duration: {:?}", duration);
    // let comp = fs::read_to_string("./bible.txt").unwrap();
    // assert!(comp == String::from_utf8(output).unwrap(), "Decrypted file does not match original file");


    let start = std::time::Instant::now();
    let data = fs::read("./bible.txt").unwrap();
    
    let duration = start.elapsed();
    println!("Read Duration: {:?}", duration);

    let start = std::time::Instant::now();
    let tree = HuffmanTree::create_tree_arr(&data);
    let duration = start.elapsed();
    println!("Tree Creation Duration: {:?}", duration);

    let path = vec![false, false, true, true];

    
    let dict = tree.clone().create_dict();
    let reverse_dict = HuffmanTree::reverse_dict(&dict);
    

    // let test = HuffmanTree::reverse_dict_get(reverse_dict.clone(), path.clone());
    
    
    let start = std::time::Instant::now();
    (0..20000).into_iter().for_each(|_|{
        dict.iter().for_each(|x|{
            let test = HuffmanTree::reverse_dict_get(&reverse_dict, x);
        });
        
    });
    let duration = start.elapsed();
    println!("Duration dict: {:?}",duration);

    let res = dict.iter().map(|x|{
        HuffmanTree::reverse_dict_get(&reverse_dict, x)
    }).filter(|&x| x != 255).map(|x| x as char).collect::<String>();

    println!("{}", res);


    

    let test_decode = vec![false, false, true, true];
    let start = std::time::Instant::now();
    (0..20000).into_iter().for_each(|_|{
        dict.iter().for_each(|x|{
            let test = HuffmanTree::decode(&tree.root, x);
        });
    });
    let duration = start.elapsed();
    println!("Decode Duration: {:?}", duration);
    
}

