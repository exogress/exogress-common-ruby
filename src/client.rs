use std::iter::FromIterator;
use rutie::{AnyException, Class, Exception, Object, RString, VM, Integer, Array};
use exogress_common::config_core::Config;

wrappable_struct!(exogress_common::config_core::ClientConfig, ClientConfigWrapper, CLIENT_CONFIG_WRAPPER);

class!(ClientConfig);

methods!(
    ClientConfig,
    itself,

    fn new(input: RString) -> Class {
        let ruby_string = input.map_err(|e| VM::raise_ex(e)).unwrap();

        match exogress_common::config_core::ClientConfig::parse(ruby_string.to_string().as_str())
        {
            Ok(config) => {
                Class::from_existing("ClientConfig").wrap_data(config, &*CLIENT_CONFIG_WRAPPER)
            },
            Err(e) => {
                let exception = AnyException::new("ClientConfigException", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
                unreachable!()
            }
        }
    }

    fn config_revision() -> Integer {
        itself.get_data(&*CLIENT_CONFIG_WRAPPER).revision.0.into()
    }

    fn mount_points_names() -> Array {
        Array::from_iter(itself
            .get_data(&*CLIENT_CONFIG_WRAPPER)
            .mount_points
            .keys()
            .map(|mount_point_name| RString::new_utf8(mount_point_name.as_ref()).try_convert_to().unwrap()))
    }

    fn config_checksum() -> Integer {
        itself.get_data(&*CLIENT_CONFIG_WRAPPER).checksum().into()
    }

    fn config_name() -> RString {
        RString::new_utf8(itself.get_data(&*CLIENT_CONFIG_WRAPPER).name.as_str())
    }

    fn to_s() -> RString {
        let data = itself.get_data(&*CLIENT_CONFIG_WRAPPER);
        RString::new_utf8(serde_yaml::to_string(&data).unwrap().as_str())
    }
);

