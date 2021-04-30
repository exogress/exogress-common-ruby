#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rutie;

use rutie::{Class, Object};

pub mod client;
pub mod project;
pub mod param;
pub mod schema;
pub mod jwt;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_exogress_common() {
    let data_class = Class::from_existing("Object");

    Class::new("ClientConfig", Some(&data_class)).define(|itself| {
        itself.def_self("new", client::new);

        itself.def("revision", client::config_revision);
        itself.def("checksum", client::config_checksum);
        itself.def("mount_points_names", client::mount_points_names);
        itself.def("name", client::config_name);

        itself.def("to_s", client::to_s);
    });

    Class::new("ProjectConfig", Some(&data_class)).define(|itself| {
        itself.def_self("new", project::new);
        itself.def_self("default", project::default);

        itself.def("checksum", project::config_checksum);
        itself.def("mount_points_names", project::mount_points_names);

        itself.def("to_s", project::to_s);
    });

    Class::new("Parameter", Some(&data_class)).define(|itself| {
        itself.def_self("new", param::new);
        itself.def_self("all_schemas", param::all_schemas);
        itself.def_self("sample", param::sample);

        itself.def("to_yaml", param::to_yaml);
        itself.def("to_json", param::to_json);
    });

    Class::new("Schema", Some(&data_class)).define(|itself| {
        itself.def_self("get", schema::get);
    });

    Class::new("Jwt", Some(&data_class)).define(|itself| {
        itself.def_self("generate_access_key_pair", jwt::generate_access_key_pair);
        itself.def_self("validate_jwt_token", jwt::validate_jwt_token);
    });
}
