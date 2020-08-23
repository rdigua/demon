trait Modify{
    fn vec8_vec32(v:Vec<u8>)->Vec<u32>{
        let mut result1:Vec<u32>=Vec::new();
        if (v.len() % 4) !=0{
            return result1
        }

        for i in 0..v.len()-1{
            if i==0 || ( i%4 ==0){
                let k:u32=u32::from_le_bytes([v[i],v[i+1],v[i+2],v[i+3]]);
                result1.push(k);
            }
        }
        result1
    }

}