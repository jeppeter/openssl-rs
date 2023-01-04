#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::options::{ExtArgsOptions};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};
#[allow(unused_imports)]
use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use extargsparse_worker::options::*;

#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[cfg(windows)]
mod wchar_windows;
#[cfg(windows)]
mod loglib_windows;
mod loglib;
mod ioparam;
mod passparam;
mod keyparam;
mod digestparam;
mod pkeyoptparam;
mod rsaexec;
mod genpkeyexec;


#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let optstr :String = format!(r#"{{ "{}" : "-",
        "{}" : "-",
        "{}" : true,
        "{}" : false ,
        "{}" : true }}"#,
        OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_JSON_OPTION,OPT_CMD_PREFIX_ADDED,
        OPT_FLAG_NO_CHANGE);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr)?;
	let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None)?;
	loglib::prepare_log(parser.clone())?;
	rsaexec::load_rsa_handler(parser.clone())?;
	genpkeyexec::load_genpkey_handler(parser.clone())?;
	let ores = parser.parse_commandline_ex(None,None,None,None);
	if ores.is_err() {
		let e = ores.err().unwrap();
		eprintln!("{:?}", e);
		return Err(e);
	}
	return Ok(());
}