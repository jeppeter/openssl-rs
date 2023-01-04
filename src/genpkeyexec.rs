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
use ssllib::config::*;


#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};

use super::ioparam::*;
use super::passparam::*;
use super::digestparam::*;
use super::pkeyoptparam::*;

fn genpkey_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	
	let types :String;
	init_log(ns.clone())?;
	types = get_cipher_name(ns.clone())?;
	let cfg :ConfigValue = get_pkeyopt_param(ns.clone())?;

	Ok(())
}


#[extargs_map_function(genpkey_handler)]
pub fn load_genpkey_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"genpkey<genpkey_handler>##to gen rsa functions##" : {
			"$" : 0,
			"paramfile##Parameters file##" : null,
			"algorithm##The public key algorithm##" : null,
			"genparam##Generate parameters, not key##" : null
		}
	}
	"#;
	let c = add_out_param(cmdline)?;
	let d = add_out_form_param(&c)?;
	let e = add_pass_param(&d)?;
	let f = add_pkeyopt_param(&e)?;
	let g = add_text_param(&f)?;
	let h = add_cipher_param(&g)?;
	/**/
	extargs_load_commandline!(parser,&h)?;
	Ok(())
}