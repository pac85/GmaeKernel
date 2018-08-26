use toml::macros::Deserialize;
use toml::value::Table;

use toml::*;


pub trait ConfigModule{
    fn get_name() -> &'static str;
}

pub struct Config {
    t : Table
}

impl Config {
    fn new<T : AsRef<str>>(x : T) -> Self {
        if let Value::Table(t) = x.as_ref().parse().unwrap() {
            return Config{t}
        } else {
            panic!("...");
        }
    }

    fn conf_for_module<'a,T : Deserialize<'a> + ConfigModule>(&mut self) -> T {
        self.t.remove(T::get_name()).unwrap().try_into().unwrap()
    }
}
