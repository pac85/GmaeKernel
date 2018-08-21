extern crate toml;
#[macro_use]
extern crate serde_derive;
use toml::macros::Deserialize;
use toml::value::Table;

use toml::*;


trait ConfigModule{
    fn get_name() -> &'static str;
}