use std::iter::FromIterator;
use rutie::{AnyException, Class, Exception, Object, RString, VM, Integer, Array};
use exogress_common::config_core::Config;

wrappable_struct!(exogress_common::config_core::ProjectConfig, ProjectConfigWrapper, PROJECT_CONFIG_WRAPPER);

class!(ProjectConfig);

methods!(
    ProjectConfig,
    itself,

    fn new(body: RString, schema: RString) -> Class {
        let ruby_string = body.map_err(|e| VM::raise_ex(e)).unwrap();

        match exogress_common::config_core::ProjectConfig::parse(ruby_string.to_string().as_str())
        {
            Ok(config) => {
                Class::from_existing("ProjectConfig").wrap_data(config, &*PROJECT_CONFIG_WRAPPER)
            },
            Err(e) => {
                let exception = AnyException::new("ProjectConfigException", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
                unreachable!()
            }
        }
    }

    fn mount_points_names() -> Array {
        Array::from_iter(itself
            .get_data(&*PROJECT_CONFIG_WRAPPER)
            .mount_points
            .keys()
            .map(|mount_point_name| RString::new_utf8(mount_point_name.as_ref()).try_convert_to().unwrap()))
    }

    fn config_checksum() -> Integer {
        itself.get_data(&*PROJECT_CONFIG_WRAPPER).checksum().into()
    }

    fn to_s() -> RString {
        let data = itself.get_data(&*PROJECT_CONFIG_WRAPPER);
        RString::new_utf8(serde_yaml::to_string(&data).unwrap().as_str())
    }

    fn default(mount_point: RString) -> Class {
        let mount_point = mount_point.map_err(|e| VM::raise_ex(e)).unwrap().to_string().parse().unwrap();

        Class::from_existing("ProjectConfig")
            .wrap_data(exogress_common::config_core::ProjectConfig::default_with_mount_point(&mount_point), &*PROJECT_CONFIG_WRAPPER)
    }
);
