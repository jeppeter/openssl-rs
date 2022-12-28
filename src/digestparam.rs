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


		val[k]["in##Input file##"] = serde_json::json!(null);
		val[k]["out##Output file##"] = serde_json::json!(null);
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


		val[k]["in##Input file##"] = serde_json::json!(null);
		val[k]["out##Output file##"] = serde_json::json!(null);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}
