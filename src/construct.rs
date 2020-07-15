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
