use std::collections::HashMap;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct Post {
	foo: String,
}

#[derive(Debug,Serialize,Deserialize)]
struct JsonRes{
	data : String,
	headers : HashMap<String,String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    	println!("Basic Post Request using reqwest,tokio,serde !");

	let mut post_map : HashMap<&str,&str> = HashMap::new();
	post_map.insert("foo","bar");	
	println!("Post Request : {:#?}",post_map);

	let res = reqwest::Client::new()
		.post("https://httpbin.org/post")
		.json(&post_map)
		.send()
		.await?;
	println!("Res Response : {:#?}",res);

	let js = res
		.json::<JsonRes>()
		.await?;

	println!("JsonRes Headers : {:#?} ",js.headers);
	let post_data : Post = serde_json::from_str(&js.data)?;
	println!("Response data passed back in to struct using serde_json {:#?}",post_data);

	Ok(())
}
