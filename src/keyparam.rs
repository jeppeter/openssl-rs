use serde_json;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::error::Error;

extargs_error_class!{KeyParamError}

pub fn add_pubin_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});

	if !val2.is_object() {
		extargs_new_error!{KeyParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{KeyParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{KeyParamError,"[{}] not valid object",k}
		}
		val = val2.clone();


		val[k]["pubin##Expect a public key in input file##"] = serde_json::json!(null);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}

pub fn add_pubout_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});

	if !val2.is_object() {
		extargs_new_error!{KeyParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{KeyParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{KeyParamError,"[{}] not valid object",k}
		}
		val = val2.clone();


		val[k]["pubout##Output a public key##"] = serde_json::json!(null);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}

pub fn add_modulus_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});

	if !val2.is_object() {
		extargs_new_error!{KeyParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{KeyParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{KeyParamError,"[{}] not valid object",k}
		}
		val = val2.clone();


		val[k]["modulus##Print modulus##"] = serde_json::json!(false);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}