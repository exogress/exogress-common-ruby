use rutie::{AnyException, Class, Exception, RString, VM, Object, Array};
use exogress_common::config_core::referenced::{ParameterSchema, ALL_PARAMETER_SCHEMAS};
use std::convert::TryInto;
use std::iter::FromIterator;
use std::str::FromStr;

wrappable_struct!(exogress_common::config_core::referenced::Parameter, ParameterWrapper, PARAMETER_WRAPPER);

class!(Parameter);

methods!(
    Parameter,
    itself,

    fn new(body_input: RString, schema_input: RString) -> Class {
        let body = body_input.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let schema = schema_input.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let parameter_schema: ParameterSchema = match schema.parse() {
            Ok(parameter_schema) => {
                parameter_schema
            },
            Err(_e) => {
                let exception = AnyException::new("ParameterException", Some(format!("Bad schema: {}", schema).as_str()));
                VM::raise_ex(exception);
                unreachable!()
            }
        };

        match (parameter_schema, body).try_into()
        {
            Ok(config) => {
                Class::from_existing("Parameter").wrap_data(config, &*PARAMETER_WRAPPER)
            },
            Err(e) => {
                let exception = AnyException::new("ParameterException", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
                unreachable!()
            }
        }
    }

    fn sample(schema: RString) -> RString {
        let schema = schema.unwrap().to_string();
        let schema = ParameterSchema::from_str(schema.as_str()).map_err(|()| {
            let exception = AnyException::new("ParameterException", Some(format!("Bad schema: {}", schema).as_str()));
            VM::raise_ex(exception)
        }).unwrap();
        RString::new_utf8(schema.sample().as_str())

    }

    fn to_json() -> RString {
        let data = itself.get_data(&*PARAMETER_WRAPPER);
        RString::new_utf8(data.to_inner_json().as_str())
    }

    fn to_yaml() -> RString {
        let data = itself.get_data(&*PARAMETER_WRAPPER);
        RString::new_utf8(data.to_inner_yaml().as_str())
    }

    fn all_schemas() -> Array {
        let schema_strigs = ALL_PARAMETER_SCHEMAS.iter().map(|schema| {
            RString::from(schema.to_string()).to_any_object()
        });

        Array::from_iter(schema_strigs)
    }


);
