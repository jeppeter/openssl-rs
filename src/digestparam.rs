use serde_json;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::error::Error;

extargs_error_class!{IoParamError}

pub fn add_digest_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});

	if !val2.is_object() {
		extargs_new_error!{IoParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{IoParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{IoParamError,"[{}] not valid object",k}
		}
		val = val2.clone();

		val[k]["sha256-cbc##sha256 cbc handle##"] = serde_json::json!(false);
		val[k]["md5##md5##"] = serde_json::json!(false);

		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}


pub fn add_cipher_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});

	if !val2.is_object() {
		extargs_new_error!{IoParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{IoParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{IoParamError,"[{}] not valid object",k}
		}
		val = val2.clone();


		val[k]["des-cbc##des cbc handle##"] = serde_json::json!(false);
		val[k]["des3##des3##"] = serde_json::json!(false);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}
