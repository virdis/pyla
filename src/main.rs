use std::{borrow::BorrowMut, io};

use bytes::{Buf, BufMut};
use pyla::ingestion;
use ingestion::*;
fn main() {
    println!("Hello, world!");
    let p1 = PylaId::new(1);
    let p2 = PylaId::new(1);
    let p3 = PylaId::new(3); 

    println!("Test Comparison: {}", p1 > p2);
    println!("Test Comparison: {}", p2 == p3);
    println!("Test Comparison: {}", p3 == p3);
    let key = "key1";
    let value = "value1";
    let mut pentry1 = PylaEntry::new(key.as_bytes(), value.as_bytes());
   
    let mut entry_bytes = pentry1.underlying;
    
    let k1_size: usize = entry_bytes.get_i32() as usize;
    let value1_size: usize = entry_bytes.get_i32() as usize;

    //let mut k_b = entry_bytes.split_to(k1_size);
    let mut k_b = entry_bytes.slice(0..k1_size);
    let mut v_b = entry_bytes.slice(k1_size..);
    println!("Key Data: {:?}", k_b);
    println!("Value Data: {:?}", entry_bytes);
    println!("Value By Slice Data: {:?}", v_b);
    
    /*let value1_size: usize = entry_bytes.get_i32() as usize;
    let mut key_buffer: Vec<u8> = vec![];
    for i in 0..k1_size {
        key_buffer.push(entry_bytes.get_u8());
    }
    let mut value_buffer: Vec<u8> = vec![];
    for i in 0..value1_size {
        value_buffer.push(entry_bytes.get_u8());
    }
    println!("Key1 Size: {:?}", k1_size);
    println!("Value1 Size: {:?}", value1_size);
    println!("Bytes after Slice: {:?}", entry_bytes);
    println!("Key Bytes slice: {:?}", key_buffer);
    println!("Value Bytes slice: {:?}", value_buffer); */
    //let mut k1_buffer: Vec<u8> = Vec::with_capacity(k1_size as usize);
    //io::copy(&mut p1_reader, &mut k1_buffer);

    println!("Key Bytes : {:?} Value Bytes: {:?}", &key.as_bytes(), value.as_bytes());
    
}
