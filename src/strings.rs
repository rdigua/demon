///Strings
pub fn string_vec(source:String,split_string:&str)->Option<Vec<String>>{
	let v:Vec<String>=source.split(split_string).map(|s| s.to_string()).collect();
	Some(v)
}