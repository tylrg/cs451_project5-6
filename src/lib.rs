mod utils;
use std::str;


use wasm_bindgen::prelude::*;
use libsteg;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-ppm!");
}

pub fn log_value(input: &str){
    log(input);
}

#[wasm_bindgen]
pub fn image_passthrough(data: &[u8]) -> Vec<u8> {

    alert(&format!("image data: {:?}", data));
    let mut ret = Vec::new();
    ret.extend_from_slice(data);

    ret
}

#[wasm_bindgen]
pub fn manipulate_image_in_memory(input: &str,data: &[u8]) -> *const u8 {

    //need to find end of header
    
    // to start with we just want to pass through
    // so whatever we get passed in, we just want
    // to stick it in the wasm memory

    let mut ret = Vec::new();
    ret.extend_from_slice(data);

    // we have a 15 byte header
    // for our hardcoded expected
    // file uploaded dahlia-red-blossom-bloom-60597.ppm

    // let's try turning the entire image white.

    

    let mut start = 99999999; // this skips our hard coded header
    let mut header_bytes: Vec<u8> = Vec::new();
    let mut newline_count = 0;
    for i in 0..ret.len() {
        if newline_count == 3{
            start = i;
            break;
        }
        header_bytes.push(ret[i]);
        if header_bytes[header_bytes.len()-1] == 10 {
            //log_value("Found a newline");
            newline_count+=1;
        }
    }
    let header_message = str::from_utf8(&header_bytes).unwrap();
    //log_value("Header");
    //log_value(header_message);

    //log_value("Start");
    for _i in 0..start{
        //log_value(" ");
    }
    //log_value(start);

    let swag = encode_message(&input,ret.clone(),start);    

    let mut assembled: Vec<u8> = Vec::new();
    for h in header_bytes{
        //log_value("h");
        assembled.push(h);
    }

    for val in 0..swag.len(){
        ret[val+start] = swag[val];
    }

    ret.as_ptr()
}

#[wasm_bindgen]
pub fn decode_message_from_bytes(data: &[u8]) -> String{
    

    let mut data_bytes:Vec<u8> = Vec::new();
    let mut header_bytes: Vec<u8> = Vec::new();
    data_bytes.extend_from_slice(data);
    let mut newline_count= 0;
    let mut start = 0;
    for i in 0..data_bytes.len() {
        if newline_count == 3{
            start = i;
            break;
        }
        header_bytes.push(data_bytes[i]);
        if header_bytes[header_bytes.len()-1] == 10 {
            log_value("Found a newline");
            newline_count+=1;
        }
    }

    let mut to_decode_vector:Vec<u8> = Vec::new();
    for i in start..data_bytes.len(){
        to_decode_vector.push(data_bytes[i]);
    }


    let ret_val: String = decode_message(&to_decode_vector);
    log_value("Decoded Value: ");
    log_value(ret_val.as_str());

    return ret_val;
}

#[wasm_bindgen]
pub fn get_text(input: &str) -> String {
    String::from(input)
}

// #[wasm_bindgen]
// pub fn double(input: &str) -> String {
//     let base = String::from(input);
//     let base = format!("{}{}",base,base);
//     log_value("This is from rust VVVV");
//     log_value(&base[0..base.len()]);
//     return base;
// }

fn encode_message(message: &str,pixels: Vec<u8>,start: usize) -> Vec<u8> {
    let mut encoded = vec![0u8; 0];
    let mut start_index = start;


    for c in message.chars() {
        encoded.extend(&encode_character(c,&pixels[start_index..start_index + 8],));
        start_index += 8;
    }

    // we need to add a null character to signify end of
    // message in this encoded image
    encoded.extend(&encode_character('\0',&pixels[start_index..start_index + 8],
    ));

    start_index += 8;

    // spit out remainder of ppm pixel data.
    encoded.extend(&pixels[start_index..]);
    
    encoded
}
fn encode_character(c: char, bytes: &[u8]) -> [u8; 8] {
    let c = c as u8;
    log_value(str::from_utf8(&[c]).unwrap());
    let mut ret = [0u8; 8];

    for i in 0..bytes.len() {
        if bit_set_at(c, i) {
            ret[i] = bytes[i] | 00000_0001;
        } else {
            ret[i] = bytes[i] & 0b1111_1110;
        }
    }

    ret
}
fn bit_set_at(c: u8, position: usize) -> bool {
    bit_at(c, position) == 1
}
fn bit_at(c: u8, position: usize) -> u8 {
    (c >> (7 - position)) & 0b0000_0001
}




fn decode_message(pixels: &Vec<u8>) -> String {
    let mut message = String::from("");

    for bytes in pixels.chunks(8) {
        // eprintln!("chunk!");
        if bytes.len() < 8 {
            panic!("There were less than 8 bytes in chunk");
        }

        let character = decode_character(bytes);

        if character > 127 {
            // return Err(StegError::BadDecode(
            //     "Found non-ascii value in decoded character!".to_string(),
            // ));
            return String::from("ERROR IN DETERMINING MESSAGE: NON ASCII VALUE FOUND!");
        }

        message.push(char::from(character));

        if char::from(character) == '\0' {
            // eprintln!("Found terminating null!");
            break;
        }
    }

    message
}

fn decode_character(bytes: &[u8]) -> u8 {
    if bytes.len() != 8 {
        panic!("Tried to decode from less than 8 bytes!");
    }

    let mut character: u8 = 0b0000_0000;

    for (i, &byte) in bytes.iter().enumerate() {
        if lsb(byte) {
            match i {
                0 => character ^= 0b1000_0000,
                1 => character ^= 0b0100_0000,
                2 => character ^= 0b0010_0000,
                3 => character ^= 0b0001_0000,
                4 => character ^= 0b0000_1000,
                5 => character ^= 0b0000_0100,
                6 => character ^= 0b0000_0010,
                7 => character ^= 0b0000_0001,
                _ => panic!("uh oh!"),
            }
        }
    }

    character
}

fn lsb(byte: u8) -> bool {
    (0b0000_0001 & byte) == 1
}