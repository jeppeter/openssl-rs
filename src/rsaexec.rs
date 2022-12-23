#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};

#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;


#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};

use super::ioparam::*;
use super::passparam::*;
use super::keyparam::*;


fn rsa_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let sarr :Vec<String>;
	init_log(ns.clone())?;
	sarr = ns.get_array("subnargs");
	debug_trace!("sarr {:?}",sarr);
	Ok(())
}


#[extargs_map_function(rsa_handler)]
pub fn load_rsa_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"rsa<rsa_handler>##to gen rsa functions##" : {
			"$" : 0,
			"RSAPublicKey_in##Input is an RSAPublicKey##" : false,
			"RSAPublicKey_out##Output is an RSAPublicKey##" : false
		}
	}
	"#;
	let c = add_in_out_param(cmdline)?;
	let d = add_in_out_form_param(&c)?;
	let e = add_passin_param(&d)?;
	let f = add_passout_param(&e)?;
	let g = add_pubin_param(&f)?;
	let h = add_pubout_param(&g)?;
	let i = add_noout_param(&h)?;
	let j = add_text_param(&i)?;
	let k = add_modulus_param(&j)?;
	/**/
	extargs_load_commandline!(parser,&k)?;
	Ok(())
}
