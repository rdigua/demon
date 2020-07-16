pub struct DayStruct {
    date:u32,
    open:u32,
    high:u32,
    low:u32,
    close:u32,
    amount:u32,
    vol:u32,
    reservation:u32
}

impl DayStruct{
    fn from_u32(s:Vec<u32>)->Vec<DayStruct>{
        let l=s.len();
       // let d=DayStruct;
        let r:Vec<DayStruct>=Vec::new();
        if s%8 !=0 {
            return r;
        }
        for i in 0..l{
            if i%8==0&&i<l-7{
                let d=DayStruct{
                    date:a[i],
                    open:a[i+1],
                    high:a[i+2],
                    low:a[i+3],
                    close:a[i+4],
                    amount:a[i+5],
                    vol:a[i+6],
                    reservation:a[i+7]
                };
                re.push(d);
            }
        }
        r
    }
}

/*
impl DayStruct {
    fn save(&self){

    }
    fn from(&self){

    }
}

 */

pub struct Origin {
    from:String,
    place:String,
    filename:String,
    db:String,
}
