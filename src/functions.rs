///Functions of project.

pub fn vec_u8_u32(s: Vec<u8>) -> Option<Vec<u32>> {
    let mut r: Vec<u32> = Vec::new();
    //if s.len()==0 || !(s.len()%4==0)
    if s.is_empty() || s.len() % 4 != 0 {
        return None;
    }
    for i in 0..s.len() {
        if (i % 4 == 0) && (i + 4 < s.len()) {
            let a: u32 = u32::from_le_bytes([s[i], s[i + 1], s[i + 2], s[i + 3]]);
//let new32: u32 = u32::from_le_bytes([s[4*i], s[4*i + 1], s[4*i + 2], s[4*i + 3]]);
            r.push(a);
        }
    }
    if s.is_empty() {
        None
    } else {
        Some(r)
    }
}