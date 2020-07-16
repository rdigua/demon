pub mod functions;
//use functions;

trait Turn {
	fn u832tou32(source: Vec<u8>) -> Result<Vec<u32>, &'static str>;
	fn u32to_dateline(source:Vec<u32>) -> Result<Vec<DayStruct>, &'static str>;
}



struct StockByte {
	stream_u32: Vec<u32>,
}

struct DayStruct
{
	date:u32,
	open:u32,
	high:u32,
	low:u32,
	close:u32,
	amount:u32,
	vol:u32,
	reservation:u32
}

impl Turn for DayStruct {
	fn u832tou32(source: Vec<u8>) -> Result<Vec<u32>, &'static str> {
		functions::u8_to_u32_result(source)
	}
	fn u32to_dateline(source:Vec<u32>) -> Result<Vec<DayStruct>, &'static str>{
		let mut re:Vec<DayStruct>=Vec::new();
		let mut a:[u32;8]=[0,0,0,0,0,0,0,0];
		if source.len() % 8 != 0 || source.is_empty() {
			return Err("It is empty or format error!");
		}
		for (i,n) in source.iter().enumerate(){
			a[(i%8) as usize]= *n;
			if i % 8 ==0&& i!=0{
				let r=DayStruct{
					date:a[0],
					open:a[1],
					high:a[2],
					low:a[3],
					close:a[4],
					amount:a[5],
					vol:a[6],
					reservation:a[7]
				};
				re.push(r);
			}

		}
		let r=DayStruct{
			date:a[0],
			open:a[1],
			high:a[2],
			low:a[3],
			close:a[4],
			amount:a[5],
			vol:a[6],
			reservation:a[7]
		};
		re.push(r);
		Ok(re)
	}
}
		/*
		let mut number: usize = 0;
		let mut a_u8: [u8; 4] = [0, 0, 0, 0];
		let mut return_u32: Vec<u32> = Vec::new();
		if source.len()==0  {
			return Err("Source is empty.");
		}
		if (source.len()%4) != 0 {
			return Err("Format errors.");
		}
		for _i in source.iter() {
			if number % 4 == 0 {
				a_u8[0] = *source.get(number).unwrap();
				a_u8[1] = *source.get(number+1).unwrap();
				a_u8[2] = *source.get(number).unwrap();
				a_u8[3] = *source.get(number).unwrap();
				//a_u8[0] = *source.iter().nth(number).unwrap();
				//a_u8[1] = *source.iter().nth(number + 1).unwrap();
				//a_u8[2] = *source.iter().nth(number + 2).unwrap();
				//a_u8[3] = *source.iter().nth(number + 3).unwrap();
				//println!("{}",u32::from_le_bytes(a_u8));
				return_u32.push(u32::from_le_bytes(a_u8));
				number += 1;
			}
		}

		if return_u32.is_empty() {
			return Err("Here is some error.");
		};

		Ok(return_u32)
	}

 let k=	u32::from_le_bytes([0x78, 0x56, 0x34, 0x12]);
				return_u32

				struct DayStruct
{
    date:String,
    open:u32,
    high:u32,
    low:u32,
    close:u32,
    amount:u32,
    vol:u32,
    reservation:u32
}
StockData;
CREATE TABLE day_dealing(

number: INTEGER,
	number:INTEGER,
	date:INTEGER,
	open:INTEGER,
	high:INTEGER,
	low:INTEGER,
	close:INTEGER,
	amount:INTEGER,
	vol:INTEGER,
	reservation:INTEGER,
	constraint keyname primary key (number,date),

)
*/
