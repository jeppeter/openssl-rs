use serde_json;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::error::Error;

extargs_error_class!{PkeyoptParamError}

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


		val[k]["pkeyopt##Public key options as opt:value##"] = serde_json::json!(null);
		idx += 1;
	}

	let rets :String = serde_json::to_string_pretty(&val)?;
	Ok(rets)
}
