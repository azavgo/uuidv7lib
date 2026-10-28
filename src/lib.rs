//Specs of uuid v7
//(1)First 48 bits is the Epoch timestamp in milliseconds
//For example the Epoch timestamp is 1711441500000 (March 25, 2024), 
//converting to HEX 0x018f4a2b1234, so the first 48 bits 
//will be 018f4a2b-1234. "0" nust be added at the beginning otherwise it 
//won't be 8-character string but only a 7-character string
//(2) 7 has to be placed in the beginning of the 3rd group - 
//4 characters all together but 7 has to be in the beginning 
//because it identifies the UUID version as version 7. The rest of this 
//group, 3 characters are random 
//(3) In the beginning of the 4th group there must be characters 
//8, 9, a, or b. These suggest that this is RFC variant of UUID v7. The 
//rest of the characters in this 4-character group is random
//(4) The final 5th group has 12 random characters 

use time::Timestamp; 
use rand;
use std::sync::Arc; 

//Generate Epoch timestamp in milliseconds
pub fn timestamp() -> String {
    format!("{:x}", Timestamp::now().as_milliseconds())
}

pub fn three_random_characters() -> String {
    format!("{:x}{:x}{:x}", rand::random_range(0..=15), 
        rand::random_range(0..=15), rand::random_range(0..=15))
}

pub fn fifteen_random_characters() -> String {
    format!("{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", 
        rand::random_range(0..=15), 
        rand::random_range(0..=15), 
        rand::random_range(0..=15),
        rand::random_range(0..=15), 
        rand::random_range(0..=15), 
        rand::random_range(0..=15),
        rand::random_range(0..=15), 
        rand::random_range(0..=15), 
        rand::random_range(0..=15),
        rand::random_range(0..=15), 
        rand::random_range(0..=15), 
        rand::random_range(0..=15),
        rand::random_range(0..=15), 
        rand::random_range(0..=15), 
        rand::random_range(0..=15))
}

//Randomly select 8, 9, a or b
pub fn uuidv7_variant() -> String {
    format!("{:x}", rand::random_range(8..=11))
}

#[derive(Debug)]
pub struct UUIDv7 {
    uuidv7: Arc<String>,
}

impl UUIDv7 {

//Convert it to hex 0x....
//Add "0" in the beginning of the hex number 0x0... 
//Add "7" in the end 
//Add three random characters (0 to 9, a to f) 
//Add three randomly selected characters in the end (0 to 9, a to f)
//Add 15 randomly characters in the end 
    pub fn new() -> Self {
        Self {uuidv7: Arc::new(format!("0{}7{}{}{}", timestamp(), 
                three_random_characters(), uuidv7_variant(), 
                fifteen_random_characters()))
        }
    }

    pub fn uuidv7(&self) -> Arc<String> {
        Arc::clone(&self.uuidv7)
    }

//(1) A string of low characters, no dashes
    pub fn to_low(&self) -> Arc<String> {
        self.uuidv7()
    }

//(2) A string of capital letter characters, no dashes
    pub fn to_capital(&self) -> Arc<String> {
        Arc::new(self.uuidv7().to_uppercase())
    }

//(3) A string of low characters with dashes 
    pub fn to_low_dashes(&self) -> Arc<String> {
        let mut id = (*self.to_low()).clone();
        if let Some((index, _)) = id.char_indices().nth(8) {
            id.insert(index, '-');
        }

        if let Some((index, _)) = id.char_indices().nth(13) {
            id.insert(index, '-');
        }

        if let Some((index, _)) = id.char_indices().nth(18) {
            id.insert(index, '-');
        }

        if let Some((index, _)) = id.char_indices().nth(23) {
            id.insert(index, '-');
        }

        Arc::new(id)
    }

//(4) A string of capital letter characters with dashes
    pub fn to_capital_dashes(&self) -> Arc<String> {
        let mut id = (*self.to_capital()).clone();
        if let Some((index, _)) = id.char_indices().nth(8) {
            id.insert(index, '-');
        }

        if let Some((index, _)) = id.char_indices().nth(13) {
            id.insert(index, '-');
        }

        if let Some((index, _)) = id.char_indices().nth(18) {
            id.insert(index, '-');
        }

        if let Some((index, _)) = id.char_indices().nth(23) {
            id.insert(index, '-');
        }

        Arc::new(id)
    }
}
