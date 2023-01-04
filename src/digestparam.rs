use serde_json;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};

use std::error::Error;
use lazy_static::lazy_static;

extargs_error_class!{DigestParamError}

lazy_static ! {
	static ref CIPHER_NAMES : Vec<String> = {
		let mut retv :Vec<String> = Vec::new();
		retv.push(format!("des3"));
		retv.push(format!("des-cbc"));
		retv.push(format!("aes-256-cbc"));
		retv
	};

	static ref DIGEST_NAMES :Vec<String> = {
		let mut retv :Vec<String> = Vec::new();
		retv.push(format!("sha256"));
		retv.push(format!("md5"));
		retv
	};
}

pub fn add_digest_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});
	let mut curs :String;

	if !val2.is_object() {
		extargs_new_error!{DigestParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{DigestParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{DigestParamError,"[{}] not valid object",k}
		}
		val = val2.clone();
		for ck in DIGEST_NAMES.iter() {
			curs = format!("{}##{} digest algorithm##", ck,ck);
			val[k][&curs] = serde_json::json!(false);
		}

		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}




pub fn get_cipher_name(ns :NameSpaceEx) -> Result<String,Box<dyn Error>> {
	let retv :String;
	let mut curs :String;
	for k in CIPHER_NAMES.iter() {
		curs = format!("{}",k);
		if ns.get_bool(&curs) {
			retv=format!("{}",k);
			return Ok(retv);
		}
	}
	extargs_new_error!{DigestParamError,"not specified cipher name"}
}


pub fn add_cipher_param(ins :&str) -> Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});
	let mut curs :String;

	if !val2.is_object() {
		extargs_new_error!{DigestParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{DigestParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{DigestParamError,"[{}] not valid object",k}
		}
		val = val2.clone();

		for ck in CIPHER_NAMES.iter() {
			curs = format!("{}##{} algorithm##",ck,ck);
			val[k][&curs] = serde_json::json!(false);
		}

		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}
