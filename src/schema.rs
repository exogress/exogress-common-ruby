use rutie::{RString, VM, AnyObject, NilClass};
use exogress_common::config_core::get_schema;

class!(Schema);

methods!(
    Schema,
    _rtself,

    fn get(kind_rstr: RString, schema_name_rstr: RString) -> AnyObject {
        let schema_name = schema_name_rstr.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let kind = kind_rstr.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        match get_schema(&kind, &schema_name) {
            Some(s) => {
                RString::from(s.to_string()).into()
            }
            None => {
                NilClass::new().into()
            }
        }
    }
);

