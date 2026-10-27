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

//Convert it to hex 0x....
//Add "0" in the beginning of the hex number 0x0... 
//Add "7" in the end 
//Add three random characters (0 to 9, a to f) 
//Add three randomly selected characters in the end (0 to 9, a to f)
//Add 15 randomly characters in the end 
pub fn uuidv7_builder() -> String {
    format!("0{}7{}{}{}", timestamp(), three_random_characters(), 
        uuidv7_variant(), fifteen_random_characters())
}




//Display the variants: 
//(1) A string of low characters, no dashes
//(2) A string of capital letter characters, no dashes
//(3) A string of low characters with dashes 
//(4) A string of capital letter characters with dashes


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuidv7_variant_test() -> () {
        let h = uuidv7_variant();
        let eight = "8".to_string(); 
        let nine = "9".to_string(); 
        let ten = "a".to_string(); 
        let eleven = "b".to_string(); 
        match h {
            eight => (),
            nine => (), 
            ten => (), 
            eleven => (), 
            _               => panic!("Expected values are 8, 9, a, b but 
                                    got {}", h),
        }

    }
}
