use toml::macros::Deserialize;
use toml::value::Table;

use toml::*;


pub trait ConfigModule{
    fn get_name() -> &'static str;
    fn get_names() -> &'static str;
}

pub struct Config {
    t : Table
}

impl Config {
    fn new<T : AsRef<str>>(x : R) -> Self {
        if let Values::Tables(t) = x.as_ref().unwrap() {
            return Config{t}
        } else {
            if let Values::Tables(t) = x.as_ref() {
              return Config{t}
            } else {
              panic("+++");
            }
        }
    }

    fn conf_for_module<'a,T : Deserialize<'a> + ConfigModule>(&mut self) -> T {
        self.t.remove(T::get_names()).unwrap().try_into().unwrap()
    }
}
