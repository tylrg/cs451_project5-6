mod utils;
use std::str;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);//creates an alert from rust
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);//used for logging to the console from rust
}

#[wasm_bindgen]
pub fn greet(input: &str) {alert(input);}//creates an alert from rust
pub fn log_value(input: &str){log(input);}//logs to console from rust

#[wasm_bindgen]
pub fn image_passthrough(data: &[u8]) -> Vec<u8> {
    alert(&format!("image data: {:?}", data));
    let mut ret = Vec::new();
    ret.extend_from_slice(data); //not quite sure what this function does but add data to a vector?
    ret
}

#[wasm_bindgen]
pub fn manipulate_image_in_memory(input: &str,data: &[u8]) -> *const u8 {    
    // to start with we just want to pass through
    // so whatever we get passed in, we just want
    // to stick it in the wasm memory
    
    //vector of value to return
    let mut ret = Vec::new();
    ret.extend_from_slice(data);//add data to vector
    let input_length = input.clone().len();//determine length of input message
    if (input_length*8) > ret.len(){
        greet("Input length greater than file size!");//reject if file can't hold it
        return Vec::new().as_ptr();//return empty vector as pointer
    }

    if input_length ==0 {
        greet("No input message!");//if no message,return empty vector as pointer
        return Vec::new().as_ptr();
    }

    let mut start = 99999999; //no header should be greater than this for some reason
    let mut header_bytes: Vec<u8> = Vec::new();//store the header in this vector
    let mut newline_count = 0;//amount of newlines counted hack around when header ends and data begins
    for i in 0..ret.len() {
        if newline_count == 3{
            start = i;//if we have "passed" the header, set start of data to current index
            break;
        }

        header_bytes.push(ret[i]);//add value to header bytes
        
        if header_bytes[header_bytes.len()-1] == 10 {newline_count+=1;}//add to newline count if value is newline
    }

    if start == 99999999{
        greet("Invalid PPM Header");//if we didn't find a start, return invalid header and then return
        return Vec::new().as_ptr();
    }
    
    let encoded = encode_message(&input,ret.clone(),start);//encode the message using the start of the data and the return vectors data

    //set the values in return vector to the encoded message, keep rest of "old" data after message is added
    for val in 0..encoded.len(){ret[val+start] = encoded[val];}

    //return the pointer to the vector
    ret.as_ptr()
}

#[wasm_bindgen]
pub fn decode_message_from_bytes(data: &[u8]) -> String{
    
    let mut data_bytes:Vec<u8> = Vec::new();//vector for storing image data
    let mut header_bytes: Vec<u8> = Vec::new();//vector for storing header data
    data_bytes.extend_from_slice(data);//add data from image to vector
    let mut newline_count= 0;//set newlines to zero
    let mut start = 0;//start of 
    for i in 0..data_bytes.len() {
        if newline_count == 3{
            start = i;//if we have passed the header
            break;
        }
        header_bytes.push(data_bytes[i]);
        if header_bytes[header_bytes.len()-1] == 10 {
            //log_value("Found a newline");
            newline_count+=1;
        }
    }

    let mut to_decode_vector:Vec<u8> = Vec::new();//vector of pixel data
    for i in start..data_bytes.len(){to_decode_vector.push(data_bytes[i]);}//add pixel data only to new vector

    let ret_val: String = decode_message(&to_decode_vector);//decode the message using pixel data

    return ret_val;//return the message
}


//encode functions

//encodes a given string into data with a start value 
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
//returns a set of bytes given a character to encode, returns encoded bytes
fn encode_character(c: char, bytes: &[u8]) -> [u8; 8] {
    let c = c as u8;
    //log_value(str::from_utf8(&[c]).unwrap());
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
//helper functions
fn bit_set_at(c: u8, position: usize) -> bool {
    bit_at(c, position) == 1
}
fn bit_at(c: u8, position: usize) -> u8 {
    (c >> (7 - position)) & 0b0000_0001
}

//decode functions

//decodes a message from given pixel data
fn decode_message(pixels: &Vec<u8>) -> String {
    let mut message = String::from("");

    for bytes in pixels.chunks(8) {

        if bytes.len() < 8 {
            greet("There were less than 8 bytes in chunk");
            return String::from("ERROR");
        }

        let character = decode_character(bytes);

        //if we had an error in decode_character, return error here too
        if character == 1{
            return String::from("ERROR");
        }

        if character > 127 {
            greet("Found non-ascii value in decoded character!");
            return String::from("ERROR");
        }

        message.push(char::from(character));

        if char::from(character) == '\0' {
            break;
        }
    }

    message
}
//decodes a set of bytes to a given character
fn decode_character(bytes: &[u8]) -> u8 {
    if bytes.len() != 8 {
        greet("Tried to decode from less than 8 bytes!");
        return 1;//character that will not be used in ouput, therefore we can use it to flag bad input
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
//helper function
fn lsb(byte: u8) -> bool {
    (0b0000_0001 & byte) == 1
}