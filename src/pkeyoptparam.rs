use serde_json;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};
use std::error::Error;
use ssllib::config::*;

use lazy_static::lazy_static;


extargs_error_class!{PkeyoptParamError}

lazy_static !{
	static ref RSA_OPT_NAMES :Vec<String> = {
		let mut retv :Vec<String> = Vec::new();
		retv.push(format!("rsa_keygen_bits"));
		retv
	};
}

pub fn add_pkeyopt_param(ins :&str) ->  Result<String,Box<dyn Error>> {
	let  val2 :serde_json::value::Value = serde_json::from_str(ins)?;
	let mut idx :usize = 0;
	let mut val :serde_json::value::Value = serde_json::json!({});

	if !val2.is_object() {
		extargs_new_error!{PkeyoptParamError,"not valid json\n{}",ins}
	}

	for (k,_) in val2.as_object().unwrap().iter() {
		if idx > 0 {
			extargs_new_error!{PkeyoptParamError,"more than one parameters"}
		}

		if !val2[k].is_object() {
			extargs_new_error!{PkeyoptParamError,"[{}] not valid object",k}
		}
		val = val2.clone();


		val[k]["pkeyopt##Public key options as opt:value##"] = serde_json::json!([]);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}

pub fn get_pkeyopt_param(ns :NameSpaceEx) -> Result<ConfigValue,Box<dyn Error>> {
	let mut retv :ConfigValue = ConfigValue::new("{}").unwrap();

	let sarr = ns.get_array("pkeyopt");
	for k in sarr.iter() {
		let vs :Vec<&str> = k.split(":").collect();
		let mut bmatched :bool = false;
		if vs.len() <= 1 {
			extargs_new_error!{PkeyoptParamError,"[{}] not valid opt format",k}
		}


		for ck in RSA_OPT_NAMES.iter() {
			if ck.eq(vs[0]) {
				let _ = retv.set_str(ck,vs[1])?;
				bmatched = true;
				break;
			}
		}

		if !bmatched {
			extargs_new_error!{PkeyoptParamError,"not suppport [{}] opt", k}
		}

	}
	Ok(retv)

}