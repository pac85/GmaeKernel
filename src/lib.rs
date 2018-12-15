extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate evmap;
extern crate parking_lot;
extern crate game_kernel_utils;

pub mod game_kernel
{
    pub mod logger;
    pub mod core_systems;
    pub mod ecs;
    pub mod settings;
}

/*#[cfg(test)]
mod tests {
    use ::game_kernel::logger;
    #[test]
    fn it_works() {
        logger::log_msg("this is a message");
        logger::log_warn("this is a message");
        logger::log_err("this is a message");
    }
}*/

