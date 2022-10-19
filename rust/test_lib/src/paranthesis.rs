use std::collections::HashMap;

// In Go if a key does not exists in map map[key] 
// returns nil for that type, 0 for integer in our case
// Rust panics when a key does not exists, so emulate 
// that with unwrap_or
pub fn match_paranthesis(input: &str) -> bool
{
    if input.len() < 1 || input.len() > 104 {
        return false;
    }

    let mut temp: Vec<u8> = Vec::new();

    let pairs: HashMap<u8, u8> = [
        (40,41), (91,93), (123,125),
    ].into();

    for ch in input.bytes() {
        if temp.len() == 0 {
            temp.push(*pairs.get(&ch).unwrap_or(&0));
        } else {    
            let last = temp[temp.len() - 1];
            if ch == last {
                temp.pop();
            } else {
                temp.push(*pairs.get(&ch).unwrap_or(&0));
            }
        }
    }

    temp.len() == 0
}
